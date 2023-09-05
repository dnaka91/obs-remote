use std::{ffi::c_void, marker::PhantomData, ptr::NonNull};

use crate::util::FfiToString;

pub struct Encoder<'a> {
    raw: NonNull<libobs_sys::obs_encoder_t>,
    life: PhantomData<&'a ()>,
}

impl<'a> Encoder<'a> {
    pub(crate) fn from_raw(raw: *mut libobs_sys::obs_encoder_t) -> Self {
        Self {
            raw: unsafe { NonNull::new_unchecked(raw) },
            life: PhantomData,
        }
    }

    pub(crate) fn as_ptr(&self) -> *mut libobs_sys::obs_encoder_t {
        self.raw.as_ptr()
    }

    pub fn codec(&self) -> String {
        unsafe { libobs_sys::obs_encoder_get_codec(self.raw.as_ptr()) }.into_string()
    }

    pub fn name(&self) -> String {
        unsafe { libobs_sys::obs_encoder_get_name(self.raw.as_ptr()) }.into_string()
    }

    pub fn ty(&self) -> EncoderType {
        EncoderType::from_native(unsafe { libobs_sys::obs_encoder_get_type(self.raw.as_ptr()) })
    }

    /// For video encoders, returns true if pre-encode scaling is enabled.
    pub fn scaling(&self) -> bool {
        unsafe { libobs_sys::obs_encoder_scaling_enabled(self.raw.as_ptr()) }
    }

    /// For video encoders, returns the width of the encoded image.
    pub fn width(&self) -> u32 {
        unsafe { libobs_sys::obs_encoder_get_width(self.raw.as_ptr()) }
    }

    /// For video encoders, returns the height of the encoded image.
    pub fn height(&self) -> u32 {
        unsafe { libobs_sys::obs_encoder_get_height(self.raw.as_ptr()) }
    }

    /// For audio encoders, returns the sample rate of the audio.
    pub fn sample_rate(&self) -> u32 {
        unsafe { libobs_sys::obs_encoder_get_sample_rate(self.raw.as_ptr()) }
    }

    /// For audio encoders, returns the frame size of the audio packet.
    pub fn frame_size(&self) -> usize {
        unsafe { libobs_sys::obs_encoder_get_frame_size(self.raw.as_ptr()) }
    }
}

pub fn list() -> Vec<Encoder<'static>> {
    unsafe extern "C" fn callback(
        param: *mut c_void,
        encoder: *mut libobs_sys::obs_encoder_t,
    ) -> bool {
        if !encoder.is_null() {
            let param = &mut *param.cast::<Vec<Encoder<'_>>>();
            param.push(Encoder::from_raw(encoder));
        }

        true
    }

    let mut encoders = Vec::<Encoder<'_>>::new();
    unsafe { libobs_sys::obs_enum_encoders(Some(callback), (&mut encoders as *mut Vec<_>).cast()) };

    encoders
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug)]
pub enum EncoderType {
    Audio,
    Video,
    Unknown(u32),
}

impl EncoderType {
    fn from_native(ty: libobs_sys::obs_encoder_type::Type) -> Self {
        use libobs_sys::obs_encoder_type::*;

        match ty {
            OBS_ENCODER_AUDIO => Self::Audio,
            OBS_ENCODER_VIDEO => Self::Video,
            _ => Self::Unknown(ty as _),
        }
    }
}
