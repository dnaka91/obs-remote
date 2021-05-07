use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

pub use self::{
    config_server::ConfigServer, events_server::EventsServer, filters_server::FiltersServer,
    general_server::GeneralServer, hotkeys_server::HotkeysServer, inputs_server::InputsServer,
    media_inputs_server::MediaInputsServer, outputs_server::OutputsServer,
    profiles_server::ProfilesServer, projectors_server::ProjectorsServer,
    recording_server::RecordingServer, replay_buffer_server::ReplayBufferServer,
    scene_collections_server::SceneCollectionsServer, scene_items_server::SceneItemsServer,
    scenes_server::ScenesServer, sources_server::SourcesServer, streaming_server::StreamingServer,
    transitions_server::TransitionsServer, virtual_cam_server::VirtualCamServer,
};

tonic::include_proto!("obs_remote.v5");

pub struct ConfigService;

#[tonic::async_trait]
impl config_server::Config for ConfigService {
    async fn global_persistent_data(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn set_global_persistent_data(
        &self,
        request: Request<()>,
    ) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn video_settings(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn set_video_settings(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }
}

pub struct EventsService;

#[tonic::async_trait]
impl events_server::Events for EventsService {
    type AllStream = ReceiverStream<Result<Event, Status>>;

    async fn all(&self, request: Request<()>) -> Result<Response<Self::AllStream>, Status> {
        todo!()
    }

    type GeneralStream = ReceiverStream<Result<GeneralEvent, Status>>;

    async fn general(&self, request: Request<()>) -> Result<Response<Self::GeneralStream>, Status> {
        todo!()
    }

    type ConfigStream = ReceiverStream<Result<ConfigEvent, Status>>;

    async fn config(&self, request: Request<()>) -> Result<Response<Self::ConfigStream>, Status> {
        todo!()
    }

    type ScenesStream = ReceiverStream<Result<ScenesEvent, Status>>;

    async fn scenes(&self, request: Request<()>) -> Result<Response<Self::ScenesStream>, Status> {
        todo!()
    }

    type InputsStream = ReceiverStream<Result<InputsEvent, Status>>;

    async fn inputs(&self, request: Request<()>) -> Result<Response<Self::InputsStream>, Status> {
        todo!()
    }

    type TransitionsStream = ReceiverStream<Result<TransitionsEvent, Status>>;

    async fn transitions(
        &self,
        request: Request<()>,
    ) -> Result<Response<Self::TransitionsStream>, Status> {
        todo!()
    }

    type FiltersStream = ReceiverStream<Result<FiltersEvent, Status>>;

    async fn filters(&self, request: Request<()>) -> Result<Response<Self::FiltersStream>, Status> {
        todo!()
    }

    type OutputsStream = ReceiverStream<Result<OutputsEvent, Status>>;

    async fn outputs(&self, request: Request<()>) -> Result<Response<Self::OutputsStream>, Status> {
        todo!()
    }

    type SceneItemsStream = ReceiverStream<Result<SceneItemsEvent, Status>>;

    async fn scene_items(
        &self,
        request: Request<()>,
    ) -> Result<Response<Self::SceneItemsStream>, Status> {
        todo!()
    }

    type MediaInputsStream = ReceiverStream<Result<MediaInputsEvent, Status>>;

    async fn media_inputs(
        &self,
        request: Request<()>,
    ) -> Result<Response<Self::MediaInputsStream>, Status> {
        todo!()
    }

    type HighVolumeStream = ReceiverStream<Result<HighVolumeEvent, Status>>;

    async fn high_volume(
        &self,
        request: Request<()>,
    ) -> Result<Response<Self::HighVolumeStream>, Status> {
        todo!()
    }
}

pub struct FiltersService;

#[tonic::async_trait]
impl filters_server::Filters for FiltersService {
    async fn list(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn default_settings(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn get(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn set_index(&self, request: Request<u32>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn set_settings(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn set_enabled(&self, request: Request<bool>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn create(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn delete(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }
}

pub struct GeneralService;

#[tonic::async_trait]
impl general_server::General for GeneralService {
    async fn version(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn broadcast_event(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn system_stats(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn is_studio_mode_enabled(&self, request: Request<()>) -> Result<Response<bool>, Status> {
        todo!()
    }

    async fn set_studio_mode_enabled(
        &self,
        request: Request<bool>,
    ) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn sleep(&self, request: Request<prost_types::Duration>) -> Result<Response<()>, Status> {
        todo!()
    }
}

pub struct HotkeysService;

#[tonic::async_trait]
impl hotkeys_server::Hotkeys for HotkeysService {
    async fn list(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn trigger_by_name(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn trigger_by_sequence(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }
}

pub struct InputsService;

#[tonic::async_trait]
impl inputs_server::Inputs for InputsService {
    async fn list(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn list_kinds(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn list_special(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn default_settings(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn settings(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn set_settings(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn mode(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn set_mode(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn toggle_mode(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn volume(&self, request: Request<()>) -> Result<Response<f32>, Status> {
        todo!()
    }

    async fn set_volume(&self, request: Request<f32>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn audio_sync_offset(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn set_audio_sync_offset(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn tracks(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn set_tracks(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn monitor_mode(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn set_monitor_mode(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn is_active(&self, request: Request<()>) -> Result<Response<bool>, Status> {
        todo!()
    }

    async fn properties_list_property_items(
        &self,
        request: Request<()>,
    ) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn press_properties_button(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn set_name(&self, request: Request<String>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn create(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn delete(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }
}

pub struct MediaInputsService;

#[tonic::async_trait]
impl media_inputs_server::MediaInputs for MediaInputsService {
    async fn status(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn offset_timecode(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn set_timecode(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn is_paused(&self, request: Request<()>) -> Result<Response<bool>, Status> {
        todo!()
    }

    async fn stop(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn restart(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn play_next(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn play_previous(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }
}

pub struct OutputsService;

#[tonic::async_trait]
impl outputs_server::Outputs for OutputsService {
    async fn list(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn status(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn toggle(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn start(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn stop(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn settings(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn set_settings(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }
}

pub struct ProfilesService;

#[tonic::async_trait]
impl profiles_server::Profiles for ProfilesService {
    async fn list(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn current(&self, request: Request<()>) -> Result<Response<String>, Status> {
        todo!()
    }

    async fn set_current(&self, request: Request<String>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn parameter(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn set_parameter(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn persistent_data(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn set_persistent_data(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn create(&self, request: Request<String>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn delete(&self, request: Request<String>) -> Result<Response<()>, Status> {
        todo!()
    }
}

pub struct ProjectorsService;

#[tonic::async_trait]
impl projectors_server::Projectors for ProjectorsService {
    async fn list(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn open(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn close(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }
}

pub struct RecordingService;

#[tonic::async_trait]
impl recording_server::Recording for RecordingService {
    async fn status(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn toggle(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn start(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn stop(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn toggle_pause(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn pause(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn resume(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn directory(&self, request: Request<()>) -> Result<Response<String>, Status> {
        todo!()
    }

    async fn set_directory(&self, request: Request<String>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn filename_formatting(&self, request: Request<()>) -> Result<Response<String>, Status> {
        todo!()
    }

    async fn set_filename_formatting(
        &self,
        request: Request<()>,
    ) -> Result<Response<String>, Status> {
        todo!()
    }
}

pub struct ReplayBufferService;

#[tonic::async_trait]
impl replay_buffer_server::ReplayBuffer for ReplayBufferService {
    async fn status(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn toggle(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn start(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn stop(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn save(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn time(&self, request: Request<()>) -> Result<Response<prost_types::Timestamp>, Status> {
        todo!()
    }

    async fn set_time(
        &self,
        request: Request<prost_types::Timestamp>,
    ) -> Result<Response<()>, Status> {
        todo!()
    }
}

pub struct SceneCollectionsService;

#[tonic::async_trait]
impl scene_collections_server::SceneCollections for SceneCollectionsService {
    async fn list(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn current(&self, request: Request<()>) -> Result<Response<String>, Status> {
        todo!()
    }

    async fn set_current(&self, request: Request<String>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn create(&self, request: Request<String>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn delete(&self, request: Request<String>) -> Result<Response<()>, Status> {
        todo!()
    }
}

pub struct SceneItemsService;

#[tonic::async_trait]
impl scene_items_server::SceneItems for SceneItemsService {
    async fn list(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn list_group(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn transform(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn set_transform(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn is_enabled(&self, request: Request<()>) -> Result<Response<bool>, Status> {
        todo!()
    }

    async fn set_enabled(&self, request: Request<bool>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn is_locked(&self, request: Request<()>) -> Result<Response<bool>, Status> {
        todo!()
    }

    async fn set_locked(&self, request: Request<bool>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn color(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn set_color(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn set_index(&self, request: Request<u32>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn create(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn delete(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn duplicate(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }
}

pub struct ScenesService;

#[tonic::async_trait]
impl scenes_server::Scenes for ScenesService {
    async fn list(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn current(&self, request: Request<()>) -> Result<Response<String>, Status> {
        todo!()
    }

    async fn set_current_program_scene(
        &self,
        request: Request<String>,
    ) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn current_preview_scene(
        &self,
        request: Request<()>,
    ) -> Result<Response<String>, Status> {
        todo!()
    }

    async fn set_current_preview_scene(
        &self,
        request: Request<String>,
    ) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn set_index(&self, request: Request<u32>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn set_name(&self, request: Request<String>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn create(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn delete(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn transition_override(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn create_transition_override(
        &self,
        request: Request<()>,
    ) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn delete_transition_override(
        &self,
        request: Request<()>,
    ) -> Result<Response<()>, Status> {
        todo!()
    }
}

pub struct SourcesService;

#[tonic::async_trait]
impl sources_server::Sources for SourcesService {
    async fn list(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn is_active(&self, request: Request<()>) -> Result<Response<bool>, Status> {
        todo!()
    }

    async fn screenshot(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn save_screenshot(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }
}

pub struct StreamingService;

#[tonic::async_trait]
impl streaming_server::Streaming for StreamingService {
    async fn status(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn toggle(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn start(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn stop(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn send_captions(&self, request: Request<String>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn bitrate(&self, request: Request<()>) -> Result<Response<u32>, Status> {
        todo!()
    }

    async fn set_bitrate(&self, request: Request<u32>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn settings(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn set_settings(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }
}

pub struct TransitionsService;

#[tonic::async_trait]
impl transitions_server::Transitions for TransitionsService {
    async fn list(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn current(&self, request: Request<()>) -> Result<Response<String>, Status> {
        todo!()
    }

    async fn set_current(&self, request: Request<String>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn set_current_duration(
        &self,
        request: Request<prost_types::Duration>,
    ) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn settings(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn set_setttings(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn release_t_bar(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn set_t_bar_position(&self, request: Request<f32>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn trigger_studio_mode_transition(
        &self,
        request: Request<()>,
    ) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn create(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn delete(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }
}

pub struct VirtualCamService;

#[tonic::async_trait]
impl virtual_cam_server::VirtualCam for VirtualCamService {
    async fn status(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn toggle(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn start(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }

    async fn stop(&self, request: Request<()>) -> Result<Response<()>, Status> {
        todo!()
    }
}
