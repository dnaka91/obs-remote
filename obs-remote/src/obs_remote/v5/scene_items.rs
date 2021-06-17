use tonic::{Request, Response, Status};

pub use self::scene_items_server::SceneItemsServer;

tonic::include_proto!("obs_remote.v5.scene_items");

pub struct SceneItemsService;

#[tonic::async_trait]
impl scene_items_server::SceneItems for SceneItemsService {
    async fn list(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn list_group(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn transform(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn set_transform(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn is_enabled(&self, request: Request<()>) -> Result<Response<bool>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn set_enabled(&self, request: Request<bool>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn is_locked(&self, request: Request<()>) -> Result<Response<bool>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn set_locked(&self, request: Request<bool>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn color(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn set_color(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn set_index(&self, request: Request<u32>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn create(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn delete(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn duplicate(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }
}
