use tonic::{Request, Response, Status};

use self::profiles_server::Profiles;
use crate::precondition;

tonic::include_proto!("obs_remote.profiles");

pub struct Service;

#[tonic::async_trait]
impl Profiles for Service {
    async fn set_current(
        &self,
        request: Request<SetCurrentRequest>,
    ) -> Result<Response<()>, Status> {
        let name = request.into_inner().name;
        precondition!(!name.is_empty(), "name mustn't be empty");

        Ok(Response::new(()))
    }

    async fn get_current(&self, request: Request<()>) -> Result<Response<GetCurrentReply>, Status> {
        Ok(Response::new(GetCurrentReply {
            name: "test".to_owned(),
        }))
    }

    async fn list(&self, request: Request<()>) -> Result<Response<ListReply>, Status> {
        Ok(Response::new(ListReply { profiles: vec![] }))
    }
}
