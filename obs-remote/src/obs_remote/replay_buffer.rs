use anyhow::Result;
use obs::frontend::replay_buffer;
use tonic::{Request, Response, Status};

use self::replay_buffer_server::ReplayBuffer;

tonic::include_proto!("obs_remote.replay_buffer");

pub struct Service;

#[tonic::async_trait]
impl ReplayBuffer for Service {
    async fn status(&self, request: Request<()>) -> Result<Response<StatusReply>, Status> {
        Ok(Response::new(StatusReply {
            active: replay_buffer::active(),
        }))
    }

    async fn toggle(&self, request: Request<()>) -> Result<Response<()>, Status> {
        if replay_buffer::active() {
            replay_buffer::stop();
        } else {
            replay_buffer::start();
        }

        Ok(Response::new(()))
    }

    async fn start(&self, request: Request<()>) -> Result<Response<()>, Status> {
        let profile = obs::frontend::profile_config();
        let enabled = || -> Result<_> {
            Ok(match profile.string("Output", "Mode").as_deref() {
                Some(mode) => match mode {
                    "Simple" => profile.bool("SimpleOutput", "RecRB"),
                    "Advanced" => profile.bool("AdvOut", "RecRB"),
                    _ => None,
                }
                .unwrap_or_default(),
                None => false,
            })
        };

        match enabled() {
            Err(e) => return Err(Status::internal(e.to_string())),
            Ok(false) => return Err(Status::failed_precondition("replayed buffer is disabled")),
            Ok(true) => (),
        }

        if replay_buffer::active() {
            return Err(Status::failed_precondition(
                "replay buffer is already active",
            ));
        }

        replay_buffer::start();

        Ok(Response::new(()))
    }

    async fn stop(&self, request: Request<()>) -> Result<Response<()>, Status> {
        if replay_buffer::active() {
            replay_buffer::stop();
            Ok(Response::new(()))
        } else {
            Err(Status::failed_precondition("replay buffer is not active"))
        }
    }

    async fn save(&self, request: Request<()>) -> Result<Response<()>, Status> {
        if replay_buffer::active() {
            replay_buffer::save();
            Ok(Response::new(()))
        } else {
            Err(Status::failed_precondition("replay buffer is not active"))
        }
    }
}
