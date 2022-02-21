use obs::frontend::profiles;
use prost_types::Any;
use tonic::{Request, Response, Status};

pub use self::profiles_server::ProfilesServer;
use crate::precondition;

tonic::include_proto!("obs_remote.v5.profiles");

pub struct ProfilesService;

#[tonic::async_trait]
impl profiles_server::Profiles for ProfilesService {
    async fn list(&self, request: Request<()>) -> Result<Response<ListReply>, Status> {
        Ok(Response::new(ListReply {
            current: profiles::current(),
            profiles: profiles::list(),
        }))
    }

    async fn current(&self, request: Request<()>) -> Result<Response<String>, Status> {
        Ok(Response::new(profiles::current()))
    }

    async fn set_current(&self, request: Request<String>) -> Result<Response<()>, Status> {
        let name = request.into_inner();
        precondition!(!name.is_empty(), "name mustn't be empty");

        let found = profiles::list()
            .into_iter()
            .find(|sc| sc == &name)
            .ok_or_else(|| Status::failed_precondition(format!("`{}` doesn't exist", name)))?;

        if profiles::current() != found {
            profiles::set_current(&found);
        }

        Ok(Response::new(()))
    }

    async fn parameter(
        &self,
        request: Request<ParameterRequest>,
    ) -> Result<Response<ParameterReply>, Status> {
        let ParameterRequest { category, name } = request.into_inner();
        precondition!(!category.is_empty(), "category mustn't be empty");
        precondition!(!name.is_empty(), "name mustn't be empty");

        let profile = profiles::config();

        Ok(Response::new(ParameterReply {
            value: profile.string(&category, &name),
            default: profile.default_string(&category, &name),
        }))
    }

    async fn set_parameter(
        &self,
        request: Request<SetParameterRequest>,
    ) -> Result<Response<()>, Status> {
        let SetParameterRequest {
            category,
            name,
            value,
        } = request.into_inner();
        precondition!(!category.is_empty(), "category mustn't be empty");
        precondition!(!name.is_empty(), "name mustn't be empty");

        let profile = profiles::config();

        if let Some(value) = value {
            profile.set_string(&category, &name, &value);
        } else {
            profile.remove_value(&category, &name);
        }

        Ok(Response::new(()))
    }

    async fn persistent_data(&self, request: Request<String>) -> Result<Response<Any>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn set_persistent_data(
        &self,
        request: Request<SetPersistentDataRequest>,
    ) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn create(&self, request: Request<String>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn delete(&self, request: Request<String>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }
}
