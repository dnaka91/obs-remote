use obs::libobs_sys::{LIBOBS_API_MAJOR_VER, LIBOBS_API_MINOR_VER, LIBOBS_API_PATCH_VER};
use tonic::{Request, Response, Status};

pub use self::general_service_server::GeneralServiceServer;

tonic::include_proto!("general.v1");

pub struct GeneralService;

#[tonic::async_trait]
impl general_service_server::GeneralService for GeneralService {
    async fn version(
        &self,
        request: Request<VersionRequest>,
    ) -> Result<Response<VersionResponse>, Status> {
        use self::version_response::SemVer;

        let VersionRequest {} = request.into_inner();

        let version = obs::obs_version();

        Ok(Response::new(VersionResponse {
            obs_compile_version: Some(SemVer {
                major: LIBOBS_API_MAJOR_VER,
                minor: LIBOBS_API_MINOR_VER,
                patch: LIBOBS_API_PATCH_VER,
            }),
            obs_runtime_version: Some(SemVer {
                major: version >> 24 & 0xff,
                minor: version >> 16 & 0xff,
                patch: version & 0xffff,
            }),
            obs_remote_version: Some(SemVer {
                major: env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap(),
                minor: env!("CARGO_PKG_VERSION_MINOR").parse().unwrap(),
                patch: env!("CARGO_PKG_VERSION_PATCH").parse().unwrap(),
            }),
        }))
    }

    async fn stats(
        &self,
        request: Request<StatsRequest>,
    ) -> Result<Response<StatsResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn broadcast_event(
        &self,
        request: Request<BroadcastEventRequest>,
    ) -> Result<Response<BroadcastEventResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn call_vendor(
        &self,
        request: Request<CallVendorRequest>,
    ) -> Result<Response<CallVendorResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn sleep(
        &self,
        request: Request<SleepRequest>,
    ) -> Result<Response<SleepResponse>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }
}
