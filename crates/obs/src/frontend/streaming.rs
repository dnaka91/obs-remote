use crate::{output::Output, service::Service};

pub fn active() -> bool {
    unsafe { libobs_sys::obs_frontend_streaming_active() }
}

pub fn start() {
    unsafe { libobs_sys::obs_frontend_streaming_start() };
}

pub fn stop() {
    unsafe { libobs_sys::obs_frontend_streaming_stop() };
}

pub fn save() {
    unsafe { libobs_sys::obs_frontend_save_streaming_service() };
}

pub fn output() -> Output<'static> {
    let raw = unsafe { libobs_sys::obs_frontend_get_streaming_output() };
    Output::from_raw(raw)
}

pub fn service() -> Service<'static> {
    let raw = unsafe { libobs_sys::obs_frontend_get_streaming_service() };
    let raw = unsafe { libobs_sys::obs_service_get_ref(raw) };
    Service::from_raw(raw)
}

pub fn set_service(service: &Service<'_>) {
    unsafe { libobs_sys::obs_frontend_set_streaming_service(service.as_ptr()) };
}
