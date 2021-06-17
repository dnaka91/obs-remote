use obs::frontend::scene_collections;
use tonic::{Request, Response, Status};

use self::scene_collections_server::SceneCollections;

tonic::include_proto!("obs_remote.scene_collections");

#[derive(Debug)]
pub struct Service;

#[tonic::async_trait]
impl SceneCollections for Service {
    async fn set_current(
        &self,
        request: Request<SetCurrentRequest>,
    ) -> Result<Response<()>, Status> {
        let name = request.into_inner().name;

        if scene_collections::list().contains(&name) {
            scene_collections::set_current(&name);
            Ok(Response::new(()))
        } else {
            Err(Status::failed_precondition(
                "scene collection doesn't exist",
            ))
        }
    }

    async fn get_current(&self, request: Request<()>) -> Result<Response<GetCurrentReply>, Status> {
        Ok(Response::new(GetCurrentReply {
            name: scene_collections::current(),
        }))
    }

    async fn list(&self, request: Request<()>) -> Result<Response<ListReply>, Status> {
        Ok(Response::new(ListReply {
            scene_collections: scene_collections::list(),
        }))
    }
}
