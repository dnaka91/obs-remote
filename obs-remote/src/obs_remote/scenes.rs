use tonic::{Request, Response, Status};

use self::scenes_server::Scenes;

tonic::include_proto!("obs_remote.scenes");

pub struct Service;

#[tonic::async_trait]
impl Scenes for Service {
    async fn set_current(
        &self,
        request: Request<SetCurrentRequest>,
    ) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn get_current(&self, request: Request<()>) -> Result<Response<GetCurrentReply>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn list(&self, request: Request<()>) -> Result<Response<ListReply>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn create(&self, request: Request<CreateRequest>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn reorder(&self, request: Request<ReorderRequest>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn set_transition_override(
        &self,
        request: Request<SetTransitionOverrideRequest>,
    ) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn remove_transition_override(
        &self,
        request: Request<RemoveTransitionOverrideRequest>,
    ) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn get_transition_override(
        &self,
        request: Request<GetTransitionOverrideRequest>,
    ) -> Result<Response<GetTransitionOverrideReply>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }
}
