use obs::video::VideoInfo;
use serde_json::{Map, Value};
use tonic::{Request, Response, Status};

pub use self::config_service_server::ConfigServiceServer;
use crate::util;

tonic::include_proto!("config.v1");

pub struct ConfigService;

#[tonic::async_trait]
impl config_service_server::ConfigService for ConfigService {
    async fn global_persistent_data(
        &self,
        request: Request<GlobalPersistentDataRequest>,
    ) -> Result<Response<GlobalPersistentDataResponse>, Status> {
        let GlobalPersistentDataRequest { name } = request.into_inner();
        let profile_path = obs::frontend::profiles::current_path();
        let data_path = profile_path.join("../../../obsWebSocketPersistentData.json");

        let result = async {
            let buf = tokio::fs::read(data_path).await?;
            let mut data = serde_json::from_slice::<Map<String, Value>>(&buf)?;

            anyhow::Ok(data.remove(&name))
        };

        match result.await {
            Ok(v) => Ok(Response::new(GlobalPersistentDataResponse {
                value: v.map(util::json_to_proto),
            })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn set_global_persistent_data(
        &self,
        request: Request<SetGlobalPersistentDataRequest>,
    ) -> Result<Response<SetGlobalPersistentDataResponse>, Status> {
        let SetGlobalPersistentDataRequest { name, value } = request.into_inner();
        let value = value.and_then(util::proto_to_json);
        let profile_path = obs::frontend::profiles::current_path();
        let data_path = profile_path.join("../../../obsWebSocketPersistentData.json");

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
            Ok(()) => Ok(Response::new(SetGlobalPersistentDataResponse {})),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }

    async fn video_settings(
        &self,
        request: Request<VideoSettingsRequest>,
    ) -> Result<Response<VideoSettingsResponse>, Status> {
        let VideoSettingsRequest {} = request.into_inner();
        let info = VideoInfo::get().ok_or_else(|| Status::not_found("no video data available"))?;

        Ok(Response::new(VideoSettingsResponse {
            settings: Some(Settings {
                fps: Some(settings::Fps {
                    numerator: *info.fps.numer(),
                    denominator: *info.fps.denom(),
                }),
                base: Some(settings::Size {
                    width: info.base_size.0,
                    height: info.base_size.1,
                }),
                output: Some(settings::Size {
                    width: info.output_size.0,
                    height: info.output_size.1,
                }),
            }),
        }))
    }

    async fn set_video_settings(
        &self,
        request: Request<SetVideoSettingsRequest>,
    ) -> Result<Response<SetVideoSettingsResponse>, Status> {
        let SetVideoSettingsRequest { settings } = request.into_inner();
        let Settings { fps, base, output } =
            settings.ok_or_else(|| Status::invalid_argument("settings must be set"))?;

        if fps.is_none() && base.is_none() && output.is_none() {
            return Err(Status::invalid_argument("must specify at least one change"));
        }

        let config = obs::frontend::profile_config();

        if let Some(fps) = fps {
            config.set_uint("Video", "FPSType", 2);
            config.set_uint("Video", "FPSNum", fps.numerator.into());
            config.set_uint("Video", "FPSDen", fps.denominator.into());
        }

        if let Some(base) = base {
            config.set_uint("Video", "BaseCX", base.width.into());
            config.set_uint("Video", "BaseCY", base.height.into());
        }

        if let Some(output) = output {
            config.set_uint("Video", "OutputCX", output.width.into());
            config.set_uint("Video", "OutputCY", output.height.into());
        }

        config
            .save_safe("tmp", None)
            .map_err(|e| Status::internal(e.to_string()))?;
        obs::frontend::reset_video();

        Ok(Response::new(SetVideoSettingsResponse {}))
    }
}
