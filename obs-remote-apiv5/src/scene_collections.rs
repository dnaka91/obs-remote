use tonic::{Request, Response, Status};

pub use self::scene_collections_server::SceneCollectionsServer;
use crate::precondition;

tonic::include_proto!("obs_remote.v5.scene_collections");

pub struct SceneCollectionsService;

#[tonic::async_trait]
impl scene_collections_server::SceneCollections for SceneCollectionsService {
    async fn list(&self, request: Request<()>) -> Result<Response<ListReply>, Status> {
        Ok(Response::new(ListReply {
            current: "test".to_owned(),
            collections: vec![],
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

    async fn create(&self, request: Request<String>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn delete(&self, request: Request<String>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }
}
