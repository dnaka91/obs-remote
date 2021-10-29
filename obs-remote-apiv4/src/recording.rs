use tokio::fs;
use tonic::{Request, Response, Status};

use self::recording_server::Recording;

tonic::include_proto!("obs_remote.recording");

pub struct Service;

#[tonic::async_trait]
impl Recording for Service {
    async fn status(&self, request: Request<()>) -> Result<Response<StatusReply>, Status> {
        Ok(Response::new(StatusReply {
            is_recording: true,
            is_recording_paused: false,
            record_timecode: None,
            recording_filename: "test".to_owned(),
        }))
    }

    async fn toggle(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Ok(Response::new(()))
    }

    async fn start(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Ok(Response::new(()))
    }

    async fn stop(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Ok(Response::new(()))
    }

    async fn pause(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Ok(Response::new(()))
    }

    async fn resume(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Ok(Response::new(()))
    }

    async fn set_recording_folder(
        &self,
        request: Request<SetRecordingFolderRequest>,
    ) -> Result<Response<()>, Status> {
        let rec_folder = request.into_inner().rec_folder;
        fs::create_dir_all(&rec_folder).await.map_err(|e| {
            Status::invalid_argument(format!("failed creating recording folder: {:?}", e))
        })?;

        Ok(Response::new(()))
    }

    async fn get_recording_folder(
        &self,
        request: Request<()>,
    ) -> Result<Response<GetRecordingFolderReply>, Status> {
        Ok(Response::new(GetRecordingFolderReply {
            rec_folder: "test".to_owned(),
        }))
    }
}
