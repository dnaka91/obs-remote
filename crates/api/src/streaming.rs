use tonic::{Request, Response, Status};

pub use self::streaming_server::StreamingServer;

tonic::include_proto!("obs_remote.streaming");

pub struct StreamingService;

#[tonic::async_trait]
impl streaming_server::Streaming for StreamingService {
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

    async fn send_captions(&self, request: Request<String>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn bitrate(&self, request: Request<()>) -> Result<Response<u32>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn set_bitrate(&self, request: Request<u32>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn settings(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn set_settings(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }
}
