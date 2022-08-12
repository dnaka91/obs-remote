use obs::{data::Data, filter::OrderMovement, source::Source};
use tonic::{Request, Response, Status};

pub use self::filters_service_server::FiltersServiceServer;
use crate::{precondition, precondition_fn};

tonic::include_proto!("obs_remote.filters");

pub struct FiltersService;

#[tonic::async_trait]
impl filters_service_server::FiltersService for FiltersService {
    async fn list(&self, request: Request<ListRequest>) -> Result<Response<ListResponse>, Status> {
        let ListRequest { source_name } = request.into_inner();
        let source = Source::by_name(&source_name)
            .ok_or_else(precondition_fn!("`{source_name}` doesn't exist"))?;

        let filters = source
            .filters()
            .into_iter()
            .enumerate()
            .map(|(i, filter)| Filter {
                enabled: filter.enabled(),
                index: i as u32,
                kind: filter.id(),
                name: filter.name(),
                settings: filter.settings().to_json(),
            })
            .collect();

        Ok(Response::new(ListResponse { filters }))
    }

    async fn default_settings(
        &self,
        request: Request<DefaultSettingsRequest>,
    ) -> Result<Response<DefaultSettingsResponse>, Status> {
        let DefaultSettingsRequest { kind } = request.into_inner();

        let defaults = obs::source::defaults(&kind)
            .ok_or_else(precondition_fn!("`{kind}` doesn't exist"))?
            .to_json();

        Ok(Response::new(DefaultSettingsResponse { defaults }))
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
        let RemoveRequest { identifier } = request.into_inner();
        let Identifier { source, filter } =
            identifier.ok_or_else(|| Status::invalid_argument("identifier must be specified"))?;

        let source =
            Source::by_name(&source).ok_or_else(precondition_fn!("`{source}` doesn't exist"))?;
        let filter = source
            .filter_by_name(&filter)
            .ok_or_else(precondition_fn!("`{filter}` doesn't exist"))?;

        filter.into_source().remove();

        Ok(Response::new(RemoveResponse {}))
    }

    async fn get(&self, request: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        let GetRequest { identifier } = request.into_inner();
        let Identifier { source, filter } =
            identifier.ok_or_else(|| Status::invalid_argument("identifier must be specified"))?;

        let source =
            Source::by_name(&source).ok_or_else(precondition_fn!("`{source}` doesn't exist"))?;
        let filter = source
            .filter_by_name(&filter)
            .ok_or_else(precondition_fn!("`{filter}` doesn't exist"))?;

        let filter = Filter {
            enabled: filter.source().enabled(),
            index: filter.index().unwrap_or_default() as u32,
            kind: filter.source().id(),
            name: filter.source().name(),
            settings: filter.source().settings().to_json(),
        };

        Ok(Response::new(GetResponse {
            filter: Some(filter),
        }))
    }

    async fn set_name(
        &self,
        request: Request<SetNameRequest>,
    ) -> Result<Response<SetNameResponse>, Status> {
        let SetNameRequest {
            identifier,
            new_name,
        } = request.into_inner();
        let Identifier { source, filter } =
            identifier.ok_or_else(precondition_fn!("identifier must be specified"))?;

        let source =
            Source::by_name(&source).ok_or_else(precondition_fn!("`{source}` doesn't exist"))?;
        let filter = source
            .filter_by_name(&filter)
            .ok_or_else(precondition_fn!("`{filter}` doesn't exist"))?;

        precondition!(
            source.filter_by_name(&new_name).is_none(),
            "another filter with the name `{new_name}` already exists"
        );

        filter.into_source().set_name(&new_name);

        Ok(Response::new(SetNameResponse {}))
    }

    async fn set_index(
        &self,
        request: Request<SetIndexRequest>,
    ) -> Result<Response<SetIndexResponse>, Status> {
        let SetIndexRequest { identifier, index } = request.into_inner();
        let Identifier { source, filter } =
            identifier.ok_or_else(precondition_fn!("identifier must be specified"))?;

        let source =
            Source::by_name(&source).ok_or_else(precondition_fn!("`{source}` doesn't exist"))?;
        let mut filter = source
            .filter_by_name(&filter)
            .ok_or_else(precondition_fn!("`{filter}` doesn't exist"))?;

        let mut current_index = filter.index().unwrap_or_default() as u32;
        let direction = if current_index < index {
            OrderMovement::Up
        } else {
            OrderMovement::Down
        };

        while current_index != index {
            filter.set_order(direction);

            if direction == OrderMovement::Down {
                current_index += 1;
            } else {
                current_index -= 1;
            }
        }

        Ok(Response::new(SetIndexResponse {}))
    }

    async fn set_enabled(
        &self,
        request: Request<SetEnabledRequest>,
    ) -> Result<Response<SetEnabledResponse>, Status> {
        let SetEnabledRequest {
            identifier,
            enabled,
        } = request.into_inner();
        let Identifier { source, filter } =
            identifier.ok_or_else(precondition_fn!("identifier must be specified"))?;

        let source =
            Source::by_name(&source).ok_or_else(precondition_fn!("`{source}` doesn't exist"))?;
        let filter = source
            .filter_by_name(&filter)
            .ok_or_else(precondition_fn!("`{filter}` doesn't exist"))?;

        filter.into_source().set_enabled(enabled);

        Ok(Response::new(SetEnabledResponse {}))
    }

    async fn set_settings(
        &self,
        request: Request<SetSettingsRequest>,
    ) -> Result<Response<SetSettingsResponse>, Status> {
        let SetSettingsRequest {
            identifier,
            settings,
            overlay,
        } = request.into_inner();
        let Identifier { source, filter } =
            identifier.ok_or_else(precondition_fn!("identifier must be specified"))?;
        let overlay = overlay.unwrap_or(true);

        let source =
            Source::by_name(&source).ok_or_else(precondition_fn!("`{source}` doesn't exist"))?;
        let filter = source
            .filter_by_name(&filter)
            .ok_or_else(precondition_fn!("`{filter}` doesn't exist"))?
            .into_source();
        let settings = Data::from_json(&settings)
            .map_err(|e| Status::invalid_argument(format!("invalid JSON settings: {e:?}")))?;

        if overlay {
            filter.update(settings);
        } else {
            filter.reset_settings(settings);
        }

        filter.update_properties();

        Ok(Response::new(SetSettingsResponse {}))
    }
}
