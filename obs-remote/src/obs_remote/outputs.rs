use tonic::{Request, Response, Status};

use self::outputs_server::Outputs;

tonic::include_proto!("obs_remote.outputs");

pub struct Service;

#[tonic::async_trait]
impl Outputs for Service {
    async fn list(&self, request: Request<()>) -> Result<Response<ListReply>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn info(&self, request: Request<InfoRequest>) -> Result<Response<InfoReply>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn start(&self, request: Request<StartRequest>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn stop(&self, request: Request<StopRequest>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }
}
