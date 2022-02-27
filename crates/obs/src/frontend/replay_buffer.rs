use crate::output::Output;

pub fn active() -> bool {
    unsafe { libobs_sys::obs_frontend_replay_buffer_active() }
}

pub fn start() {
    unsafe { libobs_sys::obs_frontend_replay_buffer_start() };
}

pub fn stop() {
    unsafe { libobs_sys::obs_frontend_replay_buffer_stop() };
}

pub fn save() {
    unsafe { libobs_sys::obs_frontend_replay_buffer_save() };
}

pub fn output() -> Option<Output<'static>> {
    let raw = unsafe { libobs_sys::obs_frontend_get_replay_buffer_output() };
    (!raw.is_null()).then(|| Output::from_raw(raw))
}
