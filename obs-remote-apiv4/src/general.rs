use log::info;
use tonic::{Request, Response, Status};

use self::general_server::General;
use crate::precondition;

tonic::include_proto!("obs_remote.general");

pub struct Service;

#[tonic::async_trait]
impl General for Service {
    async fn get_version(&self, request: Request<()>) -> Result<Response<VersionReply>, Status> {
        use self::version_reply::SemVer;

        info!(
            "General.GetVersion request from {:?}",
            request.remote_addr()
        );

        Ok(Response::new(VersionReply {
            obs_studio_version: Some(SemVer {
                major: 27,
                minor: 1,
                patch: 3,
            }),
            obs_remote_version: Some(SemVer {
                major: env!("CARGO_PKG_VERSION_MAJOR").parse().unwrap(),
                minor: env!("CARGO_PKG_VERSION_MINOR").parse().unwrap(),
                patch: env!("CARGO_PKG_VERSION_PATCH").parse().unwrap(),
            }),
            supported_image_export_formats: vec![],
        }))
    }

    async fn get_auth_required(
        &self,
        request: Request<()>,
    ) -> Result<Response<AuthRequiredReply>, Status> {
        info!(
            "General.GetAuthRequired request from {:?}",
            request.remote_addr()
        );

        Ok(Response::new(AuthRequiredReply {
            auth_required: false,
            ..Default::default()
        }))
    }

    async fn authenticate(
        &self,
        request: Request<AuthenticateRequest>,
    ) -> Result<Response<()>, Status> {
        info!(
            "General.Authenticate request from {:?}",
            request.remote_addr()
        );

        Ok(Response::new(()))
    }

    async fn set_filename_formatting(
        &self,
        request: Request<SetFilenameFormattingRequest>,
    ) -> Result<Response<()>, Status> {
        let filename_formatting = request.into_inner().filename_formatting;
        if filename_formatting.is_empty() {
            return Err(Status::invalid_argument(
                "filename formatting mustn't be empty",
            ));
        }

        let res = || Ok(Response::new(()));

        res().map_err(|e: anyhow::Error| Status::internal(e.to_string()))
    }

    async fn get_filename_formatting(
        &self,
        request: Request<()>,
    ) -> Result<Response<GetFilenameFormattingReply>, Status> {
        let res = || {
            Ok(Response::new(GetFilenameFormattingReply {
                filename_formatting: "test".to_owned(),
            }))
        };

        res().map_err(|e: anyhow::Error| Status::internal(e.to_string()))
    }

    #[allow(clippy::cast_precision_loss)]
    async fn get_stats(&self, request: Request<()>) -> Result<Response<StatsReply>, Status> {
        use stats_reply::ObsStats;

        Ok(Response::new(StatsReply {
            stats: Some(ObsStats {
                fps: 15.1,
                render_total_frames: 1000,
                render_missed_frames: 5,
                output_total_frames: 800,
                output_skipped_frames: 12,
                average_frame_time: 500.2,
                cpu_usage: 10.3,
                memory_usage: 12.4,
                free_disk_space: 20.5,
            }),
        }))
    }

    async fn get_video_info(
        &self,
        request: Request<()>,
    ) -> Result<Response<VideoInfoReply>, Status> {
        Ok(Response::new(VideoInfoReply {
            base_width: 200,
            base_height: 300,
            output_width: 100,
            output_height: 150,
            fps: 30.0,
            video_format: video_info_reply::VideoFormat::Nv12 as i32,
            color_space: video_info_reply::ColorSpace::S709 as i32,
            color_range: video_info_reply::ColorRange::Partial as i32,
            scale_type: video_info_reply::ScaleType::Bilinear as i32,
        }))
    }

    #[allow(clippy::cast_possible_truncation)]
    async fn open_projector(
        &self,
        request: Request<OpenProjectorRequest>,
    ) -> Result<Response<()>, Status> {
        info!(
            "General.OpenProjector request from {:?}",
            request.remote_addr()
        );

        Ok(Response::new(()))
    }

    async fn trigger_hotkey_by_name(
        &self,
        request: Request<TriggerHotkeyByNameRequest>,
    ) -> Result<Response<()>, Status> {
        let hotkey_name = request.into_inner().hotkey_name;
        precondition!(!hotkey_name.is_empty(), "name mustn't be empty");

        Ok(Response::new(()))
    }

    async fn trigger_hotkey_by_sequence(
        &self,
        request: Request<TriggerHotkeyBySequenceRequest>,
    ) -> Result<Response<()>, Status> {
        Ok(Response::new(()))
    }
}
