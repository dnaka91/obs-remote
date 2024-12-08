use std::{marker::PhantomData, ptr::NonNull};

use super::calldata::Calldata;
use crate::util::StringToFfi;

pub struct ProcHandler<'a> {
    raw: NonNull<libobs_sys::proc_handler_t>,
    life: PhantomData<&'a ()>,
    destroy: bool,
}

impl Drop for ProcHandler<'_> {
    fn drop(&mut self) {
        if self.destroy {
            unsafe { libobs_sys::proc_handler_destroy(self.raw.as_ptr()) };
            self.destroy = false;
        }
    }
}

impl ProcHandler<'_> {
    pub(crate) fn from_raw(raw: *mut libobs_sys::proc_handler_t, destroy: bool) -> Self {
        Self {
            raw: unsafe { NonNull::new_unchecked(raw) },
            life: PhantomData,
            destroy,
        }
    }

    pub fn create() -> Self {
        let raw = unsafe { libobs_sys::proc_handler_create() };
        Self::from_raw(raw, true)
    }

    pub fn call(&mut self, name: &str, params: &mut Calldata) -> bool {
        let name = name.cstr();

        unsafe { libobs_sys::proc_handler_call(self.raw.as_ptr(), name.as_ptr(), params.as_ptr()) }
    }
}
