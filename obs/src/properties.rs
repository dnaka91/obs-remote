use std::ptr::NonNull;

pub struct Properties {
    raw: NonNull<libobs_sys::obs_properties_t>,
}

impl Drop for Properties {
    fn drop(&mut self) {
        unsafe { libobs_sys::obs_properties_destroy(self.raw.as_ptr()) }
    }
}

impl Properties {
    pub(crate) fn from_raw(raw: *mut libobs_sys::obs_properties_t) -> Self {
        Self {
            raw: unsafe { NonNull::new_unchecked(raw) },
        }
    }
}
