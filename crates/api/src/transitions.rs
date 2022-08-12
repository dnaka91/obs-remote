use tonic::{Request, Response, Status};

pub use self::transitions_service_server::TransitionsServiceServer;

tonic::include_proto!("obs_remote.transitions");

pub struct TransitionsService;

#[tonic::async_trait]
impl transitions_service_server::TransitionsService for TransitionsService {
    async fn list(&self, request: Request<ListRequest>) -> Result<Response<ListResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn current(
        &self,
        request: Request<CurrentRequest>,
    ) -> Result<Response<CurrentResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn set_current(
        &self,
        request: Request<SetCurrentRequest>,
    ) -> Result<Response<SetCurrentResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn set_current_duration(
        &self,
        request: Request<SetCurrentDurationRequest>,
    ) -> Result<Response<SetCurrentDurationResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn settings(
        &self,
        request: Request<SettingsRequest>,
    ) -> Result<Response<SettingsResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn set_settings(
        &self,
        request: Request<SetSettingsRequest>,
    ) -> Result<Response<SetSettingsResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn release_t_bar(
        &self,
        request: Request<ReleaseTBarRequest>,
    ) -> Result<Response<ReleaseTBarResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn set_t_bar_position(
        &self,
        request: Request<SetTBarPositionRequest>,
    ) -> Result<Response<SetTBarPositionResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn trigger_studio_mode_transition(
        &self,
        request: Request<TriggerStudioModeTransitionRequest>,
    ) -> Result<Response<TriggerStudioModeTransitionResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn create(
        &self,
        request: Request<CreateRequest>,
    ) -> Result<Response<CreateResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn delete(
        &self,
        request: Request<DeleteRequest>,
    ) -> Result<Response<DeleteResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }
}
