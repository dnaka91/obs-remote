use std::{marker::PhantomData, ptr::NonNull};

pub struct Properties<'a> {
    raw: NonNull<libobs_sys::obs_properties_t>,
    life: PhantomData<&'a ()>,
}

impl<'a> Drop for Properties<'a> {
    fn drop(&mut self) {
        unsafe { libobs_sys::obs_properties_destroy(self.raw.as_ptr()) }
    }
}

impl<'a> Properties<'a> {
    pub(crate) fn from_raw(raw: *mut libobs_sys::obs_properties_t) -> Self {
        Self {
            raw: unsafe { NonNull::new_unchecked(raw) },
            life: PhantomData::default(),
        }
    }
}
