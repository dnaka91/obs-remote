use tonic::{Request, Response, Status};

use self::sources_server::Sources;

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
        Err(Status::unimplemented("not implemented!"))
    }

    async fn get_volume(
        &self,
        request: Request<GetVolumeRequest>,
    ) -> Result<Response<GetVolumeReply>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn set_volume(&self, request: Request<SetVolumeRequest>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn get_mute(
        &self,
        request: Request<GetMuteRequest>,
    ) -> Result<Response<GetMuteReply>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn set_mute(&self, request: Request<SetMuteRequest>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn toggle_mute(
        &self,
        request: Request<ToggleMuteRequest>,
    ) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn get_audio_active(
        &self,
        request: Request<GetAudioActiveRequest>,
    ) -> Result<Response<GetAudioActiveReply>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn set_name(&self, request: Request<SetNameRequest>) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn set_sync_offset(
        &self,
        request: Request<SetSyncOffsetRequest>,
    ) -> Result<Response<()>, Status> {
        Err(Status::unimplemented("not implemented!"))
    }

    async fn get_sync_offset(
        &self,
        request: Request<GetSyncOffsetRequest>,
    ) -> Result<Response<GetSyncOffsetReply>, Status> {
        Err(Status::unimplemented("not implemented!"))
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
        Err(Status::unimplemented("not implemented!"))
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
