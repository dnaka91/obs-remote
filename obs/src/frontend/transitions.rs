use crate::source::Source;

pub fn current() -> Source {
    let raw = unsafe { libobs_sys::obs_frontend_get_current_transition() };
    Source::from_raw(raw)
}

pub fn set_current(source: &Source) {
    unsafe { libobs_sys::obs_frontend_set_current_transition(source.as_ptr()) };
}

pub fn duration() -> i32 {
    unsafe { libobs_sys::obs_frontend_get_transition_duration() }
}

pub fn set_duration(duration: i32) {
    unsafe { libobs_sys::obs_frontend_set_transition_duration(duration) };
}

pub fn list() -> Vec<Source> {
    let mut sources = libobs_sys::obs_frontend_source_list::default();
    unsafe { libobs_sys::obs_frontend_get_transitions(&mut sources as *mut _) };

    let array = unsafe { sources.sources.__bindgen_anon_1.array };
    let num = unsafe { sources.sources.__bindgen_anon_1.num } as usize;

    let mut source_list = Vec::with_capacity(num);

    for i in 0..num {
        let source = unsafe { *array.add(i) };
        // unsafe { libobs_sys::obs_source_addref(source) };

        source_list.push(Source::from_raw(source));
    }

    source_list
}
