pub enum BaseEffect {
    /// RGB/YUV.
    Default,
    /// RGB/YUV (using texture_rect).
    DefaultRect,
    /// RGB/YUV (alpha set to 1.0).
    Opaque,
    /// RGB/YUV (solid color only).
    Solid,
    /// Bicubic downscale.
    Bicubic,
    /// Lanczos downscale.
    Lanczos,
    /// Bilinear low resolution downscale.
    BilinearLowres,
    /// Premultiplied alpha.
    PremultipliedAlpha,
    /// RGB/YUV (repeating).
    Repeat,
    /// Area rescale.
    Area,
}

impl BaseEffect {
    fn into_native(self) -> libobs_sys::obs_base_effect::Type {
        use libobs_sys::obs_base_effect::*;

        match self {
            Self::Default => OBS_EFFECT_DEFAULT,
            Self::DefaultRect => OBS_EFFECT_DEFAULT_RECT,
            Self::Opaque => OBS_EFFECT_OPAQUE,
            Self::Solid => OBS_EFFECT_SOLID,
            Self::Bicubic => OBS_EFFECT_BICUBIC,
            Self::Lanczos => OBS_EFFECT_LANCZOS,
            Self::BilinearLowres => OBS_EFFECT_BILINEAR_LOWRES,
            Self::PremultipliedAlpha => OBS_EFFECT_PREMULTIPLIED_ALPHA,
            Self::Repeat => OBS_EFFECT_REPEAT,
            Self::Area => OBS_EFFECT_AREA,
        }
    }
}

/// Returns a commonly used base effect.
pub fn base_effect(effect: BaseEffect) -> Option<GsEffect> {
    let raw = unsafe { libobs_sys::obs_get_base_effect(effect.into_native()) };
    if raw.is_null() {
        None
    } else {
        Some(GsEffect { _raw: raw })
    }
}

pub fn default_rect_effect() -> Option<GsEffect> {
    let raw = unsafe { libobs_sys::obs_get_default_rect_effect() };
    if raw.is_null() {
        None
    } else {
        Some(GsEffect { _raw: raw })
    }
}

pub struct GsEffect {
    _raw: *mut libobs_sys::gs_effect_t,
}
