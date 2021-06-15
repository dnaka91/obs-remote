use std::os::raw::c_char;

use crate::{cstr_ptr, util::StringConversion};

use super::convert_string_list;

pub fn current() -> String {
    unsafe { libobs_sys::obs_frontend_get_current_profile() as *const c_char }.into_string()
}

pub fn set_current(name: &str) {
    unsafe { libobs_sys::obs_frontend_set_current_profile(cstr_ptr!(name)) };
}

pub fn list() -> Vec<String> {
    convert_string_list(unsafe { libobs_sys::obs_frontend_get_profiles() })
}
