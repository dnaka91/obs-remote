use tonic::{Request, Response, Status};

use self::scene_items_server::SceneItems;

tonic::include_proto!("obs_remote.scene_items");

pub struct Service;

#[tonic::async_trait]
impl SceneItems for Service {
    async fn list(&self, request: Request<ListRequest>) -> Result<Response<ListReply>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn get_properties(
        &self,
        request: Request<GetPropertiesRequest>,
    ) -> Result<Response<GetPropertiesReply>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn set_properties(
        &self,
        request: Request<SetPropertiesRequest>,
    ) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn reset(&self, request: Request<ResetRequest>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn set_render(&self, request: Request<SetRenderRequest>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn delete(&self, request: Request<DeleteRequest>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn add(&self, request: Request<AddRequest>) -> Result<Response<AddReply>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn duplicate(
        &self,
        request: Request<DuplicateRequest>,
    ) -> Result<Response<DuplicateReply>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }
}
