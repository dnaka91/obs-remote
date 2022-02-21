use tonic::{Request, Response, Status};

pub use self::replay_buffer_server::ReplayBufferServer;

tonic::include_proto!("obs_remote.v5.replay_buffer");

pub struct ReplayBufferService;

#[tonic::async_trait]
impl replay_buffer_server::ReplayBuffer for ReplayBufferService {
    async fn status(&self, request: Request<()>) -> Result<Response<bool>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn toggle(&self, request: Request<()>) -> Result<Response<bool>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn start(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn stop(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn save(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn last_replay(&self, request: Request<()>) -> Result<Response<String>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }
}
