use tonic::{Request, Response, Status};

pub use self::streaming_service_server::StreamingServiceServer;

tonic::include_proto!("streaming.v1");

pub struct StreamingService;

#[tonic::async_trait]
impl streaming_service_server::StreamingService for StreamingService {
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

    async fn send_captions(
        &self,
        request: Request<SendCaptionsRequest>,
    ) -> Result<Response<SendCaptionsResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn bitrate(
        &self,
        request: Request<BitrateRequest>,
    ) -> Result<Response<BitrateResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn set_bitrate(
        &self,
        request: Request<SetBitrateRequest>,
    ) -> Result<Response<SetBitrateResponse>, Status> {
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
