use obs::source::{self, OutputFlags, Source, SourceType, Volume};
use tonic::{Request, Response, Status};

pub use self::inputs_service_server::InputsServiceServer;
use crate::precondition;

tonic::include_proto!("obs_remote.inputs");

pub struct InputsService;

#[tonic::async_trait]
impl inputs_service_server::InputsService for InputsService {
    async fn list(&self, request: Request<ListRequest>) -> Result<Response<ListResponse>, Status> {
        let kind = request.into_inner().kind;

        let inputs = source::list()
            .into_iter()
            .filter_map(|source| {
                let id = source.id();
                ((kind.is_empty() || id == kind) && source.ty() == SourceType::Input).then(|| {
                    list_response::Input {
                        name: source.name(),
                        kind: id,
                        unversioned_kind: source.unversioned_id(),
                    }
                })
            })
            .collect();

        Ok(Response::new(ListResponse { kind, inputs }))
    }

    async fn list_kinds(
        &self,
        request: Request<ListKindsRequest>,
    ) -> Result<Response<ListKindsResponse>, Status> {
        let unversioned = request.into_inner().unversioned;

        Ok(Response::new(ListKindsResponse {
            kinds: source::list_input_types2()
                .into_iter()
                .filter_map(|id| {
                    (!source::output_flags(&id.0).contains(OutputFlags::CAP_DISABLED)).then(|| {
                        if unversioned {
                            id.1
                        } else {
                            id.0
                        }
                    })
                })
                .collect(),
        }))
    }

    async fn list_special(
        &self,
        request: Request<ListSpecialRequest>,
    ) -> Result<Response<ListSpecialResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn default_settings(
        &self,
        request: Request<DefaultSettingsRequest>,
    ) -> Result<Response<DefaultSettingsResponse>, Status> {
        let DefaultSettingsRequest { kind } = request.into_inner();
        precondition!(!kind.is_empty(), "kind mustn't be empty");

        let defaults = source::defaults(&kind)
            .ok_or_else(|| Status::failed_precondition(format!("`{}` doesn't exist", kind)))?
            .to_json();

        Ok(Response::new(DefaultSettingsResponse { defaults }))
    }

    async fn settings(
        &self,
        request: Request<SettingsRequest>,
    ) -> Result<Response<SettingsResponse>, Status> {
        let SettingsRequest { name } = request.into_inner();
        precondition!(!name.is_empty(), "name mustn't be empty");

        let source = Source::by_name(&name)
            .ok_or_else(|| Status::failed_precondition(format!("`{}` doesn't exist", name)))?;
        let settings = source.settings().to_json();

        Ok(Response::new(SettingsResponse {
            kind: source.id(),
            settings,
        }))
    }

    async fn set_settings(
        &self,
        request: Request<SetSettingsRequest>,
    ) -> Result<Response<SetSettingsResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn mute(&self, request: Request<MuteRequest>) -> Result<Response<MuteResponse>, Status> {
        let MuteRequest { name } = request.into_inner();
        precondition!(!name.is_empty(), "name mustn't be empty");

        let source = Source::by_name(&name)
            .ok_or_else(|| Status::failed_precondition(format!("`{}` doesn't exist", name)))?;

        Ok(Response::new(MuteResponse {
            muted: source.muted(),
        }))
    }

    async fn set_mute(
        &self,
        request: Request<SetMuteRequest>,
    ) -> Result<Response<SetMuteResponse>, Status> {
        let SetMuteRequest { name, muted } = request.into_inner();
        precondition!(!name.is_empty(), "name mustn't be empty");

        let source = Source::by_name(&name)
            .ok_or_else(|| Status::failed_precondition(format!("`{}` doesn't exist", name)))?;

        source.set_muted(muted);

        Ok(Response::new(SetMuteResponse {}))
    }

    async fn toggle_mute(
        &self,
        request: Request<ToggleMuteRequest>,
    ) -> Result<Response<ToggleMuteResponse>, Status> {
        let ToggleMuteRequest { name } = request.into_inner();
        precondition!(!name.is_empty(), "name mustn't be empty");

        let source = Source::by_name(&name)
            .ok_or_else(|| Status::failed_precondition(format!("`{}` doesn't exist", name)))?;

        let muted = !source.muted();
        source.set_muted(muted);

        Ok(Response::new(ToggleMuteResponse { muted }))
    }

    async fn volume(
        &self,
        request: Request<VolumeRequest>,
    ) -> Result<Response<VolumeResponse>, Status> {
        let VolumeRequest { name } = request.into_inner();
        precondition!(!name.is_empty(), "name mustn't be empty");

        let volume = Source::by_name(&name)
            .ok_or_else(|| Status::failed_precondition(format!("`{}` doesn't exist", name)))?
            .volume();

        Ok(Response::new(VolumeResponse {
            mul: volume.as_mul(),
            db: volume.as_db(),
        }))
    }

    async fn set_volume(
        &self,
        request: Request<SetVolumeRequest>,
    ) -> Result<Response<SetVolumeResponse>, Status> {
        let SetVolumeRequest { name, volume } = request.into_inner();
        precondition!(!name.is_empty(), "name mustn't be empty");

        let volume = volume.ok_or_else(|| Status::failed_precondition("volume must be set"))?;
        let input = Source::by_name(&name)
            .ok_or_else(|| Status::failed_precondition(format!("`{}` doesn't exist", name)))?;

        input.set_volume(match volume {
            set_volume_request::Volume::Mul(v) => Volume::Mul(v),
            set_volume_request::Volume::Db(v) => Volume::Db(v),
        });

        Ok(Response::new(SetVolumeResponse {}))
    }

    async fn audio_sync_offset(
        &self,
        request: Request<AudioSyncOffsetRequest>,
    ) -> Result<Response<AudioSyncOffsetResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn set_audio_sync_offset(
        &self,
        request: Request<SetAudioSyncOffsetRequest>,
    ) -> Result<Response<SetAudioSyncOffsetResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn tracks(
        &self,
        request: Request<TracksRequest>,
    ) -> Result<Response<TracksResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn set_tracks(
        &self,
        request: Request<SetTracksRequest>,
    ) -> Result<Response<SetTracksResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn monitor_mode(
        &self,
        request: Request<MonitorModeRequest>,
    ) -> Result<Response<MonitorModeResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn set_monitor_mode(
        &self,
        request: Request<SetMonitorModeRequest>,
    ) -> Result<Response<SetMonitorModeResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn is_active(
        &self,
        request: Request<IsActiveRequest>,
    ) -> Result<Response<IsActiveResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn properties_list_property_items(
        &self,
        request: Request<PropertiesListPropertyItemsRequest>,
    ) -> Result<Response<PropertiesListPropertyItemsResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn press_properties_button(
        &self,
        request: Request<PressPropertiesButtonRequest>,
    ) -> Result<Response<PressPropertiesButtonResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn set_name(
        &self,
        request: Request<SetNameRequest>,
    ) -> Result<Response<SetNameResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn create(
        &self,
        request: Request<CreateRequest>,
    ) -> Result<Response<CreateResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn remove(
        &self,
        request: Request<RemoveRequest>,
    ) -> Result<Response<RemoveResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }
}
