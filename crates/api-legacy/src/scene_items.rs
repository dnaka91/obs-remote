use obs::{
    frontend, graphics,
    scene::{Scene, SceneItem},
    source::Source,
};
use tonic::{Request, Response, Status};

use self::scene_items_server::SceneItems;
use super::common::SceneItemTransform;
use crate::precondition;

tonic::include_proto!("obs_remote.legacy.scene_items");

pub struct Service;

#[tonic::async_trait]
impl SceneItems for Service {
    async fn list(&self, request: Request<ListRequest>) -> Result<Response<ListReply>, Status> {
        use self::list_reply::SceneItem;

        let scene_name = request.into_inner().scene_name;
        let items = scene_or_current(&scene_name)?
            .list_items()
            .iter()
            .map(|item| {
                let source = item.source();
                let mut scene_item = SceneItem {
                    id: item.id(),
                    kind: source.id(),
                    name: source.name(),
                    ..Default::default()
                };
                scene_item.set_ty(source.ty().into());
                scene_item
            })
            .collect();

        Ok(Response::new(ListReply { scene_name, items }))
    }

    async fn get_properties(
        &self,
        request: Request<GetPropertiesRequest>,
    ) -> Result<Response<GetPropertiesReply>, Status> {
        let GetPropertiesRequest { scene_name, item } = request.into_inner();

        let scene = scene_or_current(&scene_name)?;
        let mut items = scene.list_items();
        let item = find_scene_item(&mut items, item.as_ref())?;
        let name = item.source().name();

        Ok(Response::new(GetPropertiesReply {
            name,
            id: item.id(),
            properties: Some(scene_item_data(item)),
        }))
    }

    async fn set_properties(
        &self,
        request: Request<SetPropertiesRequest>,
    ) -> Result<Response<()>, Status> {
        let properties = request.into_inner();

        let scene = scene_or_current(&properties.scene_name)?;
        let mut items = scene.list_items();
        let item = find_scene_item(&mut items, properties.item.as_ref())?;

        item.update(|item| {
            if let Some(position) = properties.position {
                item.set_pos((position.x, position.y));
                if let Some(alignment) = position.alignment {
                    item.set_alignment(alignment.into());
                }
            }

            if let Some(rotation) = properties.rotation {
                item.set_rot(rotation);
            }

            if let Some(scale) = properties.scale {
                item.set_scale((scale.x, scale.y));
                if let Some(filter) = scale.filter().into() {
                    item.set_scale_filter(filter);
                }
            }

            if let Some(crop) = properties.crop {
                item.set_crop((
                    crop.left as i32,
                    crop.top as i32,
                    crop.right as i32,
                    crop.bottom as i32,
                ));
            }

            if let Some(visible) = properties.visible {
                item.set_visible(visible);
            }

            if let Some(locked) = properties.locked {
                item.set_locked(locked);
            }

            if let Some(bounds) = properties.bounds {
                item.set_bounds((bounds.x, bounds.y));
                if let Some(ty) = bounds.ty().into() {
                    item.set_bounds_type(ty);
                }
                if let Some(alignment) = bounds.alignment {
                    item.set_bounds_alignment(alignment.into());
                }
            }
        });

        Ok(Response::new(()))
    }

    async fn reset(&self, request: Request<ResetRequest>) -> Result<Response<()>, Status> {
        let ResetRequest { scene_name, item } = request.into_inner();

        let scene = scene_or_current(&scene_name)?;
        let mut items = scene.list_items();
        let item = find_scene_item(&mut items, item.as_ref())?;
        let item = item.source();

        item.update(item.settings());

        Ok(Response::new(()))
    }

    async fn delete(&self, request: Request<DeleteRequest>) -> Result<Response<()>, Status> {
        let DeleteRequest { scene_name, item } = request.into_inner();

        let scene = scene_or_current(&scene_name)?;
        let mut items = scene.list_items();
        let item = find_scene_item(&mut items, item.as_ref())?;

        item.remove();

        Ok(Response::new(()))
    }

    async fn add(&self, request: Request<AddRequest>) -> Result<Response<AddReply>, Status> {
        let AddRequest {
            scene_name,
            source_name,
            set_visible,
        } = request.into_inner();

        let mut scene = scene_or_current(&scene_name)?;
        let source = Source::by_name(&source_name).ok_or_else(|| {
            Status::failed_precondition(format!("`{}` doesn't exist", source_name))
        })?;

        precondition!(scene.source() != source, "can't add scene to itself");

        let id = graphics::scoped(|| {
            scene.atomic_update(|scene| {
                let item = scene.add(&source);
                if let Some(visible) = set_visible {
                    item.set_visible(visible);
                }

                item.id()
            })
        });

        Ok(Response::new(AddReply { id }))
    }

    async fn duplicate(
        &self,
        request: Request<DuplicateRequest>,
    ) -> Result<Response<DuplicateReply>, Status> {
        let request = request.into_inner();

        let from_scene = find_scene(&request.from_scene)?;
        let mut to_scene = find_scene(&request.to_scene)?;

        let mut items = from_scene.list_items();
        let ref_item = find_scene_item(&mut items, request.item.as_ref())?;

        let (name, id) = graphics::scoped(|| {
            to_scene.atomic_update(|scene| {
                let item = scene.add(&ref_item.source());
                item.set_visible(ref_item.visible());
                let name = item.source().name();

                (name, item.id())
            })
        });

        Ok(Response::new(DuplicateReply {
            scene: request.to_scene,
            specification: Some(SceneItemSpecification { name, id }),
        }))
    }
}

fn find_scene(scene_name: &str) -> Result<Scene<'static>, Status> {
    precondition!(!scene_name.is_empty(), "scene name mustn't be empty");

    let source = Source::by_name(scene_name)
        .ok_or_else(|| Status::failed_precondition(format!("`{}` doesn't exist", scene_name)))?;

    Scene::from_source(source)
        .ok_or_else(|| Status::failed_precondition("requested source is not a scene"))
}

fn scene_or_current(scene_name: &str) -> Result<Scene<'static>, Status> {
    let source = if scene_name.is_empty() {
        frontend::scenes::current()
    } else {
        Source::by_name(scene_name)
            .ok_or_else(|| Status::failed_precondition(format!("`{}` doesn't exist", scene_name)))?
    };

    Scene::from_source(source)
        .ok_or_else(|| Status::failed_precondition("requested source is not a scene"))
}

fn find_scene_item<'a, 'b>(
    list: &'a mut [SceneItem<'b>],
    item: Option<&Identifier>,
) -> Result<&'a mut SceneItem<'b>, Status> {
    use self::identifier::Value;

    let identifier = item
        .ok_or_else(|| Status::failed_precondition("item identifier must be specified"))?
        .value
        .as_ref()
        .ok_or_else(|| Status::failed_precondition("identifier value must be specified"))?;

    match identifier {
        Value::Name(name) => list.iter_mut().find(|item| &item.source().name() == name),
        Value::Specification(SceneItemSpecification { name, id }) => list
            .iter_mut()
            .find(|item| item.id() == *id && &item.source().name() == name),
    }
    .ok_or_else(|| Status::failed_precondition("scene item doesn't exist"))
}

fn scene_item_data(item: &SceneItem<'_>) -> SceneItemTransform {
    let source = item.source();
    let scale = item.scale();

    let parent_group_name = item
        .parent_scene()
        .filter(|scene| scene.source().id() == "group")
        .map(|scene| scene.source().name())
        .unwrap_or_default();

    let group_children = item
        .list_group_items()
        .map(|children| children.iter().map(scene_item_data).collect())
        .unwrap_or_default();

    SceneItemTransform {
        position: Some((item.pos(), item.alignment()).into()),
        rotation: item.rot(),
        scale: Some((scale, item.scale_filter()).into()),
        crop: Some(item.crop().into()),
        visible: item.visible(),
        locked: item.locked(),
        bounds: Some((item.bounds(), item.bounds_type(), item.bounds_alignment()).into()),
        source_width: source.width(),
        source_height: source.height(),
        width: source.width() as f32 * scale.0,
        height: source.height() as f32 * scale.1,
        parent_group_name,
        group_children,
    }
}
