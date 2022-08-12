use obs::{
    frontend::{preview_mode, scenes},
    scene::Scene,
    source::{Source, SourceType},
};
use tonic::{Request, Response, Status};

pub use self::scenes_service_server::ScenesServiceServer;
use crate::precondition;

tonic::include_proto!("obs_remote.scenes");

pub struct ScenesService;

#[tonic::async_trait]
impl scenes_service_server::ScenesService for ScenesService {
    async fn list(&self, request: Request<ListRequest>) -> Result<Response<ListResponse>, Status> {
        let ListRequest {} = request.into_inner();

        Ok(Response::new(ListResponse {
            current: scenes::current().name(),
            current_preview: scenes::current_preview().map(|scene| scene.name()),
            scenes: scenes::list()
                .into_iter()
                .enumerate()
                .map(|(i, scene)| list_response::Scene {
                    name: scene.name(),
                    index: i as u64,
                    group: scene.is_group(),
                })
                .collect(),
        }))
    }

    async fn current(
        &self,
        request: Request<CurrentRequest>,
    ) -> Result<Response<CurrentResponse>, Status> {
        let CurrentRequest {} = request.into_inner();

        Ok(Response::new(CurrentResponse {
            name: scenes::current().name(),
        }))
    }

    async fn set_current(
        &self,
        request: Request<SetCurrentRequest>,
    ) -> Result<Response<SetCurrentResponse>, Status> {
        let SetCurrentRequest { name } = request.into_inner();
        precondition!(!name.is_empty(), "name mustn't be empty");

        let scene = Source::by_name(&name)
            .ok_or_else(|| Status::failed_precondition(format!("`{}` doesn't exist", name)))?;
        precondition!(scene.ty() == SourceType::Scene, "`{}` isn't a scene", name);

        scenes::set_current(&scene);

        Ok(Response::new(SetCurrentResponse {}))
    }

    async fn current_preview(
        &self,
        request: Request<CurrentPreviewRequest>,
    ) -> Result<Response<CurrentPreviewResponse>, Status> {
        let CurrentPreviewRequest {} = request.into_inner();

        let name = scenes::current_preview()
            .ok_or_else(|| Status::failed_precondition("studio mode isn't active"))?
            .name();

        Ok(Response::new(CurrentPreviewResponse { name }))
    }

    async fn set_current_preview(
        &self,
        request: Request<SetCurrentPreviewRequest>,
    ) -> Result<Response<SetCurrentPreviewResponse>, Status> {
        let SetCurrentPreviewRequest { name } = request.into_inner();
        precondition!(!name.is_empty(), "name mustn't be empty");
        precondition!(preview_mode::active(), "studio mode isn't active");

        let scene = Source::by_name(&name)
            .ok_or_else(|| Status::failed_precondition(format!("`{}` doesn't exist", name)))?;
        precondition!(scene.ty() == SourceType::Scene, "`{}` isn't a scene", name);

        scenes::set_current_preview(&scene);

        Ok(Response::new(SetCurrentPreviewResponse {}))
    }

    async fn set_index(
        &self,
        request: Request<SetIndexRequest>,
    ) -> Result<Response<SetIndexResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn set_name(
        &self,
        request: Request<SetNameRequest>,
    ) -> Result<Response<SetNameResponse>, Status> {
        let SetNameRequest { name, new_name } = request.into_inner();
        precondition!(!name.is_empty(), "name mustn't be empty");
        precondition!(!new_name.is_empty(), "new name mustn't be empty");

        let scene = Source::by_name(&name)
            .ok_or_else(|| Status::failed_precondition(format!("`{}` doesn't exist", name)))?;
        precondition!(scene.ty() == SourceType::Scene, "`{}` isn't a scene", name);

        precondition!(
            Source::by_name(&new_name).is_none(),
            "Source with name `{}` already exists",
            new_name
        );

        scene.set_name(&new_name);

        Ok(Response::new(SetNameResponse {}))
    }

    async fn create(
        &self,
        request: Request<CreateRequest>,
    ) -> Result<Response<CreateResponse>, Status> {
        let CreateRequest { name } = request.into_inner();
        precondition!(!name.is_empty(), "name mustn't be empty");
        precondition!(
            Source::by_name(&name).is_none(),
            "Source with name `{}` already exists",
            name
        );

        Scene::create(&name);

        Ok(Response::new(CreateResponse {}))
    }

    async fn remove(
        &self,
        request: Request<RemoveRequest>,
    ) -> Result<Response<RemoveResponse>, Status> {
        let RemoveRequest { name } = request.into_inner();
        precondition!(!name.is_empty(), "name mustn't be empty");

        let scene = Source::by_name(&name)
            .ok_or_else(|| Status::failed_precondition(format!("`{}` doesn't exist", name)))?;
        precondition!(scene.ty() == SourceType::Scene, "`{}` isn't a scene", name);

        scene.remove();

        Ok(Response::new(RemoveResponse {}))
    }

    async fn transition_override(
        &self,
        request: Request<TransitionOverrideRequest>,
    ) -> Result<Response<TransitionOverrideResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn create_transition_override(
        &self,
        request: Request<CreateTransitionOverrideRequest>,
    ) -> Result<Response<CreateTransitionOverrideResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn delete_transition_override(
        &self,
        request: Request<DeleteTransitionOverrideRequest>,
    ) -> Result<Response<DeleteTransitionOverrideResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }
}
