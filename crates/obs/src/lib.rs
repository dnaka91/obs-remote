#![deny(rust_2018_idioms)]

use std::{ffi::CStr, fmt::{Display, self}, os::raw::c_char};

pub use num_rational::Ratio;
pub use time::Duration;

use crate::util::StringConversion;

pub mod audio;
pub mod callback;
pub mod config;
pub mod data;
pub mod encoder;
pub mod filter;
pub mod frontend;
pub mod graphics;
pub mod gs;
pub mod hotkeys;
pub mod logger;
pub mod module;
pub mod os;
pub mod output;
pub mod properties;
pub mod scene;
pub mod service;
pub mod source;
#[cfg(feature = "tracing")]
pub mod tracing;
pub(crate) mod util;
pub mod video;

pub use libobs_sys;

#[macro_export]
macro_rules! declare_module {
    ($t:ty) => {
        static mut INSTANCE: Option<$t> = None;

        static OBS_MODULE_POINTER: std::sync::atomic::AtomicPtr<$crate::libobs_sys::obs_module_t> =
            std::sync::atomic::AtomicPtr::new(std::ptr::null_mut());

        #[no_mangle]
        pub extern "C" fn obs_module_set_pointer(module: *mut $crate::libobs_sys::obs_module_t) {
            unsafe { INSTANCE = Some(<$t>::new()) };
            OBS_MODULE_POINTER.store(module, std::sync::atomic::Ordering::SeqCst);
        }

        #[no_mangle]
        pub extern "C" fn obs_current_module() -> *mut $crate::libobs_sys::obs_module_t {
            OBS_MODULE_POINTER.load(std::sync::atomic::Ordering::SeqCst)
        }

        #[no_mangle]
        pub extern "C" fn obs_module_ver() -> std::os::raw::c_uint {
            $crate::libobs_sys::LIBOBS_API_VER
        }

        #[no_mangle]
        pub extern "C" fn obs_module_load() -> bool {
            match unsafe { INSTANCE.as_mut() } {
                Some(instance) => instance.load(),
                None => false,
            }
        }

        #[no_mangle]
        pub extern "C" fn obs_module_unload() {
            if let Some(instance) = unsafe { INSTANCE.as_mut() } {
                instance.unload();
            }
        }

        #[no_mangle]
        pub extern "C" fn obs_module_post_load() {
            if let Some(instance) = unsafe { INSTANCE.as_mut() } {
                instance.post_load()
            }
        }

        #[no_mangle]
        pub extern "C" fn obs_module_author() -> *const std::os::raw::c_char {
            <$t>::author()
                .unwrap_or_else(|| unsafe {
                    std::ffi::CStr::from_bytes_with_nul_unchecked(
                        concat!(env!("CARGO_PKG_AUTHORS"), '\0').as_bytes(),
                    )
                })
                .as_ptr()
        }

        #[no_mangle]
        pub extern "C" fn obs_module_name() -> *const std::os::raw::c_char {
            <$t>::name()
                .unwrap_or_else(|| unsafe {
                    std::ffi::CStr::from_bytes_with_nul_unchecked(
                        concat!(env!("CARGO_PKG_NAME"), '\0').as_bytes(),
                    )
                })
                .as_ptr()
        }

        #[no_mangle]
        pub extern "C" fn obs_module_description() -> *const std::os::raw::c_char {
            <$t>::description()
                .unwrap_or_else(|| unsafe {
                    unsafe {
                        std::ffi::CStr::from_bytes_with_nul_unchecked(
                            concat!(env!("CARGO_PKG_DESCRIPTION"), '\0').as_bytes(),
                        )
                    }
                })
                .as_ptr()
        }
    };
}

#[macro_export]
macro_rules! module_use_default_locale {
    ($default_locale:expr) => {
        static OBS_MODULE_LOOKUP: std::sync::atomic::AtomicPtr<$crate::libobs_sys::lookup_t> =
            std::sync::atomic::AtomicPtr::new(std::ptr::null_mut());

        #[no_mangle]
        pub unsafe extern "C" fn obs_module_set_locale(locale: *const std::os::raw::c_char) {
            let lookup = OBS_MODULE_LOOKUP.load(std::sync::atomic::Ordering::SeqCst);
            if !lookup.is_null() {
                $crate::libobs_sys::text_lookup_destroy(lookup);
            }

            let default_locale = std::ffi::CStr::from_bytes_with_nul_unchecked(
                concat!($default_locale, '\0').as_bytes(),
            );
            let lookup = $crate::libobs_sys::obs_module_load_locale(
                obs_current_module(),
                default_locale.as_ptr(),
                locale,
            );
            OBS_MODULE_LOOKUP.store(lookup, std::sync::atomic::Ordering::SeqCst);
        }

        #[no_mangle]
        pub extern "C" fn obs_module_free_locale() {
            let lookup =
                OBS_MODULE_LOOKUP.swap(std::ptr::null_mut(), std::sync::atomic::Ordering::SeqCst);
            unsafe { $crate::libobs_sys::text_lookup_destroy(lookup) };
        }

        #[no_mangle]
        pub unsafe extern "C" fn obs_module_text(
            lookup_string: *const std::os::raw::c_char,
        ) -> *const std::os::raw::c_char {
            let mut out = lookup_string;
            let lookup = OBS_MODULE_LOOKUP.load(std::sync::atomic::Ordering::SeqCst);

            $crate::libobs_sys::text_lookup_getstr(lookup, lookup_string, &mut out as *mut _);

            out
        }

        #[no_mangle]
        pub unsafe extern "C" fn obs_module_get_string(
            lookup_string: *const std::os::raw::c_char,
            translated_string: *mut *const std::os::raw::c_char,
        ) -> bool {
            let lookup = OBS_MODULE_LOOKUP.load(std::sync::atomic::Ordering::SeqCst);
            $crate::libobs_sys::text_lookup_getstr(lookup, lookup_string, translated_string)
        }
    };
}

/// Internal macro to conveniently create a [`CString`](std::ffi::CString).
#[doc(hidden)]
#[macro_export]
macro_rules! cstr {
    ($v:expr) => {
        std::ffi::CString::new($v).expect("Invalid string containing 0 bytes")
    };
}

/// Internal macro that does the same as [`cstr`] but additionally turns the `CString` into a
/// `*const c_char` pointer.
#[doc(hidden)]
#[macro_export]
macro_rules! cstr_ptr {
    ($v:expr) => {
        std::ffi::CString::new($v)
            .expect("Invalid string containing 0 bytes")
            .as_ptr()
    };
}

pub trait Plugin {
    fn new() -> Self;

    fn load(&mut self) -> bool;

    fn unload(&mut self) {}

    fn post_load(&mut self) {}

    fn author() -> Option<&'static CStr> {
        None
    }

    fn name() -> Option<&'static CStr> {
        None
    }

    fn description() -> Option<&'static CStr> {
        None
    }
}

pub fn active_fps() -> f64 {
    unsafe { libobs_sys::obs_get_active_fps() }
}

pub fn average_frame_time_ns() -> u64 {
    unsafe { libobs_sys::obs_get_average_frame_time_ns() }
}

/// Get the argc/argv used to start OBS.
pub fn cmdline_args() -> Vec<String> {
    let args = unsafe { libobs_sys::obs_get_cmdline_args() };
    (0..args.argc)
        .map(|i| unsafe { (*args.argv.add(i as usize)) as *const c_char }.into_string())
        .collect()
}

pub fn frame_interval_ns() -> u64 {
    unsafe { libobs_sys::obs_get_frame_interval_ns() }
}

pub fn lagged_frames() -> u32 {
    unsafe { libobs_sys::obs_get_lagged_frames() }
}

pub fn total_frames() -> u32 {
    unsafe { libobs_sys::obs_get_total_frames() }
}

pub fn locale() -> String {
    unsafe { CStr::from_ptr(libobs_sys::obs_get_locale()) }
        .to_string_lossy()
        .into_owned()
}

/// Get the master user volume.
pub fn master_volume() -> f32 {
    unsafe { libobs_sys::obs_get_master_volume() }
}

pub fn render_main_texture() {
    unsafe { libobs_sys::obs_render_main_texture() };
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Version {
    pub major: u8,
    pub minor: u8,
    pub patch: u16,
}

impl Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}

pub fn obs_version() -> Version {
    let version = unsafe { libobs_sys::obs_get_version() };

    Version {
        major: (version >> 24 & 0xff) as u8,
        minor: (version >> 16 & 0xff) as u8,
        patch: (version & 0xffff) as u16,
    }
}

pub fn obs_version_string() -> String {
    unsafe { CStr::from_ptr(libobs_sys::obs_get_version_string()) }
        .to_string_lossy()
        .into_owned()
}
