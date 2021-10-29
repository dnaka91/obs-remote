use tonic::{Request, Response, Status};

use self::outputs_server::Outputs;
use crate::precondition;

tonic::include_proto!("obs_remote.outputs");

pub struct Service;

#[tonic::async_trait]
impl Outputs for Service {
    async fn list(&self, request: Request<()>) -> Result<Response<ListReply>, Status> {
        Ok(Response::new(ListReply { outputs: vec![] }))
    }

    async fn info(&self, request: Request<InfoRequest>) -> Result<Response<InfoReply>, Status> {
        let name = request.into_inner().name;
        precondition!(!name.is_empty(), "output name mustn't be empty");

        Ok(Response::new(InfoReply { output: None }))
    }

    async fn start(&self, request: Request<StartRequest>) -> Result<Response<()>, Status> {
        let name = request.into_inner().name;
        precondition!(!name.is_empty(), "output name mustn't be empty");

        Ok(Response::new(()))
    }

    async fn stop(&self, request: Request<StopRequest>) -> Result<Response<()>, Status> {
        let StopRequest { name, force } = request.into_inner();
        precondition!(!name.is_empty(), "output name mustn't be empty");

        Ok(Response::new(()))
    }
}
