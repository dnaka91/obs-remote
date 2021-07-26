use obs::{
    data,
    frontend::{preview_mode, transitions},
    Duration,
};
use tonic::{Request, Response, Status};

use self::transitions_server::Transitions;
use super::common::DurationExt;
use crate::precondition;

tonic::include_proto!("obs_remote.transitions");

pub struct Service;

#[tonic::async_trait]
impl Transitions for Service {
    async fn list(&self, request: Request<()>) -> Result<Response<ListReply>, Status> {
        use self::list_reply::Transition;

        let current = transitions::current();

        Ok(Response::new(ListReply {
            current: transitions::current().name(),
            transitions: transitions::list()
                .into_iter()
                .map(|t| Transition { name: t.name() })
                .collect(),
        }))
    }

    #[allow(clippy::cast_sign_loss)]
    async fn get_current(&self, request: Request<()>) -> Result<Response<GetCurrentReply>, Status> {
        let current = transitions::current();

        Ok(Response::new(GetCurrentReply {
            name: current.name(),
            duration: (!current.transition_fixed()).then(|| transitions::duration().into_proto()),
        }))
    }

    async fn set_current(
        &self,
        request: Request<SetCurrentRequest>,
    ) -> Result<Response<()>, Status> {
        let name = request.into_inner().name;
        precondition!(!name.is_empty(), "transition name mustn't be empty");

        transitions::set_current(
            &transitions::list()
                .into_iter()
                .find(|t| t.name() == name)
                .ok_or_else(|| {
                    Status::failed_precondition(format!("transition `{}` doesn't exist", name))
                })?,
        );

        Ok(Response::new(()))
    }

    #[allow(clippy::cast_possible_truncation)]
    async fn set_duration(
        &self,
        request: Request<SetDurationRequest>,
    ) -> Result<Response<()>, Status> {
        let duration = request
            .into_inner()
            .duration
            .ok_or_else(|| Status::failed_precondition("duration must be set"))?;
        let duration = Duration::from_proto(duration);
        precondition!(
            duration >= Duration::zero(),
            "invalid duration (must be positive)"
        );

        transitions::set_duration(duration);
        Ok(Response::new(()))
    }

    #[allow(clippy::cast_sign_loss)]
    async fn get_duration(
        &self,
        request: Request<()>,
    ) -> Result<Response<GetDurationReply>, Status> {
        Ok(Response::new(GetDurationReply {
            duration: Some(transitions::duration().into_proto()),
        }))
    }

    async fn position(&self, request: Request<()>) -> Result<Response<PositionReply>, Status> {
        Ok(Response::new(PositionReply {
            position: transitions::current().transition_time(),
        }))
    }

    async fn get_settings(
        &self,
        request: Request<GetSettingsRequest>,
    ) -> Result<Response<GetSettingsReply>, Status> {
        let name = request.into_inner().name;
        precondition!(!name.is_empty(), "transition name mustn't be empty");

        let transition = transitions::list()
            .into_iter()
            .find(|source| source.name() == name)
            .ok_or_else(|| {
                Status::failed_precondition(format!("transition `{}` doesn't exist", name))
            })?;

        Ok(Response::new(GetSettingsReply {
            settings: transition.settings().to_json(),
        }))
    }

    async fn set_settings(
        &self,
        request: Request<SetSettingsRequest>,
    ) -> Result<Response<SetSettingsReply>, Status> {
        let SetSettingsRequest { name, settings } = request.into_inner();
        precondition!(!name.is_empty(), "transition name mustn't be empty");
        precondition!(!settings.is_empty(), "settings mustn't be empty");

        let transition = transitions::list()
            .into_iter()
            .find(|source| source.name() == name)
            .ok_or_else(|| {
                Status::failed_precondition(format!("transition `{}` doesn't exist", name))
            })?;

        let data = data::Data::from_json(&settings)
            .map_err(|e| Status::failed_precondition(format!("invalid JSON data: {:?}", e)))?;

        transition.update(data);
        transition.update_properties();

        Ok(Response::new(SetSettingsReply {
            settings: transition.settings().to_json(),
        }))
    }

    async fn release_t_bar(&self, request: Request<()>) -> Result<Response<()>, Status> {
        precondition!(preview_mode::active(), "studio mode isn't enabled");
        precondition!(
            !transitions::current().transition_fixed(),
            "current transition doesn't support t-bar control"
        );

        transitions::release_tbar();

        Ok(Response::new(()))
    }

    async fn set_t_bar_position(
        &self,
        request: Request<SetTBarPositionRequest>,
    ) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }
}
