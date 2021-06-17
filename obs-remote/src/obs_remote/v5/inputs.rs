use obs::source::{self, OutputFlags, Source, SourceType};
use tonic::{Request, Response, Status};

use crate::precondition;

pub use self::inputs_server::InputsServer;

tonic::include_proto!("obs_remote.v5.inputs");

pub struct InputsService;

#[tonic::async_trait]
impl inputs_server::Inputs for InputsService {
    async fn list(&self, request: Request<ListRequest>) -> Result<Response<ListReply>, Status> {
        let kind = request.into_inner().kind;

        let inputs = source::list()
            .into_iter()
            .filter_map(|source| {
                let id = source.id();
                ((kind.is_empty() || id == kind) && source.ty() == SourceType::Input).then(|| {
                    list_reply::Input {
                        name: source.name(),
                        kind: id,
                        unversioned_kind: source.unversioned_id(),
                    }
                })
            })
            .collect();

        Ok(Response::new(ListReply { kind, inputs }))
    }

    async fn list_kinds(
        &self,
        request: Request<ListKindsRequest>,
    ) -> Result<Response<ListKindsReply>, Status> {
        let unversioned = request.into_inner().unversioned;

        Ok(Response::new(ListKindsReply {
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

    async fn list_special(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn default_settings(
        &self,
        request: Request<DefaultSettingsRequest>,
    ) -> Result<Response<String>, Status> {
        let kind = request.into_inner().kind;
        precondition!(!kind.is_empty(), "kind mustn't be empty");

        let defaults = source::defaults(&kind)
            .ok_or_else(|| Status::failed_precondition(format!("`{}` doesn't exist", kind)))?
            .to_json();

        Ok(Response::new(defaults))
    }

    async fn settings(&self, request: Request<String>) -> Result<Response<SettingsReply>, Status> {
        let name = request.into_inner();
        precondition!(!name.is_empty(), "name mustn't be empty");

        let source = Source::by_name(&name)
            .ok_or_else(|| Status::failed_precondition(format!("`{}` doesn't exist", name)))?;

        Ok(Response::new(SettingsReply {
            kind: source.id(),
            settings: source.settings().to_json(),
        }))
    }

    async fn set_settings(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn mute(&self, request: Request<String>) -> Result<Response<bool>, Status> {
        let name = request.into_inner();
        precondition!(!name.is_empty(), "name mustn't be empty");

        let source = Source::by_name(&name)
            .ok_or_else(|| Status::failed_precondition(format!("`{}` doesn't exist", name)))?;

        Ok(Response::new(source.muted()))
    }

    async fn set_mute(&self, request: Request<SetMuteRequest>) -> Result<Response<()>, Status> {
        let SetMuteRequest { name, muted } = request.into_inner();
        precondition!(!name.is_empty(), "name mustn't be empty");

        let source = Source::by_name(&name)
            .ok_or_else(|| Status::failed_precondition(format!("`{}` doesn't exist", name)))?;

        source.set_muted(muted);

        Ok(Response::new(()))
    }

    async fn toggle_mute(&self, request: Request<String>) -> Result<Response<bool>, Status> {
        let name = request.into_inner();
        precondition!(!name.is_empty(), "name mustn't be empty");

        let source = Source::by_name(&name)
            .ok_or_else(|| Status::failed_precondition(format!("`{}` doesn't exist", name)))?;

        let muted = !source.muted();
        source.set_muted(muted);

        Ok(Response::new(muted))
    }

    async fn volume(&self, request: Request<String>) -> Result<Response<VolumeReply>, Status> {
        let name = request.into_inner();
        precondition!(!name.is_empty(), "name mustn't be empty");

        let volume = Source::by_name(&name)
            .ok_or_else(|| Status::failed_precondition(format!("`{}` doesn't exist", name)))?
            .volume();

        Ok(Response::new(VolumeReply {
            mul: volume.as_mul(),
            db: volume.as_db(),
        }))
    }

    async fn set_volume(&self, request: Request<f32>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn audio_sync_offset(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn set_audio_sync_offset(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn tracks(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn set_tracks(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn monitor_mode(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn set_monitor_mode(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn is_active(&self, request: Request<()>) -> Result<Response<bool>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn properties_list_property_items(
        &self,
        request: Request<()>,
    ) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn press_properties_button(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn set_name(&self, request: Request<String>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn create(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn delete(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }
}
