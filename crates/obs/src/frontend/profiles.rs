use std::{os::raw::c_char, path::PathBuf};

use super::tasks::{self, TaskType};
use crate::{
    config::Config,
    util::{self, FfiToString, StringToFfi},
};

/// Get the currently active profile name.
pub fn current() -> String {
    let raw = unsafe { libobs_sys::obs_frontend_get_current_profile() as *const c_char };
    let value = raw.into_string();

    unsafe { libobs_sys::bfree(raw as *mut _) };

    value
}

/// Get the file path for the current active profile.
pub fn current_path() -> PathBuf {
    let raw = unsafe { libobs_sys::obs_frontend_get_current_profile_path() as *const c_char };
    let value = raw.into_path_buf();

    unsafe { libobs_sys::bfree(raw as *mut _) };

    value
}

/// Activate the given profile.
pub fn set_current(name: &str) {
    tasks::queue(
        TaskType::Ui,
        name,
        |name| {
            let name = name.cstr();
            unsafe { libobs_sys::obs_frontend_set_current_profile(name.as_ptr()) }
        },
        true,
    );
}

/// Create a new profile.
pub fn create_profile(name: &str) {
    tasks::queue(
        TaskType::Ui,
        name,
        |name| {
            let name = name.cstr();
            unsafe { libobs_sys::obs_frontend_create_profile(name.as_ptr()) }
        },
        true,
    );
}

/// Duplicate an existing profile.
pub fn duplicate_profile(name: &str) {
    tasks::queue(
        TaskType::Ui,
        name,
        |name| {
            let name = name.cstr();
            unsafe { libobs_sys::obs_frontend_duplicate_profile(name.as_ptr()) }
        },
        true,
    );
}

/// Delete an existing profile.
pub fn delete_profile(name: &str) {
    tasks::queue(
        TaskType::Ui,
        name,
        |name| {
            let name = name.cstr();
            unsafe { libobs_sys::obs_frontend_delete_profile(name.as_ptr()) }
        },
        true,
    );
}

/// List the available profile names.
pub fn list() -> Vec<String> {
    util::convert_string_list_mut(unsafe { libobs_sys::obs_frontend_get_profiles() })
}

pub fn config() -> Config {
    Config::from_raw(unsafe { libobs_sys::obs_frontend_get_profile_config() })
}
