use obs::frontend::profiles;
use tonic::{Request, Response, Status};

pub use self::profiles_service_server::ProfilesServiceServer;
use crate::precondition;

tonic::include_proto!("profiles.v1");

pub struct ProfilesService;

#[tonic::async_trait]
impl profiles_service_server::ProfilesService for ProfilesService {
    async fn list(&self, request: Request<ListRequest>) -> Result<Response<ListResponse>, Status> {
        let ListRequest {} = request.into_inner();

        Ok(Response::new(ListResponse {
            current: profiles::current(),
            profiles: profiles::list(),
        }))
    }

    async fn current(
        &self,
        request: Request<CurrentRequest>,
    ) -> Result<Response<CurrentResponse>, Status> {
        let CurrentRequest {} = request.into_inner();

        Ok(Response::new(CurrentResponse {
            name: profiles::current(),
        }))
    }

    async fn set_current(
        &self,
        request: Request<SetCurrentRequest>,
    ) -> Result<Response<SetCurrentResponse>, Status> {
        let SetCurrentRequest { name } = request.into_inner();
        precondition!(!name.is_empty(), "name mustn't be empty");

        let found = profiles::list()
            .into_iter()
            .find(|sc| sc == &name)
            .ok_or_else(|| Status::failed_precondition(format!("`{}` doesn't exist", name)))?;

        if profiles::current() != found {
            profiles::set_current(&found);
        }

        Ok(Response::new(SetCurrentResponse {}))
    }

    async fn parameter(
        &self,
        request: Request<ParameterRequest>,
    ) -> Result<Response<ParameterResponse>, Status> {
        let ParameterRequest { category, name } = request.into_inner();
        precondition!(!category.is_empty(), "category mustn't be empty");
        precondition!(!name.is_empty(), "name mustn't be empty");

        let profile = profiles::config();

        Ok(Response::new(ParameterResponse {
            value: profile.string(&category, &name),
            default: profile.default_string(&category, &name),
        }))
    }

    async fn set_parameter(
        &self,
        request: Request<SetParameterRequest>,
    ) -> Result<Response<SetParameterResponse>, Status> {
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

        Ok(Response::new(SetParameterResponse {}))
    }

    async fn persistent_data(
        &self,
        request: Request<PersistentDataRequest>,
    ) -> Result<Response<PersistentDataResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn set_persistent_data(
        &self,
        request: Request<SetPersistentDataRequest>,
    ) -> Result<Response<SetPersistentDataResponse>, Status> {
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
}
