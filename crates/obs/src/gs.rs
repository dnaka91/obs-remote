use std::ptr::NonNull;

use bitflags::bitflags;

use crate::graphics::Vec4;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug)]
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
    fn to_native(self) -> libobs_sys::obs_base_effect::Type {
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
pub fn base_effect(effect: BaseEffect) -> Option<Effect> {
    let raw = unsafe { libobs_sys::obs_get_base_effect(effect.to_native()) };
    (!raw.is_null()).then(|| Effect::from_raw(raw))
}

pub struct Effect {
    #[allow(dead_code)]
    raw: NonNull<libobs_sys::gs_effect_t>,
}

impl Effect {
    pub(crate) fn from_raw(raw: *mut libobs_sys::gs_effect_t) -> Self {
        Self {
            raw: unsafe { NonNull::new_unchecked(raw) },
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug)]
pub enum ColorFormat {
    Unknown,
    A8,
    R8,
    Rgba,
    Bgrx,
    Bgra,
    R10G10B10A2,
    Rgba16,
    R16,
    Rgba16f,
    Rgba32f,
    Rg16f,
    Rg32f,
    R16f,
    R32f,
    Dxt1,
    Dxt3,
    Dxt5,
    R8G8,
    RgbaUnorm,
    BgrxUnorm,
    BgraUnorm,
    Rg16,
}

impl ColorFormat {
    fn to_native(self) -> libobs_sys::gs_color_format::Type {
        use libobs_sys::gs_color_format::*;

        match self {
            Self::Unknown => GS_UNKNOWN,
            Self::A8 => GS_A8,
            Self::R8 => GS_R8,
            Self::Rgba => GS_RGBA,
            Self::Bgrx => GS_BGRX,
            Self::Bgra => GS_BGRA,
            Self::R10G10B10A2 => GS_R10G10B10A2,
            Self::Rgba16 => GS_RGBA16,
            Self::R16 => GS_R16,
            Self::Rgba16f => GS_RGBA16F,
            Self::Rgba32f => GS_RGBA32F,
            Self::Rg16f => GS_RG16F,
            Self::Rg32f => GS_RG32F,
            Self::R16f => GS_R16F,
            Self::R32f => GS_R32F,
            Self::Dxt1 => GS_DXT1,
            Self::Dxt3 => GS_DXT3,
            Self::Dxt5 => GS_DXT5,
            Self::R8G8 => GS_R8G8,
            Self::RgbaUnorm => GS_RGBA_UNORM,
            Self::BgrxUnorm => GS_BGRX_UNORM,
            Self::BgraUnorm => GS_BGRA_UNORM,
            Self::Rg16 => GS_RG16,
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug)]
pub enum ZstencilFormat {
    None,
    Z16,
    Z24S8,
    Z32F,
    Z32fS8X24,
}

impl ZstencilFormat {
    fn to_native(self) -> libobs_sys::gs_zstencil_format::Type {
        use libobs_sys::gs_zstencil_format::*;

        match self {
            Self::None => GS_ZS_NONE,
            Self::Z16 => GS_Z16,
            Self::Z24S8 => GS_Z24_S8,
            Self::Z32F => GS_Z32F,
            Self::Z32fS8X24 => GS_Z32F_S8X24,
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug)]
pub enum BlendType {
    Zero,
    One,
    SrcColor,
    InvSrcColor,
    SrcAlpha,
    InvSrcAlpha,
    DstColor,
    InvDstColor,
    DstAlpha,
    InvDstAlpha,
    SrcAlphaSat,
}

impl BlendType {
    fn to_native(self) -> libobs_sys::gs_blend_type::Type {
        use libobs_sys::gs_blend_type::*;

        match self {
            Self::Zero => GS_BLEND_ZERO,
            Self::One => GS_BLEND_ONE,
            Self::SrcColor => GS_BLEND_SRCCOLOR,
            Self::InvSrcColor => GS_BLEND_INVSRCCOLOR,
            Self::SrcAlpha => GS_BLEND_SRCALPHA,
            Self::InvSrcAlpha => GS_BLEND_INVSRCALPHA,
            Self::DstColor => GS_BLEND_DSTCOLOR,
            Self::InvDstColor => GS_BLEND_INVDSTCOLOR,
            Self::DstAlpha => GS_BLEND_DSTALPHA,
            Self::InvDstAlpha => GS_BLEND_INVDSTALPHA,
            Self::SrcAlphaSat => GS_BLEND_SRCALPHASAT,
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug)]
pub enum BlendOpType {
    Add,
    Subtract,
    ReverseSubtract,
    Min,
    Max,
}

impl BlendOpType {
    fn to_native(self) -> libobs_sys::gs_blend_op_type::Type {
        use libobs_sys::gs_blend_op_type::*;

        match self {
            Self::Add => GS_BLEND_OP_ADD,
            Self::Subtract => GS_BLEND_OP_SUBTRACT,
            Self::ReverseSubtract => GS_BLEND_OP_REVERSE_SUBTRACT,
            Self::Min => GS_BLEND_OP_MIN,
            Self::Max => GS_BLEND_OP_MAX,
        }
    }
}

pub struct TexRender {
    raw: NonNull<libobs_sys::gs_texrender_t>,
}

impl Drop for TexRender {
    fn drop(&mut self) {
        unsafe { libobs_sys::gs_texrender_destroy(self.raw.as_ptr()) };
    }
}

impl TexRender {
    pub(crate) fn from_raw(raw: *mut libobs_sys::gs_texrender_t) -> Self {
        Self {
            raw: unsafe { NonNull::new_unchecked(raw) },
        }
    }

    pub fn create(format: ColorFormat, zs_format: ZstencilFormat) -> Self {
        Self::from_raw(unsafe {
            libobs_sys::gs_texrender_create(format.to_native(), zs_format.to_native())
        })
    }

    pub fn begin(&self, size: (u32, u32)) -> bool {
        unsafe { libobs_sys::gs_texrender_begin(self.raw.as_ptr(), size.0, size.1) }
    }

    pub fn end(&self) {
        unsafe { libobs_sys::gs_texrender_end(self.raw.as_ptr()) };
    }

    pub fn reset(&self) {
        unsafe { libobs_sys::gs_texrender_reset(self.raw.as_ptr()) };
    }

    pub fn get_texture(&self) -> Texture {
        Texture::from_raw(unsafe { libobs_sys::gs_texrender_get_texture(self.raw.as_ptr()) })
    }
}

pub struct StageSurface {
    raw: NonNull<libobs_sys::gs_stagesurf_t>,
}

impl Drop for StageSurface {
    fn drop(&mut self) {
        unsafe { libobs_sys::gs_stagesurface_destroy(self.raw.as_ptr()) };
    }
}

impl StageSurface {
    pub(crate) fn from_raw(raw: *mut libobs_sys::gs_stagesurf_t) -> Self {
        Self {
            raw: unsafe { NonNull::new_unchecked(raw) },
        }
    }

    pub fn create(size: (u32, u32), format: ColorFormat) -> Self {
        Self::from_raw(unsafe {
            libobs_sys::gs_stagesurface_create(size.0, size.1, format.to_native())
        })
    }

    pub fn map(&self, mut f: impl FnMut(&[u8])) {
        let mut data = std::ptr::null_mut::<u8>();
        let mut linesize = 0_u32;

        if unsafe {
            libobs_sys::gs_stagesurface_map(
                self.raw.as_ptr(),
                (&mut data) as *mut _,
                (&mut linesize) as *mut _,
            )
        } {
            let linesize = linesize as usize;
            let height = self.height() as usize;

            for i in 0..height {
                let slice = unsafe { std::slice::from_raw_parts(data.add(linesize * i), linesize) };
                f(slice);
            }

            unsafe { libobs_sys::gs_stagesurface_unmap(self.raw.as_ptr()) };
        }
    }

    pub fn width(&self) -> u32 {
        unsafe { libobs_sys::gs_stagesurface_get_width(self.raw.as_ptr()) }
    }

    pub fn height(&self) -> u32 {
        unsafe { libobs_sys::gs_stagesurface_get_height(self.raw.as_ptr()) }
    }
}

pub struct Texture {
    raw: NonNull<libobs_sys::gs_texture_t>,
}

impl Texture {
    pub(crate) fn from_raw(raw: *mut libobs_sys::gs_texture_t) -> Self {
        Self {
            raw: unsafe { NonNull::new_unchecked(raw) },
        }
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug)]
    pub struct ClearFlags: u32 {
        const COLOR = libobs_sys::GS_CLEAR_COLOR;
        const DEPTH = libobs_sys::GS_CLEAR_DEPTH;
        const STENCIL = libobs_sys::GS_CLEAR_STENCIL;
    }
}

pub fn clear(flags: ClearFlags, color: Vec4, depth: f32, stencil: u8) {
    unsafe { libobs_sys::gs_clear(flags.bits(), color.as_ptr(), depth, stencil) };
}

pub fn ortho(left: f32, right: f32, top: f32, bottom: f32, znear: f32, zfar: f32) {
    unsafe { libobs_sys::gs_ortho(left, right, top, bottom, znear, zfar) };
}

pub fn blend_state_push() {
    unsafe { libobs_sys::gs_blend_state_push() };
}

pub fn blend_state_pop() {
    unsafe { libobs_sys::gs_blend_state_pop() };
}

pub fn blend_function(src: BlendType, dst: BlendType) {
    unsafe { libobs_sys::gs_blend_function(src.to_native(), dst.to_native()) };
}

pub fn blend_op(op: BlendOpType) {
    unsafe { libobs_sys::gs_blend_op(op.to_native()) };
}

pub fn stage_texture(dst: &StageSurface, src: &Texture) {
    unsafe { libobs_sys::gs_stage_texture(dst.raw.as_ptr(), src.raw.as_ptr()) };
}

#[inline]
pub fn float_to_u8(f: f32) -> u8 {
    (f * 255.0 + 0.5) as u8
}

#[inline]
pub fn float4_to_u8x4(f: [f32; 4]) -> [u8; 4] {
    [
        float_to_u8(f[0]),
        float_to_u8(f[1]),
        float_to_u8(f[2]),
        float_to_u8(f[3]),
    ]
}

#[inline]
pub fn u8_to_float(u: u8) -> f32 {
    u as f32 / 255.0
}

#[inline]
pub fn u8x4_to_float4(u: [u8; 4]) -> [f32; 4] {
    [
        u8_to_float(u[0]),
        u8_to_float(u[1]),
        u8_to_float(u[2]),
        u8_to_float(u[3]),
    ]
}

#[inline]
pub fn srgb_nonlinear_to_linear(u: f32) -> f32 {
    if u <= 0.04045 {
        u / 12.92
    } else {
        ((u + 0.055) / 1.055).powf(2.4)
    }
}

#[inline]
pub fn float3_srgb_nonlinear_to_linear(f: &mut [f32; 3]) {
    f[0] = srgb_nonlinear_to_linear(f[0]);
    f[1] = srgb_nonlinear_to_linear(f[1]);
    f[2] = srgb_nonlinear_to_linear(f[2]);
}
