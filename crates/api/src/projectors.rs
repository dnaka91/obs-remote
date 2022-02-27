use tonic::{Request, Response, Status};

pub use self::projectors_server::ProjectorsServer;

tonic::include_proto!("obs_remote.projectors");

pub struct ProjectorsService;

#[tonic::async_trait]
impl projectors_server::Projectors for ProjectorsService {
    async fn list(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn open(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn close(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }
}
