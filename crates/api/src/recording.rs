use tonic::{Request, Response, Status};

pub use self::recording_server::RecordingServer;

tonic::include_proto!("obs_remote.recording");

pub struct RecordingService;

#[tonic::async_trait]
impl recording_server::Recording for RecordingService {
    async fn status(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn toggle(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn start(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn stop(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn toggle_pause(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn pause(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn resume(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn directory(&self, request: Request<()>) -> Result<Response<String>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn set_directory(&self, request: Request<String>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn filename_formatting(&self, request: Request<()>) -> Result<Response<String>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn set_filename_formatting(
        &self,
        request: Request<()>,
    ) -> Result<Response<String>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }
}
