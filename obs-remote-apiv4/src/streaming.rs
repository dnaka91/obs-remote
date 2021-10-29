use tonic::{Request, Response, Status};

use self::streaming_server::Streaming;

tonic::include_proto!("obs_remote.streaming");

pub struct Service;

#[tonic::async_trait]
impl Streaming for Service {
    async fn status(&self, request: Request<()>) -> Result<Response<StatusReply>, Status> {
        Ok(Response::new(StatusReply {
            streaming: true,
            recording: true,
            recording_paused: false,
            virtualcam: false,
            preview_only: false,
            stream_timecode: None,
            rec_timecode: None,
            virtualcam_timecode: None,
        }))
    }

    async fn toggle(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Ok(Response::new(()))
    }

    async fn start(&self, request: Request<StartRequest>) -> Result<Response<()>, Status> {
        Ok(Response::new(()))
    }

    async fn stop(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Ok(Response::new(()))
    }

    async fn set_settings(
        &self,
        request: Request<SetSettingsRequest>,
    ) -> Result<Response<()>, Status> {
        let request = request.into_inner();
        let settings = request
            .settings
            .ok_or_else(|| Status::failed_precondition("The `settings` field must be set"))?;

        Ok(Response::new(()))
    }

    async fn get_settings(
        &self,
        request: Request<()>,
    ) -> Result<Response<GetSettingsReply>, Status> {
        Ok(Response::new(GetSettingsReply {
            ty: StreamType::RtmpCommon as i32,
            settings: None,
        }))
    }

    async fn save_settings(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Ok(Response::new(()))
    }

    async fn send_captions(
        &self,
        request: Request<SendCaptionsRequest>,
    ) -> Result<Response<()>, Status> {
        Ok(Response::new(()))
    }
}
