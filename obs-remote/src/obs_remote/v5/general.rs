use tonic::{Request, Response, Status};

pub use self::general_server::GeneralServer;

tonic::include_proto!("obs_remote.v5.general");

pub struct GeneralService;

#[tonic::async_trait]
impl general_server::General for GeneralService {
    async fn version(&self, request: Request<()>) -> Result<Response<VersionReply>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn broadcast_event(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn system_stats(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn is_studio_mode_enabled(&self, request: Request<()>) -> Result<Response<bool>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn set_studio_mode_enabled(
        &self,
        request: Request<bool>,
    ) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn sleep(&self, request: Request<prost_types::Duration>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }
}
