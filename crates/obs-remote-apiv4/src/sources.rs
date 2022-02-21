use std::collections::HashMap;

use obs::{
    source::{self, OutputFlags, Source, Volume},
    Duration,
};
use tonic::{Request, Response, Status};

use self::sources_server::Sources;
use super::common::DurationExt;
use crate::precondition;

tonic::include_proto!("obs_remote.sources");

pub struct Service;

#[tonic::async_trait]
impl Sources for Service {
    async fn get_media_sources_list(
        &self,
        request: Request<()>,
    ) -> Result<Response<GetMediaSourcesListReply>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn create(
        &self,
        request: Request<CreateRequest>,
    ) -> Result<Response<CreateReply>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn get_sources_list(
        &self,
        request: Request<()>,
    ) -> Result<Response<GetSourcesListReply>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn list_types(
        &self,
        request: Request<ListTypesRequest>,
    ) -> Result<Response<ListTypesReply>, Status> {
        use self::list_types_reply::{
            source_type::{self, Capabilities},
            SourceType,
        };

        let type_mapping = source::list_input_types()
            .into_iter()
            .map(|ty| (ty, source_type::SourceType::Input))
            .chain(
                source::list_filter_types()
                    .into_iter()
                    .map(|ty| (ty, source_type::SourceType::Filter)),
            )
            .chain(
                source::list_transition_types()
                    .into_iter()
                    .map(|ty| (ty, source_type::SourceType::Transition)),
            )
            .collect::<HashMap<_, _>>();

        let types = source::list_source_types()
            .into_iter()
            .map(|ty| {
                let mut source_type = SourceType {
                    type_id: Default::default(),
                    display_name: source::display_name(&ty),
                    ty: Default::default(),
                    default_settings: source::defaults(&ty)
                        .map_or_else(|| "{}".to_owned(), |data| data.to_json()),
                    caps: {
                        let flags = source::output_flags(&ty);
                        Some(Capabilities {
                            is_async: flags.contains(OutputFlags::ASYNC),
                            has_video: flags.contains(OutputFlags::VIDEO),
                            has_audio: flags.contains(OutputFlags::AUDIO),
                            can_interact: flags.contains(OutputFlags::INTERACTION),
                            is_composite: flags.contains(OutputFlags::COMPOSITE),
                            do_not_duplicate: flags.contains(OutputFlags::DO_NOT_DUPLICATE),
                            do_not_self_monitor: flags.contains(OutputFlags::DO_NOT_SELF_MONITOR),
                            is_deprecated: flags.contains(OutputFlags::DEPRECATED),
                        })
                    },
                };
                source_type.set_ty(
                    type_mapping
                        .get(&ty)
                        .copied()
                        .unwrap_or(source_type::SourceType::Other),
                );
                source_type.type_id = ty;
                source_type
            })
            .collect();

        Ok(Response::new(ListTypesReply { types }))
    }

    async fn get_volume(
        &self,
        request: Request<GetVolumeRequest>,
    ) -> Result<Response<GetVolumeReply>, Status> {
        let GetVolumeRequest {
            source,
            use_decibel,
        } = request.into_inner();
        precondition!(!source.is_empty(), "source mustn't be empty");

        let source = Source::by_name(&source)
            .ok_or_else(|| Status::failed_precondition(format!("`{}` doesn't exist", source)))?;
        let volume = source.volume();
        let volume = if use_decibel {
            volume.as_db()
        } else {
            volume.as_mul()
        };

        Ok(Response::new(GetVolumeReply {
            name: source.name(),
            volume,
            muted: source.muted(),
        }))
    }

    async fn set_volume(&self, request: Request<SetVolumeRequest>) -> Result<Response<()>, Status> {
        let SetVolumeRequest {
            source,
            volume,
            use_decibel,
        } = request.into_inner();
        precondition!(!source.is_empty(), "source mustn't be empty");
        precondition!(
            (use_decibel && volume <= 26.0) || (!use_decibel && (0.0..=20.0).contains(&volume)),
            "volume is out of range"
        );

        let source = Source::by_name(&source)
            .ok_or_else(|| Status::failed_precondition(format!("`{}` doesn't exist", source)))?;

        source.set_volume(if use_decibel {
            Volume::Db(volume)
        } else {
            Volume::Mul(volume)
        });

        Ok(Response::new(()))
    }

    async fn get_tracks(
        &self,
        request: Request<GetTracksRequest>,
    ) -> Result<Response<GetTracksReply>, Status> {
        let source = request.into_inner().source;
        precondition!(!source.is_empty(), "source mustn't be empty");

        let source = Source::by_name(&source)
            .ok_or_else(|| Status::failed_precondition(format!("`{}` doesn't exist", source)))?;
        let mixers = source.audio_mixers();

        Ok(Response::new(GetTracksReply {
            track_1: mixers[0],
            track_2: mixers[1],
            track_3: mixers[2],
            track_4: mixers[3],
            track_5: mixers[4],
            track_6: mixers[5],
        }))
    }

    async fn set_tracks(&self, request: Request<SetTracksRequest>) -> Result<Response<()>, Status> {
        let SetTracksRequest {
            source,
            track,
            active,
        } = request.into_inner();
        precondition!(!source.is_empty(), "source mustn't be empty");
        precondition!((1..=6).contains(&track), "track must be between 1 and 6");

        let mut source = Source::by_name(&source)
            .ok_or_else(|| Status::failed_precondition(format!("`{}` doesn't exist", source)))?;

        let mut mixers = source.audio_mixers();
        mixers[track as usize] = active;

        source.set_audio_mixers(mixers);

        Ok(Response::new(()))
    }

    async fn get_mute(
        &self,
        request: Request<GetMuteRequest>,
    ) -> Result<Response<GetMuteReply>, Status> {
        let source = request.into_inner().source;
        precondition!(!source.is_empty(), "source mustn't be empty");

        let source = Source::by_name(&source)
            .ok_or_else(|| Status::failed_precondition(format!("`{}` doesn't exist", source)))?;

        Ok(Response::new(GetMuteReply {
            name: source.name(),
            muted: source.muted(),
        }))
    }

    async fn set_mute(&self, request: Request<SetMuteRequest>) -> Result<Response<()>, Status> {
        let SetMuteRequest { source, mute } = request.into_inner();
        precondition!(!source.is_empty(), "source mustn't be empty");

        let source = Source::by_name(&source)
            .ok_or_else(|| Status::failed_precondition(format!("`{}` doesn't exist", source)))?;

        source.set_muted(mute);

        Ok(Response::new(()))
    }

    async fn toggle_mute(
        &self,
        request: Request<ToggleMuteRequest>,
    ) -> Result<Response<()>, Status> {
        let source = request.into_inner().source;
        precondition!(!source.is_empty(), "source mustn't be empty");

        let source = Source::by_name(&source)
            .ok_or_else(|| Status::failed_precondition(format!("`{}` doesn't exist", source)))?;

        source.set_muted(!source.muted());

        Ok(Response::new(()))
    }

    async fn get_source_active(
        &self,
        request: Request<GetSourceActiveRequest>,
    ) -> Result<Response<bool>, Status> {
        let source = request.into_inner().source_name;
        precondition!(!source.is_empty(), "source mustn't be empty");

        let source = Source::by_name(&source)
            .ok_or_else(|| Status::failed_precondition(format!("`{}` doesn't exist", source)))?;

        Ok(Response::new(source.active()))
    }

    async fn get_audio_active(
        &self,
        request: Request<GetAudioActiveRequest>,
    ) -> Result<Response<bool>, Status> {
        let source = request.into_inner().source_name;
        precondition!(!source.is_empty(), "source mustn't be empty");

        let source = Source::by_name(&source)
            .ok_or_else(|| Status::failed_precondition(format!("`{}` doesn't exist", source)))?;

        Ok(Response::new(source.audio_active()))
    }

    async fn set_name(&self, request: Request<SetNameRequest>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn set_sync_offset(
        &self,
        request: Request<SetSyncOffsetRequest>,
    ) -> Result<Response<()>, Status> {
        let SetSyncOffsetRequest { source, offset } = request.into_inner();
        precondition!(!source.is_empty(), "source mustn't be empty");

        let offset =
            offset.ok_or_else(|| Status::failed_precondition("offset must be specified"))?;
        let mut source = Source::by_name(&source)
            .ok_or_else(|| Status::failed_precondition(format!("`{}` doesn't exist", source)))?;

        source.set_sync_offset(Duration::from_proto(offset));

        Ok(Response::new(()))
    }

    async fn get_sync_offset(
        &self,
        request: Request<GetSyncOffsetRequest>,
    ) -> Result<Response<GetSyncOffsetReply>, Status> {
        let source = request.into_inner().source;
        precondition!(!source.is_empty(), "source mustn't be empty");

        let source = Source::by_name(&source)
            .ok_or_else(|| Status::failed_precondition(format!("`{}` doesn't exist", source)))?;

        Ok(Response::new(GetSyncOffsetReply {
            name: source.name(),
            offset: Some(source.sync_offset().into_proto()),
        }))
    }

    async fn get_settings(
        &self,
        request: Request<GetSettingsRequest>,
    ) -> Result<Response<SettingsReply>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn set_settings(
        &self,
        request: Request<SetSettingsRequest>,
    ) -> Result<Response<SettingsReply>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn get_text_gdi_plus_properties(
        &self,
        request: Request<GetTextGdiPlusPropertiesRequest>,
    ) -> Result<Response<GetTextGdiPlusPropertiesReply>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn set_text_gdi_plus_properties(
        &self,
        request: Request<SetTextGdiPlusPropertiesRequest>,
    ) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn get_text_freetype2_properties(
        &self,
        request: Request<GetTextFreetype2PropertiesRequest>,
    ) -> Result<Response<GetTextFreetype2PropertiesReply>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn set_text_freetype2_properties(
        &self,
        request: Request<SetTextFreetype2PropertiesRequest>,
    ) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn get_special_sources(
        &self,
        request: Request<()>,
    ) -> Result<Response<GetSpecialSourcesReply>, Status> {
        fn get_source_name(channel: u32) -> Option<String> {
            Source::by_output_channel(channel).map(|source| source.name())
        }

        Ok(Response::new(GetSpecialSourcesReply {
            desktop_1: get_source_name(1),
            desktop_2: get_source_name(2),
            mic_1: get_source_name(3),
            mic_2: get_source_name(4),
            mic_3: get_source_name(5),
        }))
    }

    async fn get_source_filters(
        &self,
        request: Request<GetSourceFiltersRequest>,
    ) -> Result<Response<GetSourceFiltersReply>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn get_source_filter_info(
        &self,
        request: Request<GetSourceFilterInfoRequest>,
    ) -> Result<Response<GetSourceFilterInfoReply>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn add_filter_to_source(
        &self,
        request: Request<AddFilterToSourceRequest>,
    ) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn remove_filter_from_source(
        &self,
        request: Request<RemoveFilterFromSourceRequest>,
    ) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn reorder_source_filter(
        &self,
        request: Request<ReorderSourceFilterRequest>,
    ) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn move_source_filter(
        &self,
        request: Request<MoveSourceFilterRequest>,
    ) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn set_source_filter_settings(
        &self,
        request: Request<SetSourceFilterSettingsRequest>,
    ) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn set_source_filter_visibility(
        &self,
        request: Request<SetSourceFilterVisibilityRequest>,
    ) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn get_audio_monitor_type(
        &self,
        request: Request<GetAudioMonitorTypeRequest>,
    ) -> Result<Response<GetAudioMonitorTypeReply>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn set_audio_monitor_type(
        &self,
        request: Request<SetAudioMonitorTypeRequest>,
    ) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn get_default_settings(
        &self,
        request: Request<GetDefaultSettingsRequest>,
    ) -> Result<Response<GetDefaultSettingsReply>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn take_source_screenshot(
        &self,
        request: Request<TakeSourceScreenshotRequest>,
    ) -> Result<Response<TakeSourceScreenshotReply>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn refresh_browser_source(
        &self,
        request: Request<RefreshBrowserSourceRequest>,
    ) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }
}
