use tonic::{Request, Response, Status};

pub use self::media_inputs_service_server::MediaInputsServiceServer;

tonic::include_proto!("media_inputs.v1");

pub struct MediaInputsService;

#[tonic::async_trait]
impl media_inputs_service_server::MediaInputsService for MediaInputsService {
    async fn status(
        &self,
        request: Request<StatusRequest>,
    ) -> Result<Response<StatusResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn offset_timecode(
        &self,
        request: Request<OffsetTimecodeRequest>,
    ) -> Result<Response<OffsetTimecodeResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn set_timecode(
        &self,
        request: Request<SetTimecodeRequest>,
    ) -> Result<Response<SetTimecodeResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn is_paused(
        &self,
        request: Request<IsPausedRequest>,
    ) -> Result<Response<IsPausedResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn stop(&self, request: Request<StopRequest>) -> Result<Response<StopResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn restart(
        &self,
        request: Request<RestartRequest>,
    ) -> Result<Response<RestartResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn play_next(
        &self,
        request: Request<PlayNextRequest>,
    ) -> Result<Response<PlayNextResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn play_previous(
        &self,
        request: Request<PlayPreviousRequest>,
    ) -> Result<Response<PlayPreviousResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }
}
