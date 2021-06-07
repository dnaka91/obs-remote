use std::{os::raw::c_char, ptr};

use crate::{config::Config, cstr_ptr, source::Source, util::StringConversion};

pub mod events;
pub mod preview_mode;
pub mod recording;
pub mod replay_buffer;
pub mod sources;
pub mod streaming;
pub(crate) mod tasks;
pub mod transitions;
pub mod virtualcam;

pub fn add_scene_collection(name: &str) -> bool {
    unsafe { libobs_sys::obs_frontend_add_scene_collection(cstr_ptr!(name)) }
}

pub fn add_tools_menu_item(name: &str) {
    unsafe { libobs_sys::obs_frontend_add_tools_menu_item(cstr_ptr!(name), None, ptr::null_mut()) };
}

pub fn current_profile() -> String {
    unsafe { libobs_sys::obs_frontend_get_current_profile() as *const c_char }.into_string()
}

pub fn current_scene() -> Source {
    let raw = unsafe { libobs_sys::obs_frontend_get_current_scene() };
    Source::from_raw(raw)
}

pub fn current_scene_collection() -> String {
    let raw = unsafe { libobs_sys::obs_frontend_get_current_scene_collection() };
    let value = raw.into_string();

    unsafe { libobs_sys::bfree(raw as *mut _) };

    value
}

pub fn set_current_scene_collection(name: &str) {
    unsafe { libobs_sys::obs_frontend_set_current_scene_collection(cstr_ptr!(name)) };
}

pub fn global_config() -> Config {
    let raw = unsafe { libobs_sys::obs_frontend_get_global_config() };
    Config::from_raw(raw)
}

pub fn profile_config() -> Config {
    let raw = unsafe { libobs_sys::obs_frontend_get_profile_config() };
    Config::from_raw(raw)
}

pub fn profiles() -> Vec<String> {
    convert_string_list(unsafe { libobs_sys::obs_frontend_get_profiles() })
}

pub fn scene_collections() -> Vec<String> {
    convert_string_list(unsafe { libobs_sys::obs_frontend_get_scene_collections() })
}

pub fn scene_names() -> Vec<String> {
    convert_string_list(unsafe { libobs_sys::obs_frontend_get_scene_names() })
}

// TODO: obs_frontend_get_scenes

pub fn open_projector(ty: &str, monitor: i32, geometry: &str, name: &str) {
    unsafe {
        libobs_sys::obs_frontend_open_projector(
            cstr_ptr!(ty),
            monitor,
            cstr_ptr!(geometry),
            cstr_ptr!(name),
        )
    };
}

pub fn reset_video() {
    unsafe { libobs_sys::obs_frontend_reset_video() };
}

fn convert_string_list(raw: *mut *mut c_char) -> Vec<String> {
    if raw.is_null() {
        return Vec::new();
    }

    let mut index = 0;
    let mut profiles = Vec::new();

    loop {
        let value = unsafe { *raw.add(index) };
        if value.is_null() {
            unsafe { libobs_sys::bfree(raw as *mut _) };
            break profiles;
        }

        profiles.push((value as *const c_char).into_string());
        index += 1;
    }
}
