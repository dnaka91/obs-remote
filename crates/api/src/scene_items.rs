use tonic::{Request, Response, Status};

pub use self::scene_items_service_server::SceneItemsServiceServer;

tonic::include_proto!("obs_remote.scene_items.v1");

pub struct SceneItemsService;

#[tonic::async_trait]
impl scene_items_service_server::SceneItemsService for SceneItemsService {
    async fn list(&self, request: Request<ListRequest>) -> Result<Response<ListResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn list_group(
        &self,
        request: Request<ListGroupRequest>,
    ) -> Result<Response<ListGroupResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn transform(
        &self,
        request: Request<TransformRequest>,
    ) -> Result<Response<TransformResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn set_transform(
        &self,
        request: Request<SetTransformRequest>,
    ) -> Result<Response<SetTransformResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn is_enabled(
        &self,
        request: Request<IsEnabledRequest>,
    ) -> Result<Response<IsEnabledResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn set_enabled(
        &self,
        request: Request<SetEnabledRequest>,
    ) -> Result<Response<SetEnabledResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn is_locked(
        &self,
        request: Request<IsLockedRequest>,
    ) -> Result<Response<IsLockedResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn set_locked(
        &self,
        request: Request<SetLockedRequest>,
    ) -> Result<Response<SetLockedResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn color(
        &self,
        request: Request<ColorRequest>,
    ) -> Result<Response<ColorResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn set_color(
        &self,
        request: Request<SetColorRequest>,
    ) -> Result<Response<SetColorResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn set_index(
        &self,
        request: Request<SetIndexRequest>,
    ) -> Result<Response<SetIndexResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
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

    async fn duplicate(
        &self,
        request: Request<DuplicateRequest>,
    ) -> Result<Response<DuplicateResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }
}
