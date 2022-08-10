use crate::output::Output;

pub fn active() -> bool {
    unsafe { libobs_sys::obs_frontend_recording_active() }
}

pub fn paused() -> bool {
    unsafe { libobs_sys::obs_frontend_recording_paused() }
}

pub fn start() {
    unsafe { libobs_sys::obs_frontend_recording_start() };
}

pub fn stop() {
    unsafe { libobs_sys::obs_frontend_recording_stop() };
}

pub fn pause(pause: bool) {
    unsafe { libobs_sys::obs_frontend_recording_pause(pause) };
}

pub fn split_file() -> bool {
    unsafe { libobs_sys::obs_frontend_recording_split_file() }
}

pub fn output() -> Output<'static> {
    let raw = unsafe { libobs_sys::obs_frontend_get_recording_output() };
    Output::from_raw(raw)
}
