use log::info;
use tonic::{Request, Response, Status};

pub use self::general_server::GeneralServer;

tonic::include_proto!("obs_remote.v5.general");

pub struct GeneralService;

#[tonic::async_trait]
impl general_server::General for GeneralService {
    async fn version(&self, request: Request<()>) -> Result<Response<VersionReply>, Status> {
        use self::version_reply::SemVer;

        info!("General.Version request from {:?}", request.remote_addr());

        Ok(Response::new(VersionReply {
            obs_version: Some(SemVer {
                major: 27,
                minor: 1,
                patch: 3,
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
        Ok(Response::new(true))
    }

    async fn set_studio_mode_enabled(
        &self,
        request: Request<bool>,
    ) -> Result<Response<()>, Status> {
        Ok(Response::new(()))
    }

    async fn sleep(&self, request: Request<prost_types::Duration>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }
}
