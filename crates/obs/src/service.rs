use std::{ffi::CStr, marker::PhantomData, os::raw::c_char, ptr::NonNull};

use crate::{
    data::Data,
    util::{self, FfiToString, StringToFfi},
};

pub struct Service<'a> {
    raw: NonNull<libobs_sys::obs_service_t>,
    life: PhantomData<&'a ()>,
}

impl<'a> Drop for Service<'a> {
    fn drop(&mut self) {
        unsafe { libobs_sys::obs_service_release(self.raw.as_ptr()) }
    }
}

impl<'a> Service<'a> {
    pub(crate) fn from_raw(raw: *mut libobs_sys::obs_service_t) -> Self {
        Self {
            raw: unsafe { NonNull::new_unchecked(raw) },
            life: PhantomData::default(),
        }
    }

    pub(crate) fn as_ptr(&self) -> *mut libobs_sys::obs_service_t {
        self.raw.as_ptr()
    }

    pub fn create(id: &str, name: &str, settings: &Data<'_>, hotkey_data: &Data<'_>) -> Self {
        let id = id.cstr();
        let name = name.cstr();

        Self::from_raw(unsafe {
            libobs_sys::obs_service_create(
                id.as_ptr(),
                name.as_ptr(),
                settings.as_ptr(),
                hotkey_data.as_ptr(),
            )
        })
    }

    pub fn display_name(id: &str) -> Option<String> {
        let id = id.cstr();

        unsafe { libobs_sys::obs_service_get_display_name(id.as_ptr()) }.into_opt_string()
    }

    pub fn id(&self) -> String {
        unsafe { libobs_sys::obs_service_get_id(self.raw.as_ptr()) }.into_string()
    }

    pub fn max_bitrate(&self) -> (u32, u32) {
        let mut video = 0;
        let mut audio = 0;

        unsafe {
            libobs_sys::obs_service_get_max_bitrate(
                self.raw.as_ptr(),
                &mut video as *mut _,
                &mut audio as *mut _,
            )
        };

        (video as u32, audio as u32)
    }

    pub fn max_fps(&self) -> u32 {
        let mut fps = 0;

        unsafe { libobs_sys::obs_service_get_max_fps(self.raw.as_ptr(), &mut fps as *mut _) };

        fps as u32
    }

    pub fn name(&self) -> String {
        unsafe { libobs_sys::obs_service_get_name(self.raw.as_ptr()) }.into_string()
    }

    pub fn settings(&self) -> Data<'_> {
        Data::from_raw(unsafe { libobs_sys::obs_service_get_settings(self.raw.as_ptr()) })
    }

    pub fn ty(&self) -> StreamType {
        StreamType::from_native(unsafe { libobs_sys::obs_service_get_type(self.raw.as_ptr()) })
    }

    pub fn url(&self) -> String {
        unsafe { libobs_sys::obs_service_get_url(self.raw.as_ptr()) }.into_string()
    }

    pub fn supported_video_codecs(&self) -> Vec<String> {
        util::convert_string_list(unsafe {
            libobs_sys::obs_service_get_supported_video_codecs(self.raw.as_ptr())
        })
    }

    /// Updates the settings of the service context.
    pub fn update(&self, settings: &Data<'_>) {
        unsafe { libobs_sys::obs_service_update(self.raw.as_ptr(), settings.as_ptr()) };
    }
}

pub fn list_service_types() -> Vec<String> {
    util::list_types(libobs_sys::obs_enum_service_types)
}

pub fn list_services() -> Vec<Service<'static>> {
    util::list_instances(
        libobs_sys::obs_enum_services,
        libobs_sys::obs_service_get_ref,
        Service::from_raw,
    )
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum StreamType {
    RtmpCommon,
    RtmpCustom,
    Unknown,
}

impl StreamType {
    fn from_native(value: *const c_char) -> Self {
        match unsafe { CStr::from_ptr(value) }.to_string_lossy().as_ref() {
            "rtmp_common" => Self::RtmpCommon,
            "rtmp_custom" => Self::RtmpCustom,
            _ => Self::Unknown,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::RtmpCommon => "rtmp_common",
            Self::RtmpCustom => "rtmp_custom",
            Self::Unknown => "",
        }
    }
}
