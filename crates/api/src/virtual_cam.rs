use tonic::{Request, Response, Status};

pub use self::virtual_cam_service_server::VirtualCamServiceServer;

tonic::include_proto!("obs_remote.virtual_cam");

pub struct VirtualCamService;

#[tonic::async_trait]
impl virtual_cam_service_server::VirtualCamService for VirtualCamService {
    async fn status(
        &self,
        request: Request<StatusRequest>,
    ) -> Result<Response<StatusResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn toggle(
        &self,
        request: Request<ToggleRequest>,
    ) -> Result<Response<ToggleResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn start(
        &self,
        request: Request<StartRequest>,
    ) -> Result<Response<StartResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn stop(&self, request: Request<StopRequest>) -> Result<Response<StopResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }
}
