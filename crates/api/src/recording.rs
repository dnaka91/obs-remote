use tonic::{Request, Response, Status};

pub use self::recording_service_server::RecordingServiceServer;

tonic::include_proto!("recording.v1");

pub struct RecordingService;

#[tonic::async_trait]
impl recording_service_server::RecordingService for RecordingService {
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

    async fn toggle_pause(
        &self,
        request: Request<TogglePauseRequest>,
    ) -> Result<Response<TogglePauseResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn pause(
        &self,
        request: Request<PauseRequest>,
    ) -> Result<Response<PauseResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn resume(
        &self,
        request: Request<ResumeRequest>,
    ) -> Result<Response<ResumeResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn directory(
        &self,
        request: Request<DirectoryRequest>,
    ) -> Result<Response<DirectoryResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn set_directory(
        &self,
        request: Request<SetDirectoryRequest>,
    ) -> Result<Response<SetDirectoryResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn filename_formatting(
        &self,
        request: Request<FilenameFormattingRequest>,
    ) -> Result<Response<FilenameFormattingResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn set_filename_formatting(
        &self,
        request: Request<SetFilenameFormattingRequest>,
    ) -> Result<Response<SetFilenameFormattingResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }
}
