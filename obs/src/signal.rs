use std::{
    ffi::{c_void, CString},
    mem,
    ptr::{self, NonNull},
};

use crate::{cstr, cstr_ptr, scene::SceneItem, source::Source, Ref};

#[derive(Clone)]
pub struct SignalHandler {
    raw: NonNull<libobs_sys::signal_handler_t>,
}

unsafe impl Send for SignalHandler {}

unsafe impl Sync for SignalHandler {}

impl SignalHandler {
    fn from_raw(raw: *mut libobs_sys::signal_handler_t) -> Self {
        Self {
            raw: unsafe { NonNull::new_unchecked(raw) },
        }
    }

    pub fn get() -> Option<Self> {
        let raw = unsafe { libobs_sys::obs_get_signal_handler() };
        (!raw.is_null()).then(|| Self::from_raw(raw))
    }

    pub fn connect<C: Callback>(&self, signal: &str, callback: C) -> Handle<C> {
        let signal = cstr!(signal);
        let callback = Box::into_raw(Box::new(callback));
        unsafe {
            libobs_sys::signal_handler_connect(
                self.raw.as_ptr(),
                signal.as_ptr(),
                Some(signal_callback::<C>),
                callback.cast(),
            )
        };

        Handle {
            handler: self.raw,
            signal,
            callback: unsafe { NonNull::new_unchecked(callback) },
        }
    }
}

pub trait Callback {
    fn call(&mut self, data: &Calldata);
}

pub struct Handle<C: Callback> {
    handler: NonNull<libobs_sys::signal_handler_t>,
    signal: CString,
    callback: NonNull<C>,
}

impl<C: Callback> Drop for Handle<C> {
    fn drop(&mut self) {
        unsafe {
            libobs_sys::signal_handler_disconnect(
                self.handler.as_ptr(),
                self.signal.as_ptr(),
                Some(signal_callback::<C>),
                self.callback.as_ptr().cast(),
            );
            Box::from_raw(self.callback.as_ptr());
        }
    }
}

unsafe impl<C: Callback> Send for Handle<C> {}

unsafe impl<C: Callback> Sync for Handle<C> {}

unsafe extern "C" fn signal_callback<C: Callback>(
    param: *mut c_void,
    data: *mut libobs_sys::calldata_t,
) {
    let callback = &mut *param.cast::<C>();
    callback.call(&Calldata { raw: data });
}

pub struct Calldata {
    raw: *mut libobs_sys::calldata_t,
}

impl Calldata {
    fn ptr<T>(&self, name: &str) -> Option<NonNull<T>> {
        let mut ptr = ptr::null_mut::<c_void>();
        let success = unsafe {
            libobs_sys::calldata_get_data(
                self.raw,
                cstr_ptr!(name),
                &mut ptr as *mut *mut _ as *mut _,
                mem::size_of::<*mut c_void>() as u64,
            )
        };

        success.then(|| unsafe { NonNull::new_unchecked(ptr.cast()) })
    }

    pub fn get_source(&self) -> Option<Ref<'_, Self, Source>> {
        self.ptr("source").map(|p| {
            // unsafe { libobs_sys::obs_source_addref(p.as_ptr()) };
            Ref::new(Source::from_raw_no_release(p.as_ptr()))
        })
    }

    pub fn get_filter(&self) -> Option<Ref<'_, Self, Source>> {
        self.ptr("filter").map(|p| {
            unsafe { libobs_sys::obs_source_addref(p.as_ptr()) };
            Ref::new(Source::from_raw(p.as_ptr()))
        })
    }

    pub fn get_scene_item(&self) -> Option<Ref<'_, Self, SceneItem>> {
        self.ptr("item").map(|p| {
            unsafe { libobs_sys::obs_sceneitem_addref(p.as_ptr()) };
            Ref::new(SceneItem::from_raw(p.as_ptr()))
        })
    }
}
