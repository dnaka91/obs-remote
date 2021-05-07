use std::ptr::NonNull;

use crate::util::StringConversion;

pub struct Video {
    raw: NonNull<libobs_sys::video_t>,
}

impl Video {
    pub fn get() -> Self {
        Self::from_raw(unsafe { libobs_sys::obs_get_video() })
    }

    pub(crate) fn from_raw(raw: *mut libobs_sys::video_t) -> Self {
        Self {
            raw: unsafe { NonNull::new_unchecked(raw) },
        }
    }

    pub fn total_frames(&self) -> u32 {
        unsafe { libobs_sys::video_output_get_total_frames(self.raw.as_ptr()) }
    }

    pub fn skipped_frames(&self) -> u32 {
        unsafe { libobs_sys::video_output_get_skipped_frames(self.raw.as_ptr()) }
    }

    pub fn frame_time(&self) -> u64 {
        unsafe { libobs_sys::video_output_get_frame_time(self.raw.as_ptr()) }
    }
}

pub struct VideoInfo {
    pub graphics_module: String,
    pub fps: f64,
    pub base_size: (u32, u32),
    pub output_size: (u32, u32),
    pub output_format: Format,
    pub adapter: u32,
    pub gpu_conversion: bool,
    pub colorspace: Colorspace,
    pub range: RangeType,
    pub scale_type: ScaleType,
}

impl VideoInfo {
    pub fn get() -> Option<Self> {
        let mut raw = libobs_sys::obs_video_info::default();
        if !unsafe { libobs_sys::obs_get_video_info((&mut raw) as *mut _) } {
            return None;
        }

        Some(Self {
            graphics_module: raw.graphics_module.into_string(),
            fps: raw.fps_num as f64 / raw.fps_den as f64,
            base_size: (raw.base_width, raw.base_height),
            output_size: (raw.output_width, raw.output_height),
            output_format: Format::from_native(raw.output_format),
            adapter: raw.adapter,
            gpu_conversion: raw.gpu_conversion,
            colorspace: Colorspace::from_native(raw.colorspace),
            range: RangeType::from_native(raw.range),
            scale_type: ScaleType::from_native(raw.scale_type),
        })
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Colorspace {
    Default,
    Cs601,
    Cs709,
    Srgb,
}

impl Colorspace {
    fn from_native(value: libobs_sys::video_colorspace::Type) -> Self {
        use libobs_sys::video_colorspace::*;

        match value {
            VIDEO_CS_DEFAULT => Self::Default,
            VIDEO_CS_601 => Self::Cs601,
            VIDEO_CS_709 => Self::Cs709,
            VIDEO_CS_SRGB => Self::Srgb,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Format {
    None,
    I420,
    Nv12,
    Yvyu,
    Yuy2,
    Uyvy,
    Rgba,
    Bgra,
    Bgrx,
    Y800,
    I444,
    Bgr3,
    I422,
    I40a,
    I42a,
    Yuva,
    Ayuv,
}

impl Format {
    fn from_native(value: libobs_sys::video_format::Type) -> Self {
        use libobs_sys::video_format::*;

        match value {
            VIDEO_FORMAT_NONE => Self::None,
            VIDEO_FORMAT_I420 => Self::I420,
            VIDEO_FORMAT_NV12 => Self::Nv12,
            VIDEO_FORMAT_YVYU => Self::Yvyu,
            VIDEO_FORMAT_YUY2 => Self::Yuy2,
            VIDEO_FORMAT_UYVY => Self::Uyvy,
            VIDEO_FORMAT_RGBA => Self::Rgba,
            VIDEO_FORMAT_BGRA => Self::Bgra,
            VIDEO_FORMAT_BGRX => Self::Bgrx,
            VIDEO_FORMAT_Y800 => Self::Y800,
            VIDEO_FORMAT_I444 => Self::I444,
            VIDEO_FORMAT_BGR3 => Self::Bgr3,
            VIDEO_FORMAT_I422 => Self::I422,
            VIDEO_FORMAT_I40A => Self::I40a,
            VIDEO_FORMAT_I42A => Self::I42a,
            VIDEO_FORMAT_YUVA => Self::Yuva,
            VIDEO_FORMAT_AYUV => Self::Ayuv,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum RangeType {
    Default,
    Partial,
    Full,
}

impl RangeType {
    fn from_native(value: libobs_sys::video_range_type::Type) -> Self {
        use libobs_sys::video_range_type::*;

        match value {
            VIDEO_RANGE_DEFAULT => Self::Default,
            VIDEO_RANGE_PARTIAL => Self::Partial,
            VIDEO_RANGE_FULL => Self::Full,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ScaleType {
    Disable,
    Point,
    Bicubic,
    Bilinear,
    Lanczos,
    Area,
}

impl ScaleType {
    fn from_native(value: libobs_sys::obs_scale_type::Type) -> Self {
        use libobs_sys::obs_scale_type::*;

        match value {
            OBS_SCALE_DISABLE => Self::Disable,
            OBS_SCALE_POINT => Self::Point,
            OBS_SCALE_BICUBIC => Self::Bicubic,
            OBS_SCALE_BILINEAR => Self::Bilinear,
            OBS_SCALE_LANCZOS => Self::Lanczos,
            OBS_SCALE_AREA => Self::Area,
            _ => unreachable!(),
        }
    }
}
