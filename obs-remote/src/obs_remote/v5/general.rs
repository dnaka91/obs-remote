use obs::frontend;
use tonic::{Request, Response, Status};

pub use self::general_server::GeneralServer;

tonic::include_proto!("obs_remote.v5.general");

pub struct GeneralService;

#[tonic::async_trait]
impl general_server::General for GeneralService {
    async fn version(&self, request: Request<()>) -> Result<Response<VersionReply>, Status> {
        use self::version_reply::SemVer;

        Ok(Response::new(VersionReply {
            obs_version: Some(SemVer {
                major: obs::libobs_sys::LIBOBS_API_MAJOR_VER,
                minor: obs::libobs_sys::LIBOBS_API_PATCH_VER,
                patch: obs::libobs_sys::LIBOBS_API_MINOR_VER,
            }),
            obs_remote_version: Some(SemVer {
                major: env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap(),
                minor: env!("CARGO_PKG_VERSION_MINOR").parse().unwrap(),
                patch: env!("CARGO_PKG_VERSION_PATCH").parse().unwrap(),
            }),
            rpc_version: 1,
        }))
    }

    async fn broadcast_event(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn system_stats(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn is_studio_mode_enabled(&self, request: Request<()>) -> Result<Response<bool>, Status> {
        Ok(Response::new(frontend::preview_mode::active()))
    }

    async fn set_studio_mode_enabled(
        &self,
        request: Request<bool>,
    ) -> Result<Response<()>, Status> {
        let enabled = request.into_inner();
        if frontend::preview_mode::active() != enabled {
            frontend::preview_mode::set(enabled);
        }

        Ok(Response::new(()))
    }

    async fn sleep(&self, request: Request<prost_types::Duration>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }
}
