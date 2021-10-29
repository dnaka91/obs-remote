use anyhow::Result;
use tonic::{Request, Response, Status};

use self::replay_buffer_server::ReplayBuffer;

tonic::include_proto!("obs_remote.replay_buffer");

pub struct Service;

#[tonic::async_trait]
impl ReplayBuffer for Service {
    async fn status(&self, request: Request<()>) -> Result<Response<StatusReply>, Status> {
        Ok(Response::new(StatusReply { active: true }))
    }

    async fn toggle(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Ok(Response::new(()))
    }

    async fn start(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Ok(Response::new(()))
    }

    async fn stop(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::failed_precondition("replay buffer is not active"))
    }

    async fn save(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::failed_precondition("replay buffer is not active"))
    }
}
