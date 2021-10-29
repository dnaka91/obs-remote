use tonic::{Request, Response, Status};

pub use self::sources_server::SourcesServer;
use crate::precondition;

tonic::include_proto!("obs_remote.v5.sources");

pub struct SourcesService;

#[tonic::async_trait]
impl sources_server::Sources for SourcesService {
    async fn list(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn is_active(&self, request: Request<String>) -> Result<Response<IsActiveReply>, Status> {
        let name = request.into_inner();
        precondition!(!name.is_empty(), "name mustn't be empty");

        Ok(Response::new(IsActiveReply {
            active: true,
            showing: true,
        }))
    }

    async fn screenshot(
        &self,
        request: Request<ScreenshotRequest>,
    ) -> Result<Response<Vec<u8>>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn save_screenshot(
        &self,
        request: Request<SaveScreenshotRequest>,
    ) -> Result<Response<()>, Status> {
        Ok(Response::new(()))
    }
}
