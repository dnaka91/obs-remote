use tonic::{Request, Response, Status};

pub use self::projectors_service_server::ProjectorsServiceServer;

tonic::include_proto!("obs_remote.projectors");

pub struct ProjectorsService;

#[tonic::async_trait]
impl projectors_service_server::ProjectorsService for ProjectorsService {
    async fn list(&self, request: Request<ListRequest>) -> Result<Response<ListResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn open(&self, request: Request<OpenRequest>) -> Result<Response<OpenResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn close(
        &self,
        request: Request<CloseRequest>,
    ) -> Result<Response<CloseResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }
}
