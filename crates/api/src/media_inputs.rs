use tonic::{Request, Response, Status};

pub use self::media_inputs_server::MediaInputsServer;

tonic::include_proto!("obs_remote.media_inputs");

pub struct MediaInputsService;

#[tonic::async_trait]
impl media_inputs_server::MediaInputs for MediaInputsService {
    async fn status(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn offset_timecode(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn set_timecode(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn is_paused(&self, request: Request<()>) -> Result<Response<bool>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn stop(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn restart(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn play_next(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn play_previous(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }
}
