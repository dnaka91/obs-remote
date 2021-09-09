use crate::output::Output;

pub fn active() -> bool {
    unsafe { libobs_sys::obs_frontend_virtualcam_active() }
}

pub fn start() {
    unsafe { libobs_sys::obs_frontend_start_virtualcam() };
}

pub fn stop() {
    unsafe { libobs_sys::obs_frontend_stop_virtualcam() };
}

pub fn output() -> Output<'static> {
    let raw = unsafe { libobs_sys::obs_frontend_get_virtualcam_output() };
    Output::from_raw(raw)
}
