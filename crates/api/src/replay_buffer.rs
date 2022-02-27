use obs::{callback::calldata::Calldata, frontend::replay_buffer};
use tonic::{Request, Response, Status};

pub use self::replay_buffer_server::ReplayBufferServer;
use crate::{precondition, precondition_fn};

tonic::include_proto!("obs_remote.replay_buffer");

pub struct ReplayBufferService;

#[tonic::async_trait]
impl replay_buffer_server::ReplayBuffer for ReplayBufferService {
    async fn status(&self, request: Request<()>) -> Result<Response<bool>, Status> {
        precondition!(
            replay_buffer::output().is_some(),
            "replay buffer not available"
        );

        Ok(Response::new(replay_buffer::active()))
    }

    async fn toggle(&self, request: Request<()>) -> Result<Response<bool>, Status> {
        precondition!(
            replay_buffer::output().is_some(),
            "replay buffer not available"
        );

        let active = replay_buffer::active();

        if active {
            replay_buffer::stop();
        } else {
            replay_buffer::start();
        }

        Ok(Response::new(!active))
    }

    async fn start(&self, request: Request<()>) -> Result<Response<()>, Status> {
        precondition!(
            replay_buffer::output().is_some(),
            "replay buffer not available"
        );
        precondition!(!replay_buffer::active(), "replay buffer already active");

        replay_buffer::start();

        Ok(Response::new(()))
    }

    async fn stop(&self, request: Request<()>) -> Result<Response<()>, Status> {
        precondition!(
            replay_buffer::output().is_some(),
            "replay buffer not available"
        );
        precondition!(replay_buffer::active(), "replay buffer already inactive");

        replay_buffer::stop();

        Ok(Response::new(()))
    }

    async fn save(&self, request: Request<()>) -> Result<Response<()>, Status> {
        precondition!(
            replay_buffer::output().is_some(),
            "replay buffer not available"
        );
        precondition!(replay_buffer::active(), "replay buffer inactive");

        replay_buffer::save();

        Ok(Response::new(()))
    }

    async fn last_replay(&self, request: Request<()>) -> Result<Response<String>, Status> {
        let output =
            replay_buffer::output().ok_or_else(precondition_fn!("replay buffer not available"))?;
        let mut handler = output.proc_handler();
        let mut calldata = Calldata::default();

        handler.call("get_last_replay", &mut calldata);

        Ok(Response::new(calldata.string("path").unwrap_or_default()))
    }
}
