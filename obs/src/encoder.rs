use std::{ffi::c_void, ptr::NonNull};

use crate::util::StringConversion;

pub struct Encoder {
    raw: NonNull<libobs_sys::obs_encoder_t>,
}

impl Encoder {
    pub(crate) fn from_raw(raw: *mut libobs_sys::obs_encoder_t) -> Self {
        Self {
            raw: unsafe { NonNull::new_unchecked(raw) },
        }
    }

    pub fn name(&self) -> String {
        unsafe { libobs_sys::obs_encoder_get_name(self.raw.as_ptr()) }.into_string()
    }
}

pub fn list() -> Vec<Encoder> {
    unsafe extern "C" fn callback(
        param: *mut c_void,
        encoder: *mut libobs_sys::obs_encoder_t,
    ) -> bool {
        if !encoder.is_null() {
            let param = &mut *param.cast::<Vec<Encoder>>();
            param.push(Encoder::from_raw(encoder));
        }

        true
    }

    let mut encoders = Vec::<Encoder>::new();
    unsafe { libobs_sys::obs_enum_encoders(Some(callback), (&mut encoders as *mut Vec<_>).cast()) };

    encoders
}
