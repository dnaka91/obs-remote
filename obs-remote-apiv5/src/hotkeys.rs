use tonic::{Request, Response, Status};

pub use self::hotkeys_server::HotkeysServer;
use crate::precondition;

tonic::include_proto!("obs_remote.v5.hotkeys");

pub struct HotkeysService;

#[tonic::async_trait]
impl hotkeys_server::Hotkeys for HotkeysService {
    async fn list(&self, request: Request<()>) -> Result<Response<ListReply>, Status> {
        Ok(Response::new(ListReply { hotkeys: vec![] }))
    }

    async fn trigger_by_name(
        &self,
        request: Request<TriggerByNameRequest>,
    ) -> Result<Response<()>, Status> {
        let name = request.into_inner().name;
        precondition!(!name.is_empty(), "name mustn't be empty");

        Ok(Response::new(()))
    }

    async fn trigger_by_sequence(
        &self,
        request: Request<TriggerBySequenceRequest>,
    ) -> Result<Response<()>, Status> {
        Ok(Response::new(()))
    }
}
