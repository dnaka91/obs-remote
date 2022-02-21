use tonic::{Request, Response, Status};

pub use self::transitions_server::TransitionsServer;

tonic::include_proto!("obs_remote.v5.transitions");

pub struct TransitionsService;

#[tonic::async_trait]
impl transitions_server::Transitions for TransitionsService {
    async fn list(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn current(&self, request: Request<()>) -> Result<Response<String>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn set_current(&self, request: Request<String>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn set_current_duration(
        &self,
        request: Request<prost_types::Duration>,
    ) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn settings(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn set_settings(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn release_t_bar(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn set_t_bar_position(&self, request: Request<f32>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn trigger_studio_mode_transition(
        &self,
        request: Request<()>,
    ) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn create(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn delete(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }
}
