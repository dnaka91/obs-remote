use obs::{
    frontend::{preview_mode, scenes},
    scene::Scene,
    source::{Source, SourceType},
};
use tonic::{Request, Response, Status};

pub use self::scenes_server::ScenesServer;
use crate::precondition;

tonic::include_proto!("obs_remote.scenes");

pub struct ScenesService;

#[tonic::async_trait]
impl scenes_server::Scenes for ScenesService {
    async fn list(&self, request: Request<()>) -> Result<Response<ListReply>, Status> {
        Ok(Response::new(ListReply {
            current: scenes::current().name(),
            current_preview: scenes::current_preview().map(|scene| scene.name()),
            scenes: scenes::list()
                .into_iter()
                .enumerate()
                .map(|(i, scene)| list_reply::Scene {
                    name: scene.name(),
                    index: i as u64,
                    group: scene.is_group(),
                })
                .collect(),
        }))
    }

    async fn current(&self, request: Request<()>) -> Result<Response<String>, Status> {
        Ok(Response::new(scenes::current().name()))
    }

    async fn set_current(&self, request: Request<String>) -> Result<Response<()>, Status> {
        let name = request.into_inner();
        precondition!(!name.is_empty(), "name mustn't be empty");

        let scene = Source::by_name(&name)
            .ok_or_else(|| Status::failed_precondition(format!("`{}` doesn't exist", name)))?;
        precondition!(scene.ty() == SourceType::Scene, "`{}` isn't a scene", name);

        scenes::set_current(&scene);

        Ok(Response::new(()))
    }

    async fn current_preview(&self, request: Request<()>) -> Result<Response<String>, Status> {
        let name = scenes::current_preview()
            .ok_or_else(|| Status::failed_precondition("studio mode isn't active"))?
            .name();

        Ok(Response::new(name))
    }

    async fn set_current_preview(&self, request: Request<String>) -> Result<Response<()>, Status> {
        let name = request.into_inner();
        precondition!(!name.is_empty(), "name mustn't be empty");
        precondition!(preview_mode::active(), "studio mode isn't active");

        let scene = Source::by_name(&name)
            .ok_or_else(|| Status::failed_precondition(format!("`{}` doesn't exist", name)))?;
        precondition!(scene.ty() == SourceType::Scene, "`{}` isn't a scene", name);

        scenes::set_current_preview(&scene);

        Ok(Response::new(()))
    }

    async fn set_index(&self, request: Request<u32>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn set_name(&self, request: Request<SetNameRequest>) -> Result<Response<()>, Status> {
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

        Ok(Response::new(()))
    }

    async fn create(&self, request: Request<String>) -> Result<Response<()>, Status> {
        let name = request.into_inner();
        precondition!(!name.is_empty(), "name mustn't be empty");
        precondition!(
            Source::by_name(&name).is_none(),
            "Source with name `{}` already exists",
            name
        );

        Scene::create(&name);

        Ok(Response::new(()))
    }

    async fn delete(&self, request: Request<String>) -> Result<Response<()>, Status> {
        let name = request.into_inner();
        precondition!(!name.is_empty(), "name mustn't be empty");

        let scene = Source::by_name(&name)
            .ok_or_else(|| Status::failed_precondition(format!("`{}` doesn't exist", name)))?;
        precondition!(scene.ty() == SourceType::Scene, "`{}` isn't a scene", name);

        scene.remove();

        Ok(Response::new(()))
    }

    async fn transition_override(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn create_transition_override(
        &self,
        request: Request<()>,
    ) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn delete_transition_override(
        &self,
        request: Request<()>,
    ) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }
}
