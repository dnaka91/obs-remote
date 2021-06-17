use std::os::raw::c_char;

use super::{
    convert_string_list,
    tasks::{self, TaskType},
};
use crate::{config::Config, cstr_ptr, util::StringConversion};

/// Get the currently active profile name.
pub fn current() -> String {
    let raw = unsafe { libobs_sys::obs_frontend_get_current_profile() as *const c_char };
    let value = raw.into_string();

    unsafe { libobs_sys::bfree(raw as *mut _) };

    value
}

/// Activate the given profile.
pub fn set_current(name: &str) {
    tasks::queue(
        TaskType::Ui,
        name,
        |name| unsafe { libobs_sys::obs_frontend_set_current_profile(cstr_ptr!(name)) },
        true,
    );
}

/// List the available profile names.
pub fn list() -> Vec<String> {
    convert_string_list(unsafe { libobs_sys::obs_frontend_get_profiles() })
}

pub fn config() -> Config {
    Config::from_raw(unsafe { libobs_sys::obs_frontend_get_profile_config() })
}
