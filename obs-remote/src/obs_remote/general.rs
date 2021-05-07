use log::info;
use tonic::{Request, Response, Status};

use self::general_server::General;

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
                major: obs::libobs_sys::LIBOBS_API_MAJOR_VER,
                minor: obs::libobs_sys::LIBOBS_API_MINOR_VER,
                patch: obs::libobs_sys::LIBOBS_API_PATCH_VER,
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

        let res = || {
            let config = obs::frontend::profile_config();
            config.set_string("Output", "FilenameFormatting", &filename_formatting);
            config.save()?;
            Ok(Response::new(()))
        };

        res().map_err(|e: anyhow::Error| Status::internal(e.to_string()))
    }

    async fn get_filename_formatting(
        &self,
        request: Request<()>,
    ) -> Result<Response<GetFilenameFormattingReply>, Status> {
        let res = || {
            Ok(Response::new(GetFilenameFormattingReply {
                filename_formatting: obs::frontend::profile_config()
                    .string("Output", "FilenameFormatting")
                    .unwrap_or_default(),
            }))
        };

        res().map_err(|e: anyhow::Error| Status::internal(e.to_string()))
    }

    #[allow(clippy::cast_precision_loss)]
    async fn get_stats(&self, request: Request<()>) -> Result<Response<StatsReply>, Status> {
        use obs::video::Video;
        use stats_reply::ObsStats;

        let main_video = Video::get();
        let path = {
            let config = obs::frontend::profile_config();
            let path = if config
                .string("Output", "Mode")
                .as_deref()
                .unwrap_or_default()
                == "Advanced"
            {
                config.string("AdvOut", "RecFilePath")
            } else {
                config.string("SimpleOutput", "FilePath")
            };

            path.unwrap_or_default()
        };

        Ok(Response::new(StatsReply {
            stats: Some(ObsStats {
                fps: obs::active_fps(),
                render_total_frames: obs::total_frames(),
                render_missed_frames: obs::lagged_frames(),
                output_total_frames: main_video.total_frames(),
                output_skipped_frames: main_video.skipped_frames(),
                average_frame_time: obs::average_frame_time_ns() as f64 / 1_000_000.0,
                cpu_usage: obs::os::cpu_usage(),
                memory_usage: obs::os::memory_usage() as f64 / 1024.0 / 1024.0,
                free_disk_space: obs::os::free_disk_space(&path) as f64 / 1024.0 / 1024.0,
            }),
        }))
    }

    async fn get_video_info(
        &self,
        request: Request<()>,
    ) -> Result<Response<VideoInfoReply>, Status> {
        use obs::video::{Colorspace, Format, RangeType, ScaleType, VideoInfo};

        let video = match VideoInfo::get() {
            Some(video) => video,
            None => return Err(Status::unavailable("currently no video active")),
        };

        Ok(Response::new(VideoInfoReply {
            base_width: video.base_size.0,
            base_height: video.base_size.1,
            output_width: video.output_size.0,
            output_height: video.output_size.1,
            fps: video.fps,
            video_format: match video.output_format {
                Format::Nv12 => video_info_reply::VideoFormat::Nv12,
                Format::I420 => video_info_reply::VideoFormat::I420,
                Format::I444 => video_info_reply::VideoFormat::I444,
                Format::Rgba => video_info_reply::VideoFormat::Rgba,
                _ => {
                    return Err(Status::internal(format!(
                        "unsupported video format `{:?}`",
                        video.output_format
                    )))
                }
            } as i32,
            color_space: match video.colorspace {
                Colorspace::Default | Colorspace::Cs709 => video_info_reply::ColorSpace::Cs709,
                Colorspace::Cs601 => video_info_reply::ColorSpace::Cs601,
                Colorspace::Srgb => video_info_reply::ColorSpace::CsSrgb,
            } as i32,
            color_range: match video.range {
                RangeType::Default | RangeType::Partial => video_info_reply::ColorRange::Partial,
                RangeType::Full => video_info_reply::ColorRange::Full,
            } as i32,
            scale_type: match video.scale_type {
                ScaleType::Bilinear => video_info_reply::ScaleType::Bilinear,
                ScaleType::Area => video_info_reply::ScaleType::Area,
                ScaleType::Bicubic => video_info_reply::ScaleType::Bicubic,
                ScaleType::Lanczos => video_info_reply::ScaleType::Lanczos,
                ScaleType::Disable | ScaleType::Point => {
                    return Err(Status::internal(format!(
                        "unsupported scale type `{:?}`",
                        video.scale_type
                    )))
                }
            } as i32,
        }))
    }

    #[allow(clippy::cast_possible_truncation)]
    async fn open_projector(
        &self,
        request: Request<OpenProjectorRequest>,
    ) -> Result<Response<()>, Status> {
        use self::open_projector_request::ProjectorType;

        info!(
            "General.OpenProjector request from {:?}",
            request.remote_addr()
        );

        let request = request.into_inner();

        obs::frontend::open_projector(
            match request.r#type() {
                ProjectorType::Default => "Default",
                ProjectorType::Preview => "Preview",
                ProjectorType::Source => "Source",
                ProjectorType::Scene => "Scene",
                ProjectorType::StudioProgram => "StudioProgram",
                ProjectorType::Multiview => "Multiview",
            },
            request.monitor.unwrap_or(-1) as i32,
            request.geometry.as_deref().unwrap_or_default(),
            request.name.as_deref().unwrap_or_default(),
        );

        Ok(Response::new(()))
    }

    async fn trigger_hotkey_by_name(
        &self,
        request: Request<TriggerHotkeyByNameRequest>,
    ) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn trigger_hotkey_by_sequence(
        &self,
        request: Request<TriggerHotkeyBySequenceRequest>,
    ) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }
}
