use tonic::{Request, Response, Status};

use self::profiles_server::Profiles;

tonic::include_proto!("obs_remote.profiles");

pub struct Service;

#[tonic::async_trait]
impl Profiles for Service {
    async fn set_current(
        &self,
        request: Request<SetCurrentRequest>,
    ) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn get_current(&self, request: Request<()>) -> Result<Response<GetCurrentReply>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn list(&self, request: Request<()>) -> Result<Response<ListReply>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }
}
