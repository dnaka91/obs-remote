use tonic::{Request, Response, Status};

use self::studio_mode_server::StudioMode;
use crate::precondition;

tonic::include_proto!("obs_remote.studio_mode");

pub struct Service;

#[tonic::async_trait]
impl StudioMode for Service {
    async fn status(&self, request: Request<()>) -> Result<Response<StatusReply>, Status> {
        Ok(Response::new(StatusReply { active: false }))
    }

    #[allow(clippy::cast_possible_wrap, clippy::cast_precision_loss)]
    async fn get_preview_scene(
        &self,
        request: Request<()>,
    ) -> Result<Response<GetPreviewSceneReply>, Status> {
        Ok(Response::new(GetPreviewSceneReply {
            name: "test".to_owned(),
            sources: vec![],
        }))
    }

    async fn set_preview_scene(
        &self,
        request: Request<SetPreviewSceneRequest>,
    ) -> Result<Response<()>, Status> {
        Ok(Response::new(()))
    }

    #[allow(clippy::cast_possible_wrap, clippy::cast_precision_loss)]
    async fn transition_to_program(
        &self,
        request: Request<TransitionToProgramRequest>,
    ) -> Result<Response<()>, Status> {
        if let Some(info) = request.into_inner().with_transition {
            if let Some(name) = info.name {
                precondition!(!name.is_empty(), "transition name must be set");
            }
        }

        Ok(Response::new(()))
    }

    async fn enable(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Ok(Response::new(()))
    }

    async fn disable(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Ok(Response::new(()))
    }

    async fn toggle(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Ok(Response::new(()))
    }
}
