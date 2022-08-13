use std::ptr;

use crate::{config::Config, util::StringToFfi};

pub mod events;
pub mod preview_mode;
pub mod profiles;
pub mod recording;
pub mod replay_buffer;
pub mod scene_collections;
pub mod scenes;
pub mod sources;
pub mod streaming;
pub(crate) mod tasks;
pub mod transitions;
pub mod virtualcam;

pub fn add_tools_menu_item(name: &str) {
    let name = name.cstr();

    unsafe { libobs_sys::obs_frontend_add_tools_menu_item(name.as_ptr(), None, ptr::null_mut()) };
}

pub fn global_config() -> Config {
    let raw = unsafe { libobs_sys::obs_frontend_get_global_config() };
    Config::from_raw(raw)
}

pub fn profile_config() -> Config {
    let raw = unsafe { libobs_sys::obs_frontend_get_profile_config() };
    Config::from_raw(raw)
}

pub fn open_projector(ty: &str, monitor: i32, geometry: &str, name: &str) {
    let ty = ty.cstr();
    let geometry = geometry.cstr();
    let name = name.cstr();

    unsafe {
        libobs_sys::obs_frontend_open_projector(
            ty.as_ptr(),
            monitor,
            geometry.as_ptr(),
            name.as_ptr(),
        )
    };
}

pub fn reset_video() {
    unsafe { libobs_sys::obs_frontend_reset_video() };
}
