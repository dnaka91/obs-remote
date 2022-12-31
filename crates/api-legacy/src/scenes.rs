use obs::{
    frontend,
    scene::{Scene, SceneItem},
    source::{Source, SourceType},
    Duration,
};
use tonic::{Request, Response, Status};

use self::scenes_server::Scenes;
use super::common::DurationExt;
use crate::precondition;

tonic::include_proto!("obs_remote.legacy.scenes");

pub struct Service;

#[tonic::async_trait]
impl Scenes for Service {
    async fn set_current(
        &self,
        request: Request<SetCurrentRequest>,
    ) -> Result<Response<()>, Status> {
        let name = request.into_inner().name;
        precondition!(!name.is_empty(), "name mustn't be empty");

        let source = Source::by_name(&name)
            .ok_or_else(|| Status::failed_precondition(format!("`{name}` doesn't exist")))?;

        frontend::scenes::set_current(&source);

        Ok(Response::new(()))
    }

    async fn get_current(&self, request: Request<()>) -> Result<Response<GetCurrentReply>, Status> {
        let source = frontend::scenes::current();
        let name = source.name();
        let scene = Scene::from_source(source)
            .ok_or_else(|| Status::internal("current scene appears to be invalid"))?;
        let sources = scene.list_items().iter().map(source_data).collect();

        Ok(Response::new(GetCurrentReply { name, sources }))
    }

    async fn list(&self, request: Request<()>) -> Result<Response<ListReply>, Status> {
        let current = frontend::scenes::current().name();
        let scenes = frontend::scenes::list()
            .into_iter()
            .map(|source| {
                let name = source.name();
                let scene = Scene::from_source(source)
                    .ok_or_else(|| Status::internal("scene appears to be invalid"))?;
                let sources = scene.list_items().iter().map(source_data).collect();

                Ok(self::list_reply::Scene { name, sources })
            })
            .collect::<Result<_, Status>>()?;

        Ok(Response::new(ListReply { current, scenes }))
    }

    async fn create(&self, request: Request<CreateRequest>) -> Result<Response<()>, Status> {
        let name = request.into_inner().name;
        precondition!(Source::by_name(&name).is_none(), "scene already exists");

        Scene::create(&name);

        Ok(Response::new(()))
    }

    async fn reorder(&self, request: Request<ReorderRequest>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn set_transition_override(
        &self,
        request: Request<SetTransitionOverrideRequest>,
    ) -> Result<Response<()>, Status> {
        let SetTransitionOverrideRequest {
            scene_name,
            transition_name,
            transition_duration,
        } = request.into_inner();
        precondition!(!scene_name.is_empty(), "scene name mustn't be empty");
        precondition!(
            !transition_name.is_empty(),
            "transition name mustn't be empty"
        );

        let scene = Source::by_name(&scene_name)
            .ok_or_else(|| Status::failed_precondition(format!("`{scene_name}` doesn't exist")))?;
        precondition!(scene.ty() == SourceType::Scene, "scene is invalid");

        let transition = frontend::transitions::list()
            .into_iter()
            .find(|t| t.name() == transition_name)
            .ok_or_else(|| {
                Status::failed_precondition(format!("`{transition_name}` doesn't exist"))
            })?;

        let mut data = scene.private_settings();
        data.set_string("transition", &transition.name());
        data.set_int(
            "transition_duration",
            transition_duration
                .map_or_else(frontend::transitions::duration, Duration::from_proto)
                .whole_milliseconds() as i64,
        );

        Ok(Response::new(()))
    }

    async fn remove_transition_override(
        &self,
        request: Request<RemoveTransitionOverrideRequest>,
    ) -> Result<Response<()>, Status> {
        let name = request.into_inner().name;
        precondition!(!name.is_empty(), "name mustn't be empty");

        let scene = Source::by_name(&name)
            .ok_or_else(|| Status::failed_precondition(format!("`{name}` doesn't exist")))?;
        precondition!(scene.ty() == SourceType::Scene, "scene is invalid");

        let mut data = scene.private_settings();
        data.erase("transition");
        data.erase("transition_duration");

        Ok(Response::new(()))
    }

    async fn get_transition_override(
        &self,
        request: Request<GetTransitionOverrideRequest>,
    ) -> Result<Response<GetTransitionOverrideReply>, Status> {
        let name = request.into_inner().name;
        precondition!(!name.is_empty(), "name mustn't be empty");

        let scene = Source::by_name(&name)
            .ok_or_else(|| Status::failed_precondition(format!("`{name}` doesn't exist")))?;
        precondition!(scene.ty() == SourceType::Scene, "scene is invalid");

        let data = scene.private_settings();
        data.string("transition");

        Ok(Response::new(GetTransitionOverrideReply {
            name: data.string("transition").unwrap_or_default(),
            duration: data
                .int("transition_duration")
                .map(Duration::milliseconds)
                .map(DurationExt::into_proto),
        }))
    }
}

// TODO: This is almost the same as for the scene items API.
// TODO: Probably good to unify.
fn source_data(item: &SceneItem<'_>) -> super::common::SceneItem {
    let source = item.source();
    let scale = item.scale();
    let (x, y) = item.pos();

    let parent_group_name = item
        .parent_scene()
        .filter(|scene| scene.source().id() == "group")
        .map(|scene| scene.source().name());

    let group_children = item
        .list_group_items()
        .map(|children| children.iter().map(source_data).collect())
        .unwrap_or_default();

    let mut item = super::common::SceneItem {
        cx: source.width() as f32 * scale.0,
        cy: source.height() as f32 * scale.1,
        alignment: Some(item.alignment().into()),
        name: source.name(),
        id: item.id(),
        render: item.visible(),
        muted: source.muted(),
        locked: item.locked(),
        source_cx: source.width(),
        source_cy: source.height(),
        ty: Default::default(),
        volume: source.volume().as_mul(),
        x,
        y,
        parent_group_name,
        group_children,
    };
    item.set_ty(source.ty().into());
    item
}
