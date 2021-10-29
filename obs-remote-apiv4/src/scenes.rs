use tonic::{Request, Response, Status};

use self::scenes_server::Scenes;
use crate::precondition;

tonic::include_proto!("obs_remote.scenes");

pub struct Service;

#[tonic::async_trait]
impl Scenes for Service {
    async fn set_current(
        &self,
        request: Request<SetCurrentRequest>,
    ) -> Result<Response<()>, Status> {
        let name = request.into_inner().name;
        precondition!(!name.is_empty(), "name mustn't be empty");

        Ok(Response::new(()))
    }

    async fn get_current(&self, request: Request<()>) -> Result<Response<GetCurrentReply>, Status> {
        Ok(Response::new(GetCurrentReply {
            name: "test".to_owned(),
            sources: vec![],
        }))
    }

    async fn list(&self, request: Request<()>) -> Result<Response<ListReply>, Status> {
        Ok(Response::new(ListReply {
            current: "test".to_owned(),
            scenes: vec![],
        }))
    }

    async fn create(&self, request: Request<CreateRequest>) -> Result<Response<()>, Status> {
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

        Ok(Response::new(()))
    }

    async fn remove_transition_override(
        &self,
        request: Request<RemoveTransitionOverrideRequest>,
    ) -> Result<Response<()>, Status> {
        let name = request.into_inner().name;
        precondition!(!name.is_empty(), "name mustn't be empty");

        Ok(Response::new(()))
    }

    async fn get_transition_override(
        &self,
        request: Request<GetTransitionOverrideRequest>,
    ) -> Result<Response<GetTransitionOverrideReply>, Status> {
        let name = request.into_inner().name;
        precondition!(!name.is_empty(), "name mustn't be empty");

        Ok(Response::new(GetTransitionOverrideReply {
            name,
            duration: None,
        }))
    }
}
