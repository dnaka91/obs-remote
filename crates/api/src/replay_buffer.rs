use obs::{callback::calldata::Calldata, frontend::replay_buffer};
use tonic::{Request, Response, Status};

pub use self::replay_buffer_service_server::ReplayBufferServiceServer;
use crate::{precondition, precondition_fn};

tonic::include_proto!("obs_remote.replay_buffer");

pub struct ReplayBufferService;

#[tonic::async_trait]
impl replay_buffer_service_server::ReplayBufferService for ReplayBufferService {
    async fn status(
        &self,
        request: Request<StatusRequest>,
    ) -> Result<Response<StatusResponse>, Status> {
        let StatusRequest {} = request.into_inner();

        precondition!(
            replay_buffer::output().is_some(),
            "replay buffer not available"
        );

        Ok(Response::new(StatusResponse {
            active: replay_buffer::active(),
        }))
    }

    async fn toggle(
        &self,
        request: Request<ToggleRequest>,
    ) -> Result<Response<ToggleResponse>, Status> {
        let ToggleRequest {} = request.into_inner();

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

        Ok(Response::new(ToggleResponse { active: !active }))
    }

    async fn start(
        &self,
        request: Request<StartRequest>,
    ) -> Result<Response<StartResponse>, Status> {
        let StartRequest {} = request.into_inner();

        precondition!(
            replay_buffer::output().is_some(),
            "replay buffer not available"
        );
        precondition!(!replay_buffer::active(), "replay buffer already active");

        replay_buffer::start();

        Ok(Response::new(StartResponse {}))
    }

    async fn stop(&self, request: Request<StopRequest>) -> Result<Response<StopResponse>, Status> {
        let StopRequest {} = request.into_inner();

        precondition!(
            replay_buffer::output().is_some(),
            "replay buffer not available"
        );
        precondition!(replay_buffer::active(), "replay buffer already inactive");

        replay_buffer::stop();

        Ok(Response::new(StopResponse {}))
    }

    async fn save(&self, request: Request<SaveRequest>) -> Result<Response<SaveResponse>, Status> {
        let SaveRequest {} = request.into_inner();

        precondition!(
            replay_buffer::output().is_some(),
            "replay buffer not available"
        );
        precondition!(replay_buffer::active(), "replay buffer inactive");

        replay_buffer::save();

        Ok(Response::new(SaveResponse {}))
    }

    async fn last_replay(
        &self,
        request: Request<LastReplayRequest>,
    ) -> Result<Response<LastReplayResponse>, Status> {
        let LastReplayRequest {} = request.into_inner();

        let output =
            replay_buffer::output().ok_or_else(precondition_fn!("replay buffer not available"))?;
        let mut handler = output.proc_handler();
        let mut calldata = Calldata::default();

        handler.call("get_last_replay", &mut calldata);

        Ok(Response::new(LastReplayResponse {
            file: calldata.string("path").unwrap_or_default(),
        }))
    }
}
