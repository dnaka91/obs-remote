use std::{mem::size_of_val, os::raw::c_void};

use crate::{
    data::Data,
    source::{IconType, OutputFlags, Source, SourceType},
};

#[derive(Clone, Copy, Debug)]
pub struct SourceInfo {
    raw: libobs_sys::obs_source_info,
    pointers: Pointers,
}

#[derive(Clone, Copy, Debug, Default)]
struct Pointers {
    get_name: Option<fn() -> &'static str>,
    create: Option<fn(settings: &mut Data<'_>, source: &mut Source<'_>) -> *mut ()>,
    get_width: Option<fn(*mut ()) -> u32>,
    get_height: Option<fn(*mut ()) -> u32>,
}

impl SourceInfo {
    pub fn new(id: &'static str, ty: SourceType, output_flags: OutputFlags) -> Self {
        let pointers = Pointers::default();

        Self {
            raw: libobs_sys::obs_source_info {
                id: id.as_ptr().cast(),
                type_: ty.to_native(),
                output_flags: output_flags.bits(),
                get_name: None,
                create: None,
                destroy: None,
                get_width: None,
                get_height: None,
                get_defaults: None,
                get_properties: None,
                update: None,
                activate: None,
                deactivate: None,
                show: None,
                hide: None,
                video_tick: None,
                video_render: None,
                filter_video: None,
                filter_audio: None,
                enum_active_sources: None,
                save: None,
                load: None,
                mouse_click: None,
                mouse_move: None,
                mouse_wheel: None,
                focus: None,
                key_click: None,
                filter_remove: None,
                type_data: &pointers as *const _ as *mut _,
                free_type_data: None,
                audio_render: None,
                enum_all_sources: None,
                transition_start: None,
                transition_stop: None,
                get_defaults2: None,
                get_properties2: None,
                audio_mix: None,
                icon_type: IconType::Color.to_native(),
                media_play_pause: None,
                media_restart: None,
                media_stop: None,
                media_next: None,
                media_previous: None,
                media_get_duration: None,
                media_get_time: None,
                media_set_time: None,
                media_get_state: None,
                version: 0,
                unversioned_id: std::ptr::null(),
                missing_files: None,
                video_get_color_space: None,
                filter_add: None,
            },
            pointers,
        }
    }

    pub fn get_name(mut self, f: fn() -> &'static str) -> Self {
        unsafe extern "C" fn trampoline(ptrs: *mut c_void) -> *const i8 {
            let ptrs = unsafe { &*ptrs.cast::<Pointers>() };
            ptrs.get_name
                .map(|f| f().as_ptr().cast())
                .unwrap_or(std::ptr::null())
        }

        self.pointers.get_name = Some(f);
        self.raw.get_name = Some(trampoline);
        self
    }

    pub fn create(mut self, f: fn(&mut Data<'_>, &mut Source<'_>) -> *mut ()) -> Self {
        unsafe extern "C" fn trampoline(
            data: *mut libobs_sys::obs_data_t,
            source: *mut libobs_sys::obs_source_t,
        ) -> *mut c_void {
            let mut data = Data::from_raw(data);
            let mut source = Source::from_raw_no_release(source);
            let ptrs = unsafe { &*source.type_data().cast::<Pointers>() };

            ptrs.create
                .map(|f| f(&mut data, &mut source).cast())
                .unwrap_or(std::ptr::null_mut())
        }
        self.pointers.create = Some(f);
        self.raw.create = Some(trampoline);
        self
    }

    pub fn get_width(mut self, f: fn(*mut ()) -> u32) -> Self {
        unsafe extern "C" fn trampoline(_data: *mut c_void) -> u32 {
            0
        }
        self.pointers.get_width = Some(f);
        self.raw.get_width = Some(trampoline);
        self
    }

    pub fn get_height(mut self, f: fn(*mut ()) -> u32) -> Self {
        unsafe extern "C" fn trampoline(_data: *mut c_void) -> u32 {
            0
        }
        self.pointers.get_height = Some(f);
        self.raw.get_height = Some(trampoline);
        self
    }
}

pub fn source(info: &SourceInfo) {
    unsafe { libobs_sys::obs_register_source_s(&info.raw as *const _, size_of_val(&info.raw)) };
}
