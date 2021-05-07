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

        if obs::frontend::scene_collections().contains(&name) {
            obs::frontend::set_current_scene_collection(&name);
            Ok(Response::new(()))
        } else {
            Err(Status::failed_precondition(
                "scene collection doesn't exist",
            ))
        }
    }

    async fn get_current(&self, request: Request<()>) -> Result<Response<GetCurrentReply>, Status> {
        Ok(Response::new(GetCurrentReply {
            name: obs::frontend::current_scene_collection(),
        }))
    }

    async fn list(&self, request: Request<()>) -> Result<Response<ListReply>, Status> {
        Ok(Response::new(ListReply {
            scene_collections: obs::frontend::scene_collections(),
        }))
    }
}
