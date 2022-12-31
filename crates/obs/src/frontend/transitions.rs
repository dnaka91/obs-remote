use std::os::raw::c_int;

use time::Duration;

use crate::source::Source;

pub fn current() -> Source<'static> {
    let raw = unsafe { libobs_sys::obs_frontend_get_current_transition() };
    Source::from_raw(raw)
}

pub fn set_current(source: &Source<'_>) {
    unsafe { libobs_sys::obs_frontend_set_current_transition(source.as_ptr()) };
}

pub fn duration() -> Duration {
    Duration::milliseconds(unsafe { libobs_sys::obs_frontend_get_transition_duration() }.into())
}

pub fn set_duration(duration: Duration) {
    // TODO: maybe fail on truncation
    unsafe {
        libobs_sys::obs_frontend_set_transition_duration(duration.whole_milliseconds() as c_int)
    };
}

pub fn release_tbar() {
    unsafe { libobs_sys::obs_frontend_release_tbar() };
}

pub fn tbar_position() -> i32 {
    unsafe { libobs_sys::obs_frontend_get_tbar_position() }
}

pub fn set_tbar_position(position: i32) {
    unsafe { libobs_sys::obs_frontend_set_tbar_position(position) };
}

pub fn list() -> Vec<Source<'static>> {
    let mut sources = libobs_sys::obs_frontend_source_list::default();
    unsafe { libobs_sys::obs_frontend_get_transitions(&mut sources as *mut _) };

    let array = unsafe { sources.sources.__bindgen_anon_1.array };
    let num = unsafe { sources.sources.__bindgen_anon_1.num };

    let mut source_list = Vec::with_capacity(num);

    for i in 0..num {
        let source = unsafe { *array.add(i) };
        source_list.push(Source::from_raw(source));
    }

    source_list
}
