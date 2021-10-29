use tonic::{Request, Response, Status};

use self::transitions_server::Transitions;
use crate::precondition;

tonic::include_proto!("obs_remote.transitions");

pub struct Service;

#[tonic::async_trait]
impl Transitions for Service {
    async fn list(&self, request: Request<()>) -> Result<Response<ListReply>, Status> {
        Ok(Response::new(ListReply {
            current: "test".to_owned(),
            transitions: vec![],
        }))
    }

    #[allow(clippy::cast_sign_loss)]
    async fn get_current(&self, request: Request<()>) -> Result<Response<GetCurrentReply>, Status> {
        Ok(Response::new(GetCurrentReply {
            name: "test".to_owned(),
            duration: None,
        }))
    }

    async fn set_current(
        &self,
        request: Request<SetCurrentRequest>,
    ) -> Result<Response<()>, Status> {
        let name = request.into_inner().name;
        precondition!(!name.is_empty(), "transition name mustn't be empty");

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

        Ok(Response::new(()))
    }

    #[allow(clippy::cast_sign_loss)]
    async fn get_duration(
        &self,
        request: Request<()>,
    ) -> Result<Response<GetDurationReply>, Status> {
        Ok(Response::new(GetDurationReply { duration: None }))
    }

    async fn position(&self, request: Request<()>) -> Result<Response<PositionReply>, Status> {
        Ok(Response::new(PositionReply { position: 5.0 }))
    }

    async fn get_settings(
        &self,
        request: Request<GetSettingsRequest>,
    ) -> Result<Response<GetSettingsReply>, Status> {
        let name = request.into_inner().name;
        precondition!(!name.is_empty(), "transition name mustn't be empty");

        Ok(Response::new(GetSettingsReply {
            settings: "{}".to_owned(),
        }))
    }

    async fn set_settings(
        &self,
        request: Request<SetSettingsRequest>,
    ) -> Result<Response<SetSettingsReply>, Status> {
        let SetSettingsRequest { name, settings } = request.into_inner();
        precondition!(!name.is_empty(), "transition name mustn't be empty");
        precondition!(!settings.is_empty(), "settings mustn't be empty");

        Ok(Response::new(SetSettingsReply {
            settings: "{}".to_owned(),
        }))
    }

    async fn release_t_bar(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Ok(Response::new(()))
    }

    async fn set_t_bar_position(
        &self,
        request: Request<SetTBarPositionRequest>,
    ) -> Result<Response<()>, Status> {
        let SetTBarPositionRequest { position, release } = request.into_inner();
        precondition!((0.0..=1.0).contains(&position), "position out of range");

        Ok(Response::new(()))
    }
}
