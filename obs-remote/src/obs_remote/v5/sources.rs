use tonic::{Request, Response, Status};

pub use self::sources_server::SourcesServer;

tonic::include_proto!("obs_remote.v5.sources");

pub struct SourcesService;

#[tonic::async_trait]
impl sources_server::Sources for SourcesService {
    async fn list(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn is_active(&self, request: Request<()>) -> Result<Response<bool>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn screenshot(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn save_screenshot(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }
}
