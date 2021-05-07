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

pub mod common;
pub mod events;
pub mod general;
pub mod media_control;
pub mod outputs;
pub mod profiles;
pub mod recording;
pub mod replay_buffer;
pub mod scene_collections;
pub mod scene_items;
pub mod scenes;
pub mod sources;
pub mod streaming;
pub mod studio_mode;
pub mod transitions;
pub mod v5;
