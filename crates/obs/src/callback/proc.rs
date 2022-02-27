use std::{marker::PhantomData, ptr::NonNull};

use super::calldata::Calldata;
use crate::cstr_ptr;

pub struct ProcHandler<'a> {
    raw: NonNull<libobs_sys::proc_handler_t>,
    life: PhantomData<&'a ()>,
    destroy: bool,
}

impl<'a> Drop for ProcHandler<'a> {
    fn drop(&mut self) {
        if self.destroy {
            unsafe { libobs_sys::proc_handler_destroy(self.raw.as_ptr()) };
            self.destroy = false;
        }
    }
}

impl<'a> ProcHandler<'a> {
    pub(crate) fn from_raw(raw: *mut libobs_sys::proc_handler_t, destroy: bool) -> Self {
        Self {
            raw: unsafe { NonNull::new_unchecked(raw) },
            life: PhantomData::default(),
            destroy,
        }
    }

    pub fn create() -> Self {
        let raw = unsafe { libobs_sys::proc_handler_create() };
        Self::from_raw(raw, true)
    }

    pub fn call(&mut self, name: &str, params: &mut Calldata) -> bool {
        unsafe {
            libobs_sys::proc_handler_call(self.raw.as_ptr(), cstr_ptr!(name), params.as_ptr())
        }
    }
}
