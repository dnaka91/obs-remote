use anyhow::Result;
use obs::frontend::virtualcam;
use tonic::{Request, Response, Status};

use self::virtual_cam_server::VirtualCam;
use crate::precondition;

tonic::include_proto!("obs_remote.virtual_cam");

pub struct Service;

#[tonic::async_trait]
impl VirtualCam for Service {
    async fn status(&self, request: Request<()>) -> Result<Response<StatusReply>, Status> {
        Ok(Response::new(StatusReply {
            active: virtualcam::active(),
            timecode: None,
        }))
    }

    async fn toggle(&self, request: Request<()>) -> Result<Response<()>, Status> {
        if virtualcam::active() {
            virtualcam::stop();
        } else {
            virtualcam::start();
        }

        Ok(Response::new(()))
    }

    async fn start(&self, request: Request<()>) -> Result<Response<()>, Status> {
        precondition!(!virtualcam::active(), "virtual cam is already active");

        virtualcam::start();
        Ok(Response::new(()))
    }

    async fn stop(&self, request: Request<()>) -> Result<Response<()>, Status> {
        precondition!(virtualcam::active(), "virtual cam isn't active");

        virtualcam::stop();
        Ok(Response::new(()))
    }
}
