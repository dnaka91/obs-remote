#![deny(rust_2018_idioms, clippy::all, clippy::pedantic)]
#![allow(
    unused_variables,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss
)]
#![allow(
    clippy::default_trait_access,
    clippy::doc_markdown,
    clippy::enum_variant_names,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
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

#[doc(hidden)]
#[macro_export]
macro_rules! precondition {
    ($cond:expr, $($arg:tt)*) => {
        if !($cond) {
            return Err(Status::failed_precondition(format!($($arg)*)));
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! precondition_fn {
    ($($arg:tt)*) => {
        || Status::failed_precondition(format!($($arg)*))
    };
}

pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("obs_remote_descriptor");
