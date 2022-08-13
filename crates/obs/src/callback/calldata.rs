use std::{
    ffi::c_void,
    mem,
    os::raw::{c_char, c_double, c_longlong},
    ptr::{self, NonNull},
};

use crate::{
    output::Output,
    scene::{Scene, SceneItem},
    source::Source,
    util::{FfiToString, StringToFfi},
};

pub struct Calldata {
    raw: NonNull<libobs_sys::calldata_t>,
    free: bool,
}

impl Drop for Calldata {
    fn drop(&mut self) {
        if self.free {
            let raw = unsafe { self.raw.as_mut() };
            if !raw.fixed {
                unsafe { libobs_sys::bfree(raw.stack as *mut _) };
            }
            self.free = false;
        }
    }
}

impl Default for Calldata {
    fn default() -> Self {
        // TODO: Safety: probably UB to call `bfree` on a pointer created with `Box`.
        let raw = Box::leak(Box::new(libobs_sys::calldata_t::default()));
        Self::from_raw(raw as *mut _, true)
    }
}

impl Calldata {
    pub(crate) fn from_raw(raw: *mut libobs_sys::calldata_t, free: bool) -> Self {
        Self {
            raw: unsafe { NonNull::new_unchecked(raw) },
            free,
        }
    }

    pub(crate) fn as_ptr(&self) -> *mut libobs_sys::calldata_t {
        self.raw.as_ptr()
    }

    pub fn int(&self, name: &str) -> Option<i64> {
        let name = name.cstr();
        let mut val: c_longlong = 0;

        let success = unsafe {
            libobs_sys::calldata_get_data(
                self.raw.as_ptr(),
                name.as_ptr(),
                (&mut val as *mut c_longlong).cast(),
                mem::size_of::<c_longlong>() as u64,
            )
        };

        success.then(|| val as i64)
    }

    pub fn float(&self, name: &str) -> Option<f64> {
        let name = name.cstr();
        let mut val: c_double = 0.0;

        let success = unsafe {
            libobs_sys::calldata_get_data(
                self.raw.as_ptr(),
                name.as_ptr(),
                (&mut val as *mut c_double).cast(),
                mem::size_of::<c_double>() as u64,
            )
        };

        success.then(|| val as f64)
    }

    pub fn bool(&self, name: &str) -> Option<bool> {
        let name = name.cstr();
        let mut val = false;

        let success = unsafe {
            libobs_sys::calldata_get_data(
                self.raw.as_ptr(),
                name.as_ptr(),
                (&mut val as *mut bool).cast(),
                mem::size_of::<bool>() as u64,
            )
        };

        success.then(|| val)
    }

    fn ptr<T>(&self, name: &str) -> Option<NonNull<T>> {
        let name = name.cstr();
        let mut val = ptr::null_mut::<c_void>();
        let success = unsafe {
            libobs_sys::calldata_get_data(
                self.raw.as_ptr(),
                name.as_ptr(),
                (&mut val as *mut *mut c_void).cast(),
                mem::size_of::<*mut c_void>() as u64,
            )
        };

        (success && !val.is_null()).then(|| unsafe { NonNull::new_unchecked(val.cast()) })
    }

    pub fn string(&self, name: &str) -> Option<String> {
        let name = name.cstr();
        let mut val = ptr::null_mut::<c_char>();

        let success = unsafe {
            libobs_sys::calldata_get_data(
                self.raw.as_ptr(),
                name.as_ptr(),
                (&mut val as *mut *mut c_char).cast(),
                mem::size_of::<*mut c_char>() as u64,
            )
        };

        (success && !val.is_null()).then(|| val.into_string())
    }

    pub fn get_source(&self) -> Option<Source<'_>> {
        self.ptr("source")
            .map(|p| Source::from_raw_no_release(p.as_ptr()))
    }

    pub fn get_filter(&self) -> Option<Source<'_>> {
        self.ptr("filter").map(|p| {
            let ptr = unsafe { libobs_sys::obs_source_get_ref(p.as_ptr()) };
            Source::from_raw(ptr)
        })
    }

    pub fn get_scene(&self) -> Option<Scene<'_>> {
        self.ptr("scene").map(|p| {
            let ptr = unsafe { libobs_sys::obs_scene_get_ref(p.as_ptr()) };
            Scene::from_raw(ptr)
        })
    }

    pub fn get_scene_item(&self) -> Option<SceneItem<'_>> {
        self.ptr("item").map(|p| {
            unsafe { libobs_sys::obs_sceneitem_addref(p.as_ptr()) };
            SceneItem::from_raw(p.as_ptr())
        })
    }

    pub fn get_output(&self) -> Option<Output<'_>> {
        self.ptr("output").map(|p| {
            let ptr = unsafe { libobs_sys::obs_output_get_ref(p.as_ptr()) };
            Output::from_raw(ptr)
        })
    }
}
