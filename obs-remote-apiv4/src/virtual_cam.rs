use anyhow::Result;
use tonic::{Request, Response, Status};

use self::virtual_cam_server::VirtualCam;

tonic::include_proto!("obs_remote.virtual_cam");

pub struct Service;

#[tonic::async_trait]
impl VirtualCam for Service {
    async fn status(&self, request: Request<()>) -> Result<Response<StatusReply>, Status> {
        Ok(Response::new(StatusReply {
            active: true,
            timecode: None,
        }))
    }

    async fn toggle(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Ok(Response::new(()))
    }

    async fn start(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Ok(Response::new(()))
    }

    async fn stop(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Ok(Response::new(()))
    }
}
