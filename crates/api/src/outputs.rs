use tonic::{Request, Response, Status};

pub use self::outputs_service_server::OutputsServiceServer;

tonic::include_proto!("obs_remote.outputs.v1");

pub struct OutputsService;

#[tonic::async_trait]
impl outputs_service_server::OutputsService for OutputsService {
    async fn list(&self, request: Request<ListRequest>) -> Result<Response<ListResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

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

    async fn settings(
        &self,
        request: Request<SettingsRequest>,
    ) -> Result<Response<SettingsResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn set_settings(
        &self,
        request: Request<SetSettingsRequest>,
    ) -> Result<Response<SetSettingsResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }
}
