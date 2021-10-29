use tonic::{Request, Response, Status};

pub use self::scenes_server::ScenesServer;
use crate::precondition;

tonic::include_proto!("obs_remote.v5.scenes");

pub struct ScenesService;

#[tonic::async_trait]
impl scenes_server::Scenes for ScenesService {
    async fn list(&self, request: Request<()>) -> Result<Response<ListReply>, Status> {
        Ok(Response::new(ListReply {
            current: "test".to_owned(),
            current_preview: None,
            scenes: vec![],
        }))
    }

    async fn current(&self, request: Request<()>) -> Result<Response<String>, Status> {
        Ok(Response::new("test".to_owned()))
    }

    async fn set_current(&self, request: Request<String>) -> Result<Response<()>, Status> {
        let name = request.into_inner();
        precondition!(!name.is_empty(), "name mustn't be empty");

        Ok(Response::new(()))
    }

    async fn current_preview(&self, request: Request<()>) -> Result<Response<String>, Status> {
        Ok(Response::new("test".to_owned()))
    }

    async fn set_current_preview(&self, request: Request<String>) -> Result<Response<()>, Status> {
        let name = request.into_inner();
        precondition!(!name.is_empty(), "name mustn't be empty");

        Ok(Response::new(()))
    }

    async fn set_index(&self, request: Request<u32>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn set_name(&self, request: Request<SetNameRequest>) -> Result<Response<()>, Status> {
        let SetNameRequest { name, new_name } = request.into_inner();
        precondition!(!name.is_empty(), "name mustn't be empty");
        precondition!(!new_name.is_empty(), "new name mustn't be empty");

        Ok(Response::new(()))
    }

    async fn create(&self, request: Request<String>) -> Result<Response<()>, Status> {
        let name = request.into_inner();
        precondition!(!name.is_empty(), "name mustn't be empty");

        Ok(Response::new(()))
    }

    async fn delete(&self, request: Request<String>) -> Result<Response<()>, Status> {
        let name = request.into_inner();
        precondition!(!name.is_empty(), "name mustn't be empty");

        Ok(Response::new(()))
    }

    async fn transition_override(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn create_transition_override(
        &self,
        request: Request<()>,
    ) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn delete_transition_override(
        &self,
        request: Request<()>,
    ) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }
}
