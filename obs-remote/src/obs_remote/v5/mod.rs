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
    config::{ConfigServer, ConfigService},
    events::{EventsServer, EventsService},
    filters::{FiltersServer, FiltersService},
    general::{GeneralServer, GeneralService},
    hotkeys::{HotkeysServer, HotkeysService},
    inputs::{InputsServer, InputsService},
    media_inputs::{MediaInputsServer, MediaInputsService},
    outputs::{OutputsServer, OutputsService},
    profiles::{ProfilesServer, ProfilesService},
    projectors::{ProjectorsServer, ProjectorsService},
    recording::{RecordingServer, RecordingService},
    replay_buffer::{ReplayBufferServer, ReplayBufferService},
    scene_collections::{SceneCollectionsServer, SceneCollectionsService},
    scene_items::{SceneItemsServer, SceneItemsService},
    scenes::{ScenesServer, ScenesService},
    sources::{SourcesServer, SourcesService},
    streaming::{StreamingServer, StreamingService},
    transitions::{TransitionsServer, TransitionsService},
    virtual_cam::{VirtualCamServer, VirtualCamService},
};

mod config;
mod events;
mod filters;
mod general;
mod hotkeys;
mod inputs;
mod media_inputs;
mod outputs;
mod profiles;
mod projectors;
mod recording;
mod replay_buffer;
mod scene_collections;
mod scene_items;
mod scenes;
mod sources;
mod streaming;
mod transitions;
mod virtual_cam;
