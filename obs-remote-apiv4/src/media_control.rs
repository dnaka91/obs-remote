use tonic::{Request, Response, Status};

use self::media_control_server::MediaControl;
use crate::precondition;

tonic::include_proto!("obs_remote.media_control");

pub struct Service;

#[tonic::async_trait]
impl MediaControl for Service {
    async fn toggle(&self, request: Request<ToggleRequest>) -> Result<Response<()>, Status> {
        let ToggleRequest { name, play_pause } = request.into_inner();
        precondition!(!name.is_empty(), "name mustn't be empty");

        Ok(Response::new(()))
    }

    async fn restart(&self, request: Request<Identifier>) -> Result<Response<()>, Status> {
        source_from_identifier(request)?;
        Ok(Response::new(()))
    }

    async fn stop(&self, request: Request<Identifier>) -> Result<Response<()>, Status> {
        source_from_identifier(request)?;
        Ok(Response::new(()))
    }

    async fn next(&self, request: Request<Identifier>) -> Result<Response<()>, Status> {
        source_from_identifier(request)?;
        Ok(Response::new(()))
    }

    async fn previous(&self, request: Request<Identifier>) -> Result<Response<()>, Status> {
        source_from_identifier(request)?;
        Ok(Response::new(()))
    }

    async fn duration(
        &self,
        request: Request<Identifier>,
    ) -> Result<Response<DurationReply>, Status> {
        source_from_identifier(request)?;

        Ok(Response::new(DurationReply { duration: None }))
    }

    async fn get_time(
        &self,
        request: Request<Identifier>,
    ) -> Result<Response<GetTimeReply>, Status> {
        source_from_identifier(request)?;

        Ok(Response::new(GetTimeReply { timestamp: None }))
    }

    async fn set_time(&self, request: Request<SetTimeRequest>) -> Result<Response<()>, Status> {
        let SetTimeRequest { name, timestamp } = request.into_inner();
        precondition!(!name.is_empty(), "name mustn't be empty");

        let timestamp =
            timestamp.ok_or_else(|| Status::failed_precondition("timestamp must be defined"))?;

        Ok(Response::new(()))
    }

    async fn scrub(&self, request: Request<ScrubRequest>) -> Result<Response<()>, Status> {
        let ScrubRequest { name, offset } = request.into_inner();
        precondition!(!name.is_empty(), "name mustn't be empty");

        let offset = offset.ok_or_else(|| Status::failed_precondition("offset must be defined"))?;

        Ok(Response::new(()))
    }

    async fn state(&self, request: Request<Identifier>) -> Result<Response<StateReply>, Status> {
        let reply = StateReply::default();
        source_from_identifier(request)?;

        Ok(Response::new(reply))
    }
}

fn source_from_identifier(request: Request<Identifier>) -> Result<(), Status> {
    let name = request.into_inner().name;
    precondition!(!name.is_empty(), "name mustn't be empty");

    Err(Status::failed_precondition(format!(
        "`{}` doesn't exist",
        name
    )))
}
