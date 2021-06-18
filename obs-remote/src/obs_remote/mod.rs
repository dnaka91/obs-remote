#![allow(
    clippy::default_trait_access,
    clippy::module_name_repetitions,
    clippy::option_if_let_else,
    clippy::similar_names,
    clippy::struct_excessive_bools,
    clippy::too_many_lines,
    clippy::use_self,
    clippy::wildcard_imports
)]

pub use self::{
    events::{events_server::EventsServer, Service as EventsService},
    general::{general_server::GeneralServer, Service as GeneralService},
    media_control::{media_control_server::MediaControlServer, Service as MediaControlService},
    outputs::{outputs_server::OutputsServer, Service as OutputsService},
    profiles::{profiles_server::ProfilesServer, Service as ProfilesService},
    recording::{recording_server::RecordingServer, Service as RecordingService},
    replay_buffer::{replay_buffer_server::ReplayBufferServer, Service as ReplayBufferService},
    scene_collections::{
        scene_collections_server::SceneCollectionsServer, Service as SceneCollectionsService,
    },
    scene_items::{scene_items_server::SceneItemsServer, Service as SceneItemsService},
    scenes::{scenes_server::ScenesServer, Service as ScenesService},
    sources::{sources_server::SourcesServer, Service as SourcesService},
    streaming::{streaming_server::StreamingServer, Service as StreamingService},
    studio_mode::{studio_mode_server::StudioModeServer, Service as StudioModeService},
    transitions::{transitions_server::TransitionsServer, Service as TransitionsService},
    virtual_cam::{virtual_cam_server::VirtualCamServer, Service as VirtualCamService},
};

mod common;
mod events;
mod general;
mod media_control;
mod outputs;
mod profiles;
mod recording;
mod replay_buffer;
mod scene_collections;
mod scene_items;
mod scenes;
mod sources;
mod streaming;
mod studio_mode;
mod transitions;
mod virtual_cam;

pub mod v5;

pub(crate) const FILE_DESCRIPTOR_SET_V4: &[u8] =
    tonic::include_file_descriptor_set!("obs_remote_v4_descriptor");

pub(crate) const FILE_DESCRIPTOR_SET_V5: &[u8] =
    tonic::include_file_descriptor_set!("obs_remote_v5_descriptor");
