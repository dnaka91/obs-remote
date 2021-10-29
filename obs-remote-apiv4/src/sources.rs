use tonic::{Request, Response, Status};

use self::sources_server::Sources;
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
        Ok(Response::new(ListTypesReply { types: vec![] }))
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

        Ok(Response::new(GetVolumeReply {
            name: source,
            volume: 15.0,
            muted: false,
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

        Ok(Response::new(()))
    }

    async fn get_tracks(
        &self,
        request: Request<GetTracksRequest>,
    ) -> Result<Response<GetTracksReply>, Status> {
        let source = request.into_inner().source;
        precondition!(!source.is_empty(), "source mustn't be empty");

        Ok(Response::new(GetTracksReply {
            track_1: true,
            track_2: true,
            track_3: true,
            track_4: false,
            track_5: false,
            track_6: false,
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

        Ok(Response::new(()))
    }

    async fn get_mute(
        &self,
        request: Request<GetMuteRequest>,
    ) -> Result<Response<GetMuteReply>, Status> {
        let source = request.into_inner().source;
        precondition!(!source.is_empty(), "source mustn't be empty");

        Ok(Response::new(GetMuteReply {
            name: source,
            muted: false,
        }))
    }

    async fn set_mute(&self, request: Request<SetMuteRequest>) -> Result<Response<()>, Status> {
        let SetMuteRequest { source, mute } = request.into_inner();
        precondition!(!source.is_empty(), "source mustn't be empty");

        Ok(Response::new(()))
    }

    async fn toggle_mute(
        &self,
        request: Request<ToggleMuteRequest>,
    ) -> Result<Response<()>, Status> {
        let source = request.into_inner().source;
        precondition!(!source.is_empty(), "source mustn't be empty");

        Ok(Response::new(()))
    }

    async fn get_source_active(
        &self,
        request: Request<GetSourceActiveRequest>,
    ) -> Result<Response<bool>, Status> {
        let source = request.into_inner().source_name;
        precondition!(!source.is_empty(), "source mustn't be empty");

        Ok(Response::new(true))
    }

    async fn get_audio_active(
        &self,
        request: Request<GetAudioActiveRequest>,
    ) -> Result<Response<bool>, Status> {
        let source = request.into_inner().source_name;
        precondition!(!source.is_empty(), "source mustn't be empty");

        Ok(Response::new(true))
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

        Ok(Response::new(()))
    }

    async fn get_sync_offset(
        &self,
        request: Request<GetSyncOffsetRequest>,
    ) -> Result<Response<GetSyncOffsetReply>, Status> {
        let source = request.into_inner().source;
        precondition!(!source.is_empty(), "source mustn't be empty");

        Ok(Response::new(GetSyncOffsetReply {
            name: source,
            offset: None,
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
        Ok(Response::new(GetSpecialSourcesReply {
            desktop_1: None,
            desktop_2: None,
            mic_1: None,
            mic_2: None,
            mic_3: None,
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
