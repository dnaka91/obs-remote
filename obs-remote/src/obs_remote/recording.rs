use obs::frontend::recording;
use tonic::{Request, Response, Status};

use self::recording_server::Recording;
use super::common;
use crate::precondition;

tonic::include_proto!("obs_remote.recording");

pub struct Service;

#[tonic::async_trait]
impl Recording for Service {
    async fn status(&self, request: Request<()>) -> Result<Response<StatusReply>, Status> {
        Ok(Response::new(StatusReply {
            is_recording: recording::active(),
            is_recording_paused: recording::paused(),
            record_timecode: recording::active()
                .then(|| common::ns_to_timestamp(common::recording_time(&recording::output()))),
            recording_filename: recording_filename().unwrap_or_default(),
        }))
    }

    async fn toggle(&self, request: Request<()>) -> Result<Response<()>, Status> {
        if recording::active() {
            recording::stop();
        } else {
            recording::start();
        }

        Ok(Response::new(()))
    }

    async fn start(&self, request: Request<()>) -> Result<Response<()>, Status> {
        precondition!(recording::active(), "recording is already active");

        recording::start();
        Ok(Response::new(()))
    }

    async fn stop(&self, request: Request<()>) -> Result<Response<()>, Status> {
        precondition!(!recording::active(), "recording isn't active");

        recording::stop();
        Ok(Response::new(()))
    }

    async fn pause(&self, request: Request<()>) -> Result<Response<()>, Status> {
        precondition!(!recording::active(), "recording isn't active");
        precondition!(recording::paused(), "recording is already paused");

        recording::pause(true);
        Ok(Response::new(()))
    }

    async fn resume(&self, request: Request<()>) -> Result<Response<()>, Status> {
        precondition!(!recording::active(), "recording isn't active");
        precondition!(!recording::paused(), "recording isn't paused");

        recording::pause(false);
        Ok(Response::new(()))
    }

    async fn set_recording_folder(
        &self,
        request: Request<SetRecordingFolderRequest>,
    ) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn get_recording_folder(
        &self,
        request: Request<()>,
    ) -> Result<Response<GetRecordingFolderReply>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }
}

fn recording_filename() -> Option<String> {
    let output = recording::output();
    output
        .active()
        .then(|| {
            let settings = output.settings();
            settings
                .item_by_name("url")
                .or_else(|| settings.item_by_name("path"))
                .and_then(|item| item.string())
        })
        .flatten()
}
