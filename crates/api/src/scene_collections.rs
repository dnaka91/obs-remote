use obs::frontend::scene_collections;
use tonic::{Request, Response, Status};

pub use self::scene_collections_service_server::SceneCollectionsServiceServer;
use crate::precondition;

tonic::include_proto!("obs_remote.scene_collections.v1");

pub struct SceneCollectionsService;

#[tonic::async_trait]
impl scene_collections_service_server::SceneCollectionsService for SceneCollectionsService {
    async fn list(&self, request: Request<ListRequest>) -> Result<Response<ListResponse>, Status> {
        let ListRequest {} = request.into_inner();

        Ok(Response::new(ListResponse {
            current: scene_collections::current(),
            collections: scene_collections::list(),
        }))
    }

    async fn current(
        &self,
        request: Request<CurrentRequest>,
    ) -> Result<Response<CurrentResponse>, Status> {
        let CurrentRequest {} = request.into_inner();

        Ok(Response::new(CurrentResponse {
            name: scene_collections::current(),
        }))
    }

    async fn set_current(
        &self,
        request: Request<SetCurrentRequest>,
    ) -> Result<Response<SetCurrentResponse>, Status> {
        let SetCurrentRequest { name } = request.into_inner();
        precondition!(!name.is_empty(), "name mustn't be empty");

        let found = scene_collections::list()
            .into_iter()
            .find(|sc| sc == &name)
            .ok_or_else(|| Status::failed_precondition(format!("`{}` doesn't exist", name)))?;

        if scene_collections::current() != found {
            scene_collections::set_current(&found);
        }

        Ok(Response::new(SetCurrentResponse {}))
    }

    async fn create(
        &self,
        request: Request<CreateRequest>,
    ) -> Result<Response<CreateResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn remove(
        &self,
        request: Request<RemoveRequest>,
    ) -> Result<Response<RemoveResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }
}
