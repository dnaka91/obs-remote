use super::tasks::{self, TaskType};
use crate::util::{self, FfiToString, StringToFfi};

pub fn add(name: &str) -> bool {
    let name = name.cstr();
    unsafe { libobs_sys::obs_frontend_add_scene_collection(name.as_ptr()) }
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
        |name| {
            let name = name.cstr();
            unsafe { libobs_sys::obs_frontend_set_current_scene_collection(name.as_ptr()) }
        },
        true,
    );
}

pub fn list() -> Vec<String> {
    util::convert_string_list_mut(unsafe { libobs_sys::obs_frontend_get_scene_collections() })
}
