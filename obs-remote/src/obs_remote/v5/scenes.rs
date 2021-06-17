use tonic::{Request, Response, Status};

pub use self::scenes_server::ScenesServer;

tonic::include_proto!("obs_remote.v5.scenes");

pub struct ScenesService;

#[tonic::async_trait]
impl scenes_server::Scenes for ScenesService {
    async fn list(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn current(&self, request: Request<()>) -> Result<Response<String>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn set_current_program_scene(
        &self,
        request: Request<String>,
    ) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn current_preview_scene(
        &self,
        request: Request<()>,
    ) -> Result<Response<String>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn set_current_preview_scene(
        &self,
        request: Request<String>,
    ) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn set_index(&self, request: Request<u32>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn set_name(&self, request: Request<String>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn create(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn delete(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
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
