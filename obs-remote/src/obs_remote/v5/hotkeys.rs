use tonic::{Request, Response, Status};

pub use self::hotkeys_server::HotkeysServer;

tonic::include_proto!("obs_remote.v5.hotkeys");

pub struct HotkeysService;

#[tonic::async_trait]
impl hotkeys_server::Hotkeys for HotkeysService {
    async fn list(&self, request: Request<()>) -> Result<Response<ListReply>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn trigger_by_name(
        &self,
        request: Request<TriggerByNameRequest>,
    ) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn trigger_by_sequence(
        &self,
        request: Request<TriggerBySequenceRequest>,
    ) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }
}
