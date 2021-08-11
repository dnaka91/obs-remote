use tonic::{Request, Response, Status};

use self::outputs_server::Outputs;
use crate::precondition;

tonic::include_proto!("obs_remote.outputs");

impl From<obs::output::Flags> for output::OutputFlags {
    fn from(value: obs::output::Flags) -> Self {
        Self {
            raw_value: value.bits(),
            audio: value.contains(obs::output::Flags::AUDIO),
            video: value.contains(obs::output::Flags::VIDEO),
            encoded: value.contains(obs::output::Flags::ENCODED),
            multi_track: value.contains(obs::output::Flags::MULTI_TRACK),
            service: value.contains(obs::output::Flags::SERVICE),
        }
    }
}

impl From<obs::output::Output> for Output {
    fn from(value: obs::output::Output) -> Self {
        Self {
            name: value.name(),
            ty: value.id(),
            width: value.width(),
            height: value.height(),
            flags: Some(value.flags().into()),
            settings: value.settings().to_json(),
            active: value.active(),
            reconnecting: value.reconnecting(),
            congestion: value.congestion(),
            total_frames: value.total_frames(),
            dropped_frames: value.frames_dropped(),
            total_bytes: value.total_bytes(),
        }
    }
}

pub struct Service;

#[tonic::async_trait]
impl Outputs for Service {
    async fn list(&self, request: Request<()>) -> Result<Response<ListReply>, Status> {
        Ok(Response::new(ListReply {
            outputs: obs::output::list_outputs()
                .into_iter()
                .map(Into::into)
                .collect(),
        }))
    }

    async fn info(&self, request: Request<InfoRequest>) -> Result<Response<InfoReply>, Status> {
        let name = request.into_inner().name;
        precondition!(!name.is_empty(), "output name mustn't be empty");

        let output = obs::output::Output::by_name(&name).ok_or_else(|| {
            Status::failed_precondition(format!("output `{}` doesn't exist", name))
        })?;

        Ok(Response::new(InfoReply {
            output: Some(output.into()),
        }))
    }

    async fn start(&self, request: Request<StartRequest>) -> Result<Response<()>, Status> {
        let name = request.into_inner().name;
        precondition!(!name.is_empty(), "output name mustn't be empty");

        let output = obs::output::Output::by_name(&name).ok_or_else(|| {
            Status::failed_precondition(format!("output `{}` doesn't exist", name))
        })?;

        precondition!(!output.active(), "output is already active");

        output.start();

        Ok(Response::new(()))
    }

    async fn stop(&self, request: Request<StopRequest>) -> Result<Response<()>, Status> {
        let StopRequest { name, force } = request.into_inner();
        precondition!(!name.is_empty(), "output name mustn't be empty");

        let output = obs::output::Output::by_name(&name).ok_or_else(|| {
            Status::failed_precondition(format!("output `{}` doesn't exist", name))
        })?;

        precondition!(output.active(), "output isn't active");

        if force {
            output.force_stop();
        } else {
            output.stop();
        }

        Ok(Response::new(()))
    }
}
