use super::convert_string_list;
use crate::source::Source;

pub fn current() -> Source<'static> {
    let raw = unsafe { libobs_sys::obs_frontend_get_current_scene() };
    Source::from_raw(raw)
}

pub fn set_current(scene: &Source<'_>) {
    unsafe { libobs_sys::obs_frontend_set_current_scene(scene.as_ptr()) };
}

pub fn current_preview() -> Option<Source<'static>> {
    let raw = unsafe { libobs_sys::obs_frontend_get_current_preview_scene() };
    (!raw.is_null()).then(|| Source::from_raw(raw))
}

pub fn set_current_preview(scene: &Source<'_>) {
    unsafe { libobs_sys::obs_frontend_set_current_preview_scene(scene.as_ptr()) };
}

pub fn list() -> Vec<Source<'static>> {
    let mut list = libobs_sys::obs_frontend_source_list::default();
    unsafe { libobs_sys::obs_frontend_get_scenes((&mut list) as *mut _) };

    let list = unsafe { list.sources.__bindgen_anon_1 };

    (0..list.num as usize)
        .map(|i| Source::from_raw(unsafe { *list.array.add(i) }))
        .collect()
}

pub fn names() -> Vec<String> {
    convert_string_list(unsafe { libobs_sys::obs_frontend_get_scene_names() })
}
