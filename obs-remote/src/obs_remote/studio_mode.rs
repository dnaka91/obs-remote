use obs::{
    frontend::{preview_mode, transitions},
    scene::{self, Scene},
    source::Source,
};
use tonic::{Request, Response, Status};

use self::studio_mode_server::StudioMode;
use super::common::{SceneItem, SourceType};
use crate::precondition;

tonic::include_proto!("obs_remote.studio_mode");

pub struct Service;

#[tonic::async_trait]
impl StudioMode for Service {
    async fn status(&self, request: Request<()>) -> Result<Response<StatusReply>, Status> {
        Ok(Response::new(StatusReply {
            active: preview_mode::active(),
        }))
    }

    #[allow(clippy::cast_possible_wrap, clippy::cast_precision_loss)]
    async fn get_preview_scene(
        &self,
        request: Request<()>,
    ) -> Result<Response<GetPreviewSceneReply>, Status> {
        fn get_data(item: &scene::SceneItem) -> SceneItem {
            let pos = item.pos();
            let scale = item.scale();
            let item_source = item.source();
            let parent = item.parent_scene().and_then(|p| {
                let source = p.source();
                (source.id() == "group").then(|| source.name())
            });

            SceneItem {
                name: item_source.name(),
                id: item.id(),
                ty: SourceType::from(item_source.ty()) as i32,
                volume: item_source.volume(),
                x: pos.0,
                y: pos.1,
                source_cx: item_source.width(),
                source_cy: item_source.height(),
                muted: item_source.muted(),
                alignment: Some(item.alignment().into()),
                cx: item_source.width() as f32 * scale.0,
                cy: item_source.height() as f32 * scale.1,
                render: item.visible(),
                locked: item.locked(),
                parent_group_name: parent,
                group_children: item
                    .list_group_items()
                    .map(|items| items.iter().map(get_data).collect())
                    .unwrap_or_default(),
            }
        }

        precondition!(preview_mode::active(), "studio mode isn't active");

        let source = preview_mode::current_scene()
            .ok_or_else(|| Status::internal("no current preview scene"))?;
        let scene = Scene::from_source(&source)
            .ok_or_else(|| Status::internal("current source is not a scene"))?;

        Ok(Response::new(GetPreviewSceneReply {
            name: source.name(),
            sources: scene.list_items().iter().map(get_data).collect(),
        }))
    }

    async fn set_preview_scene(
        &self,
        request: Request<SetPreviewSceneRequest>,
    ) -> Result<Response<()>, Status> {
        precondition!(preview_mode::active(), "studio mode isn't active");

        let scene_name = request.into_inner().scene_name;
        let scene = if scene_name.is_empty() {
            obs::frontend::current_scene()
        } else {
            Source::by_name(&scene_name).ok_or_else(|| {
                Status::failed_precondition(format!("scene `{}` doesn't exist", scene_name))
            })?
        };

        preview_mode::set_current_scene(&scene);

        Ok(Response::new(()))
    }

    #[allow(clippy::cast_possible_wrap, clippy::cast_precision_loss)]
    async fn transition_to_program(
        &self,
        request: Request<TransitionToProgramRequest>,
    ) -> Result<Response<()>, Status> {
        precondition!(preview_mode::active(), "studio mode isn't active");

        if let Some(info) = request.into_inner().with_transition {
            if let Some(name) = info.name {
                precondition!(!name.is_empty(), "transition name must be set");

                let transition = transitions::list()
                    .into_iter()
                    .find(|source| source.name() == name)
                    .ok_or_else(|| {
                        Status::failed_precondition(format!("transition `{}` doesn't exist", name))
                    })?;

                transitions::set_current(&transition);
            }

            if let Some(duration) = info.duration {
                transitions::set_duration(duration as i32);
            }
        }

        preview_mode::trigger_transition();
        Ok(Response::new(()))
    }

    async fn enable(&self, request: Request<()>) -> Result<Response<()>, Status> {
        precondition!(!preview_mode::active(), "studio mode is already active");

        preview_mode::set(true);
        Ok(Response::new(()))
    }

    async fn disable(&self, request: Request<()>) -> Result<Response<()>, Status> {
        precondition!(preview_mode::active(), "studio mode isn't active");

        preview_mode::set(false);
        Ok(Response::new(()))
    }

    async fn toggle(&self, request: Request<()>) -> Result<Response<()>, Status> {
        preview_mode::set(!preview_mode::active());
        Ok(Response::new(()))
    }
}
