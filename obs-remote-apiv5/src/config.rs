use tonic::{Request, Response, Status};

pub use self::config_server::ConfigServer;

tonic::include_proto!("obs_remote.v5.config");

pub struct ConfigService;

#[tonic::async_trait]
impl config_server::Config for ConfigService {
    async fn global_persistent_data(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn set_global_persistent_data(
        &self,
        request: Request<()>,
    ) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn video_settings(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn set_video_settings(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }
}
