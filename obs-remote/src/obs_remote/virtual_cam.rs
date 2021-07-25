use anyhow::Result;
use obs::{frontend::virtualcam, output::Output};
use tonic::{Request, Response, Status};

use self::virtual_cam_server::VirtualCam;
use super::common;
use crate::precondition;

tonic::include_proto!("obs_remote.virtual_cam");

pub struct Service;

#[tonic::async_trait]
impl VirtualCam for Service {
    async fn status(&self, request: Request<()>) -> Result<Response<StatusReply>, Status> {
        Ok(Response::new(StatusReply {
            active: virtualcam::active(),
            timecode: output_running_time(&virtualcam::output()).map(common::ns_to_timestamp),
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

fn output_running_time(output: &Output) -> Option<u64> {
    output.active().then(|| {
        let frame_time = output.video().frame_time();
        let total_frames = output.total_frames() as u64;

        frame_time * total_frames
    })
}
