use obs::frontend::virtualcam;
use tonic::{Request, Response, Status};

pub use self::virtual_cam_service_server::VirtualCamServiceServer;
use crate::precondition;

tonic::include_proto!("virtual_cam.v1");

pub struct VirtualCamService;

#[tonic::async_trait]
impl virtual_cam_service_server::VirtualCamService for VirtualCamService {
    async fn status(
        &self,
        request: Request<StatusRequest>,
    ) -> Result<Response<StatusResponse>, Status> {
        let StatusRequest {} = request.into_inner();

        Ok(Response::new(StatusResponse {
            active: virtualcam::active(),
        }))
    }

    async fn toggle(
        &self,
        request: Request<ToggleRequest>,
    ) -> Result<Response<ToggleResponse>, Status> {
        let ToggleRequest {} = request.into_inner();

        let active = virtualcam::active();
        if active {
            virtualcam::stop();
        } else {
            virtualcam::start();
        }

        Ok(Response::new(ToggleResponse { active: !active }))
    }

    async fn start(
        &self,
        request: Request<StartRequest>,
    ) -> Result<Response<StartResponse>, Status> {
        let StartRequest {} = request.into_inner();

        precondition!(!virtualcam::active(), "virtual cam already active");

        virtualcam::start();

        Ok(Response::new(StartResponse {}))
    }

    async fn stop(&self, request: Request<StopRequest>) -> Result<Response<StopResponse>, Status> {
        let StopRequest {} = request.into_inner();

        precondition!(virtualcam::active(), "virtual cam already inactive");

        virtualcam::stop();

        Ok(Response::new(StopResponse {}))
    }
}
