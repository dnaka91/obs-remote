use obs::{
    source::{MediaState, Source},
    Duration,
};
use tonic::{Request, Response, Status};

use self::media_control_server::MediaControl;
use super::common::DurationExt;
use crate::precondition;

tonic::include_proto!("obs_remote.legacy.media_control");

pub struct Service;

#[tonic::async_trait]
impl MediaControl for Service {
    async fn toggle(&self, request: Request<ToggleRequest>) -> Result<Response<()>, Status> {
        let ToggleRequest { name, play_pause } = request.into_inner();
        precondition!(!name.is_empty(), "name mustn't be empty");

        let source = Source::by_name(&name)
            .ok_or_else(|| Status::failed_precondition(format!("`{name}` doesn't exist")))?;

        let play_pause = play_pause.unwrap_or_else(|| source.media_state() == MediaState::Playing);

        source.media_play_pause(play_pause);

        Ok(Response::new(()))
    }

    async fn restart(&self, request: Request<Identifier>) -> Result<Response<()>, Status> {
        source_from_identifier(request)?.media_restart();
        Ok(Response::new(()))
    }

    async fn stop(&self, request: Request<Identifier>) -> Result<Response<()>, Status> {
        source_from_identifier(request)?.media_stop();
        Ok(Response::new(()))
    }

    async fn next(&self, request: Request<Identifier>) -> Result<Response<()>, Status> {
        source_from_identifier(request)?.media_next();
        Ok(Response::new(()))
    }

    async fn previous(&self, request: Request<Identifier>) -> Result<Response<()>, Status> {
        source_from_identifier(request)?.media_previous();
        Ok(Response::new(()))
    }

    async fn duration(
        &self,
        request: Request<Identifier>,
    ) -> Result<Response<DurationReply>, Status> {
        let duration = source_from_identifier(request)?.media_duration();

        Ok(Response::new(DurationReply {
            duration: Some(duration.into_proto()),
        }))
    }

    async fn get_time(
        &self,
        request: Request<Identifier>,
    ) -> Result<Response<GetTimeReply>, Status> {
        let time = source_from_identifier(request)?.media_time();

        Ok(Response::new(GetTimeReply {
            timestamp: Some(time.into_proto()),
        }))
    }

    async fn set_time(&self, request: Request<SetTimeRequest>) -> Result<Response<()>, Status> {
        let SetTimeRequest { name, timestamp } = request.into_inner();
        precondition!(!name.is_empty(), "name mustn't be empty");

        let timestamp =
            timestamp.ok_or_else(|| Status::failed_precondition("timestamp must be defined"))?;

        let source = Source::by_name(&name)
            .ok_or_else(|| Status::failed_precondition(format!("`{name}` doesn't exist")))?;

        source.media_set_time(
            Duration::seconds(timestamp.seconds) + Duration::nanoseconds(timestamp.nanos.into()),
        );

        Ok(Response::new(()))
    }

    async fn scrub(&self, request: Request<ScrubRequest>) -> Result<Response<()>, Status> {
        let ScrubRequest { name, offset } = request.into_inner();
        precondition!(!name.is_empty(), "name mustn't be empty");

        let offset = offset.ok_or_else(|| Status::failed_precondition("offset must be defined"))?;

        let source = Source::by_name(&name)
            .ok_or_else(|| Status::failed_precondition(format!("`{name}` doesn't exist")))?;

        let time = source.media_time()
            + Duration::seconds(offset.seconds)
            + Duration::nanoseconds(offset.nanos.into());

        source.media_set_time(time);

        Ok(Response::new(()))
    }

    async fn state(&self, request: Request<Identifier>) -> Result<Response<StateReply>, Status> {
        let mut reply = StateReply::default();
        reply.set_state(source_from_identifier(request)?.media_state().into());

        Ok(Response::new(reply))
    }
}

fn source_from_identifier(request: Request<Identifier>) -> Result<Source<'static>, Status> {
    let name = request.into_inner().name;
    precondition!(!name.is_empty(), "name mustn't be empty");

    Source::by_name(&name)
        .ok_or_else(|| Status::failed_precondition(format!("`{name}` doesn't exist")))
}
