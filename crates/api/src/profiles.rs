use obs::frontend::profiles;
use serde_json::{Map, Value};
use tonic::{Request, Response, Status};

pub use self::profiles_service_server::ProfilesServiceServer;
use crate::{precondition, util};

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
        let PersistentDataRequest { name } = request.into_inner();
        let profile_path = obs::frontend::profiles::current_path();
        let data_path = profile_path.join("obsWebSocketPersistentData.json");

        let result = async {
            let buf = tokio::fs::read(data_path).await?;
            let mut data = serde_json::from_slice::<Map<String, Value>>(&buf)?;

            anyhow::Ok(data.remove(&name))
        };

        match result.await {
            Ok(v) => Ok(Response::new(PersistentDataResponse {
                value: v.map(util::json_to_proto),
            })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn set_persistent_data(
        &self,
        request: Request<SetPersistentDataRequest>,
    ) -> Result<Response<SetPersistentDataResponse>, Status> {
        let SetPersistentDataRequest { name, value } = request.into_inner();
        let value = value.and_then(util::proto_to_json);
        let profile_path = obs::frontend::profiles::current_path();
        let data_path = profile_path.join("obsWebSocketPersistentData.json");

        let result = async {
            let buf = tokio::fs::read(data_path).await?;
            let mut data = serde_json::from_slice::<Map<String, Value>>(&buf)?;

            match value {
                Some(value) => {
                    data.insert(name, value);
                }
                None => {
                    data.remove(&name);
                }
            }

            anyhow::Ok(())
        };

        match result.await {
            Ok(()) => Ok(Response::new(SetPersistentDataResponse {})),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn create(
        &self,
        request: Request<CreateRequest>,
    ) -> Result<Response<CreateResponse>, Status> {
        let CreateRequest { name } = request.into_inner();

        precondition!(
            !profiles::list().contains(&name),
            "profile with that name already exists"
        );

        profiles::create_profile(&name);

        Ok(Response::new(CreateResponse {}))
    }

    async fn remove(
        &self,
        request: Request<RemoveRequest>,
    ) -> Result<Response<RemoveResponse>, Status> {
        let RemoveRequest { name } = request.into_inner();

        precondition!(profiles::list().contains(&name), "profile doesn't exist");

        profiles::delete_profile(&name);

        Ok(Response::new(RemoveResponse {}))
    }
}
