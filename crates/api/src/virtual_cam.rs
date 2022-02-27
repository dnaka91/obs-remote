use tonic::{Request, Response, Status};

pub use self::virtual_cam_server::VirtualCamServer;

tonic::include_proto!("obs_remote.virtual_cam");

pub struct VirtualCamService;

#[tonic::async_trait]
impl virtual_cam_server::VirtualCam for VirtualCamService {
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
}
