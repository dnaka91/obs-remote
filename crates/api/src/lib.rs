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
    clippy::derive_partial_eq_without_eq,
    clippy::doc_markdown,
    clippy::enum_variant_names,
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::option_if_let_else,
    clippy::similar_names,
    clippy::struct_excessive_bools,
    clippy::too_many_lines,
    clippy::trivially_copy_pass_by_ref,
    clippy::use_self,
    clippy::wildcard_imports
)]

pub use self::{
    config::{ConfigService, ConfigServiceServer},
    events::{EventsService, EventsServiceServer},
    filters::{FiltersService, FiltersServiceServer},
    general::{GeneralService, GeneralServiceServer},
    hotkeys::{HotkeysService, HotkeysServiceServer},
    inputs::{InputsService, InputsServiceServer},
    media_inputs::{MediaInputsService, MediaInputsServiceServer},
    outputs::{OutputsService, OutputsServiceServer},
    profiles::{ProfilesService, ProfilesServiceServer},
    projectors::{ProjectorsService, ProjectorsServiceServer},
    recording::{RecordingService, RecordingServiceServer},
    replay_buffer::{ReplayBufferService, ReplayBufferServiceServer},
    scene_collections::{SceneCollectionsService, SceneCollectionsServiceServer},
    scene_items::{SceneItemsService, SceneItemsServiceServer},
    scenes::{ScenesService, ScenesServiceServer},
    sources::{SourcesService, SourcesServiceServer},
    streaming::{StreamingService, StreamingServiceServer},
    transitions::{TransitionsService, TransitionsServiceServer},
    virtual_cam::{VirtualCamService, VirtualCamServiceServer},
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
mod util;
mod virtual_cam;

pub const FILE_DESCRIPTOR_SET: &[u8] = tonic::include_file_descriptor_set!("obs_remote_descriptor");
