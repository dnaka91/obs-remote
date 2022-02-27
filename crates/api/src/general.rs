use log::info;
use tonic::{Request, Response, Status};

pub use self::general_server::GeneralServer;

tonic::include_proto!("obs_remote.general");

pub struct GeneralService;

#[tonic::async_trait]
impl general_server::General for GeneralService {
    async fn version(&self, request: Request<()>) -> Result<Response<VersionReply>, Status> {
        use self::version_reply::SemVer;

        info!("General.Version request from {:?}", request.remote_addr());

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

    async fn stats(&self, request: Request<()>) -> Result<Response<StatsReply>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn broadcast_event(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn call_vendor(&self, request: Request<()>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn sleep(&self, request: Request<SleepRequest>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }
}
