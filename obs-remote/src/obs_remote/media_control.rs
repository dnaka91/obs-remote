use tonic::{Request, Response, Status};

use self::media_control_server::MediaControl;

tonic::include_proto!("obs_remote.media_control");

pub struct Service;

#[tonic::async_trait]
impl MediaControl for Service {
    async fn toggle(&self, request: Request<ToggleRequest>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn restart(&self, request: Request<Identifier>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn stop(&self, request: Request<Identifier>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn next(&self, request: Request<Identifier>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn previous(&self, request: Request<Identifier>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn duration(
        &self,
        request: Request<Identifier>,
    ) -> Result<Response<DurationReply>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn get_time(
        &self,
        request: Request<Identifier>,
    ) -> Result<Response<GetTimeReply>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn set_time(&self, request: Request<SetTimeReqeust>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn scrub(&self, request: Request<ScrubReqeust>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn state(&self, request: Request<Identifier>) -> Result<Response<StateReply>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }
}
