use obs::{
    data::Data,
    frontend::{recording, streaming, virtualcam},
    hotkeys, service,
};
use tonic::{Request, Response, Status};

use self::streaming_server::Streaming;
use super::common;
use crate::precondition;

tonic::include_proto!("obs_remote.legacy.streaming");

impl From<StreamType> for service::StreamType {
    fn from(value: StreamType) -> Self {
        match value {
            StreamType::Unspecified | StreamType::RtmpCommon => Self::RtmpCommon,
            StreamType::RtmpCustom => Self::RtmpCustom,
        }
    }
}

impl From<service::StreamType> for StreamType {
    fn from(value: service::StreamType) -> Self {
        match value {
            service::StreamType::RtmpCommon => Self::RtmpCommon,
            service::StreamType::RtmpCustom => Self::RtmpCustom,
        }
    }
}

impl<'a> From<Data<'a>> for StreamSettings {
    fn from(value: Data<'a>) -> Self {
        Self {
            server: value.string("server"),
            key: value.string("key"),
            use_auth: value.bool("use_auth"),
            username: value.string("username"),
            password: value.string("password"),
        }
    }
}

impl From<StreamSettings> for Data<'static> {
    fn from(value: StreamSettings) -> Self {
        let mut data = Self::new();

        if let Some(server) = value.server {
            data.set_string("server", &server);
        }
        if let Some(key) = value.key {
            data.set_string("key", &key);
        }
        if let Some(use_auth) = value.use_auth {
            data.set_bool("use_auth", use_auth);
        }
        if let Some(username) = value.username {
            data.set_string("username", &username);
        }
        if let Some(password) = value.password {
            data.set_string("password", &password);
        }

        data
    }
}

const CUSTOM_SERVICE_ID: &str = "obs_remote_custom_service";

pub struct Service;

#[tonic::async_trait]
impl Streaming for Service {
    async fn status(&self, request: Request<()>) -> Result<Response<StatusReply>, Status> {
        Ok(Response::new(StatusReply {
            streaming: streaming::active(),
            recording: recording::active(),
            recording_paused: recording::paused(),
            virtualcam: virtualcam::active(),
            preview_only: false,
            stream_timecode: streaming::active()
                .then(|| common::ns_to_timestamp(common::recording_time(&streaming::output()))),
            rec_timecode: recording::active()
                .then(|| common::ns_to_timestamp(common::recording_time(&recording::output()))),
            virtualcam_timecode: virtualcam::active()
                .then(|| common::ns_to_timestamp(common::recording_time(&virtualcam::output()))),
        }))
    }

    async fn toggle(&self, request: Request<()>) -> Result<Response<()>, Status> {
        if streaming::active() {
            streaming::stop();
        } else {
            streaming::start();
        }

        Ok(Response::new(()))
    }

    async fn start(&self, request: Request<StartRequest>) -> Result<Response<()>, Status> {
        precondition!(streaming::active(), "streaming is already active");

        // TODO: setup streaming parameters
        streaming::start();
        Ok(Response::new(()))
    }

    async fn stop(&self, request: Request<()>) -> Result<Response<()>, Status> {
        precondition!(!streaming::active(), "streaming isn't active");

        streaming::stop();
        Ok(Response::new(()))
    }

    async fn set_settings(
        &self,
        request: Request<SetSettingsRequest>,
    ) -> Result<Response<()>, Status> {
        let request = request.into_inner();
        let ty = service::StreamType::from(request.ty());
        let settings = request
            .settings
            .ok_or_else(|| Status::failed_precondition("The `settings` field must be set"))?;

        let service = streaming::service();

        if ty == service.ty() {
            let service = service::Service::create(
                ty.as_str(),
                CUSTOM_SERVICE_ID,
                &settings.into(),
                &hotkeys::save_service(&service),
            );
            streaming::set_service(&service);
        } else {
            let current = service.settings();
            let mut new = Data::new();

            new.apply(&current);
            new.apply(&settings.into());

            service.update(&new);
        }

        if request.save {
            streaming::save();
        }

        Ok(Response::new(()))
    }

    async fn get_settings(
        &self,
        request: Request<()>,
    ) -> Result<Response<GetSettingsReply>, Status> {
        let service = streaming::service();
        let settings = service.settings();

        Ok(Response::new(GetSettingsReply {
            ty: StreamType::from(service.ty()) as i32,
            settings: Some(settings.into()),
        }))
    }

    async fn save_settings(&self, request: Request<()>) -> Result<Response<()>, Status> {
        streaming::save();
        Ok(Response::new(()))
    }

    async fn send_captions(
        &self,
        request: Request<SendCaptionsRequest>,
    ) -> Result<Response<()>, Status> {
        let text = request.into_inner().text;

        streaming::output().output_caption_text2(&text, 0.0);
        Ok(Response::new(()))
    }
}
