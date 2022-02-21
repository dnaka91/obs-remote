use super::tasks::{self, TaskType};
use crate::source::Source;

pub fn active() -> bool {
    unsafe { libobs_sys::obs_frontend_preview_program_mode_active() }
}

pub fn set(enable: bool) {
    fn do_enable() {
        unsafe { libobs_sys::obs_frontend_set_preview_program_mode(true) };
    }
    fn do_disable() {
        unsafe { libobs_sys::obs_frontend_set_preview_program_mode(false) };
    }

    tasks::queue(
        TaskType::Ui,
        (),
        if enable {
            |_| do_enable()
        } else {
            |_| do_disable()
        },
        true,
    );
}

pub fn trigger_transition() {
    unsafe { libobs_sys::obs_frontend_preview_program_trigger_transition() };
}

pub fn current_scene() -> Option<Source<'static>> {
    let raw = unsafe { libobs_sys::obs_frontend_get_current_preview_scene() };
    if raw.is_null() {
        None
    } else {
        Some(Source::from_raw(raw))
    }
}

pub fn set_current_scene(source: &Source<'_>) {
    unsafe { libobs_sys::obs_frontend_set_current_preview_scene(source.as_ptr()) }
}
