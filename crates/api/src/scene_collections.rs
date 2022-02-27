use obs::frontend::scene_collections;
use tonic::{Request, Response, Status};

pub use self::scene_collections_server::SceneCollectionsServer;
use crate::precondition;

tonic::include_proto!("obs_remote.scene_collections");

pub struct SceneCollectionsService;

#[tonic::async_trait]
impl scene_collections_server::SceneCollections for SceneCollectionsService {
    async fn list(&self, request: Request<()>) -> Result<Response<ListReply>, Status> {
        Ok(Response::new(ListReply {
            current: scene_collections::current(),
            collections: scene_collections::list(),
        }))
    }

    async fn current(&self, request: Request<()>) -> Result<Response<String>, Status> {
        Ok(Response::new(scene_collections::current()))
    }

    async fn set_current(&self, request: Request<String>) -> Result<Response<()>, Status> {
        let name = request.into_inner();
        precondition!(!name.is_empty(), "name mustn't be empty");

        let found = scene_collections::list()
            .into_iter()
            .find(|sc| sc == &name)
            .ok_or_else(|| Status::failed_precondition(format!("`{}` doesn't exist", name)))?;

        if scene_collections::current() != found {
            scene_collections::set_current(&found);
        }

        Ok(Response::new(()))
    }

    async fn create(&self, request: Request<String>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn delete(&self, request: Request<String>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }
}
