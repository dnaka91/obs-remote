use super::{
    convert_string_list,
    tasks::{self, TaskType},
};
use crate::{cstr_ptr, util::StringConversion};

pub fn add(name: &str) -> bool {
    unsafe { libobs_sys::obs_frontend_add_scene_collection(cstr_ptr!(name)) }
}

pub fn current() -> String {
    let raw = unsafe { libobs_sys::obs_frontend_get_current_scene_collection() };
    let value = raw.into_string();

    unsafe { libobs_sys::bfree(raw as *mut _) };

    value
}

pub fn set_current(name: &str) {
    tasks::queue(
        TaskType::Ui,
        name,
        |name| unsafe { libobs_sys::obs_frontend_set_current_scene_collection(cstr_ptr!(name)) },
        true,
    );
}

pub fn list() -> Vec<String> {
    convert_string_list(unsafe { libobs_sys::obs_frontend_get_scene_collections() })
}
