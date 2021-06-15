use std::ptr::NonNull;

use bitflags::bitflags;

use crate::{
    cstr_ptr,
    data::Data,
    util::{self, StringConversion},
};

pub struct Source {
    raw: NonNull<libobs_sys::obs_source_t>,
    release: bool,
}

impl Drop for Source {
    fn drop(&mut self) {
        if self.release {
            unsafe { libobs_sys::obs_source_release(self.raw.as_ptr()) };
            self.release = false;
        }
    }
}

impl Source {
    pub(crate) fn from_raw(raw: *mut libobs_sys::obs_source_t) -> Self {
        Self {
            raw: unsafe { NonNull::new_unchecked(raw) },
            release: true,
        }
    }

    pub(crate) fn from_raw_no_release(raw: *mut libobs_sys::obs_source_t) -> Self {
        Self {
            raw: unsafe { NonNull::new_unchecked(raw) },
            release: false,
        }
    }

    pub(crate) fn as_ptr(&self) -> *mut libobs_sys::obs_source_t {
        self.raw.as_ptr()
    }

    pub fn by_name(name: &str) -> Option<Source> {
        let raw = unsafe { libobs_sys::obs_get_source_by_name(cstr_ptr!(name)) };

        if raw.is_null() {
            None
        } else {
            Some(Self::from_raw(raw))
        }
    }

    pub fn audio_mixers(&self) -> [bool; 6] {
        let mixers = unsafe { libobs_sys::obs_source_get_audio_mixers(self.raw.as_ptr()) };
        [
            mixers & (1 << 0) > 0,
            mixers & (1 << 1) > 0,
            mixers & (1 << 2) > 0,
            mixers & (1 << 3) > 0,
            mixers & (1 << 4) > 0,
            mixers & (1 << 5) > 0,
        ]
    }

    pub fn balance_value(&self) -> f32 {
        unsafe { libobs_sys::obs_source_get_balance_value(self.raw.as_ptr()) }
    }

    pub fn base_height(&self) -> u32 {
        unsafe { libobs_sys::obs_source_get_base_height(self.raw.as_ptr()) }
    }

    pub fn base_width(&self) -> u32 {
        unsafe { libobs_sys::obs_source_get_base_width(self.raw.as_ptr()) }
    }

    pub fn display_name(name: &str) -> String {
        unsafe { libobs_sys::obs_source_get_display_name(cstr_ptr!(name)) }.into_string()
    }

    pub fn filter_by_name(&self, name: &str) -> Option<Self> {
        let raw = unsafe {
            libobs_sys::obs_source_get_filter_by_name(self.raw.as_ptr(), cstr_ptr!(name))
        };

        if raw.is_null() {
            None
        } else {
            Some(Self::from_raw(raw))
        }
    }

    pub fn flags(&self) -> u32 {
        unsafe { libobs_sys::obs_source_get_flags(self.raw.as_ptr()) }
    }

    // TODO: obs_source_get_frame

    pub fn height(&self) -> u32 {
        unsafe { libobs_sys::obs_source_get_height(self.raw.as_ptr()) }
    }

    pub fn icon_type(id: &str) -> IconType {
        IconType::from_native(unsafe { libobs_sys::obs_source_get_icon_type(cstr_ptr!(id)) })
    }

    pub fn id(&self) -> String {
        unsafe { libobs_sys::obs_source_get_id(self.raw.as_ptr()) }.into_string()
    }

    pub fn monitoring_type(&self) -> MonitoringType {
        MonitoringType::from_native(unsafe {
            libobs_sys::obs_source_get_monitoring_type(self.raw.as_ptr())
        })
    }

    pub fn name(&self) -> String {
        unsafe { libobs_sys::obs_source_get_name(self.raw.as_ptr()) }.into_string()
    }

    pub fn output_flags(&self) -> OutputFlags {
        OutputFlags::from_bits_truncate(unsafe {
            libobs_sys::obs_source_get_output_flags(self.raw.as_ptr())
        })
    }

    pub fn settings(&self) -> Data {
        let raw = unsafe { libobs_sys::obs_source_get_settings(self.raw.as_ptr()) };
        Data::from_raw(raw)
    }

    pub fn sync_offset(&self) -> i64 {
        unsafe { libobs_sys::obs_source_get_sync_offset(self.raw.as_ptr()) }
    }

    pub fn ty(&self) -> SourceType {
        SourceType::from_native(unsafe { libobs_sys::obs_source_get_type(self.raw.as_ptr()) })
    }

    pub fn unversioned_id(&self) -> String {
        unsafe { libobs_sys::obs_source_get_unversioned_id(self.raw.as_ptr()) }.into_string()
    }

    pub fn volume(&self) -> f32 {
        unsafe { libobs_sys::obs_source_get_volume(self.raw.as_ptr()) }
    }

    pub fn width(&self) -> u32 {
        unsafe { libobs_sys::obs_source_get_width(self.raw.as_ptr()) }
    }

    pub fn muted(&self) -> bool {
        unsafe { libobs_sys::obs_source_muted(self.raw.as_ptr()) }
    }

    // TODO: Move to transitions area.
    pub fn transition_fixed(&self) -> bool {
        unsafe { libobs_sys::obs_transition_fixed(self.raw.as_ptr()) }
    }

    // TODO: Move to transitions area.
    pub fn transition_time(&self) -> f32 {
        unsafe { libobs_sys::obs_transition_get_time(self.raw.as_ptr()) }
    }

    pub fn filter_count(&self) -> u64 {
        unsafe { libobs_sys::obs_source_filter_count(self.raw.as_ptr()) }
    }

    pub fn update(&self, settings: Data) {
        unsafe { libobs_sys::obs_source_update(self.raw.as_ptr(), settings.as_ptr()) };
    }

    pub fn update_properties(&self) {
        unsafe { libobs_sys::obs_source_update_properties(self.raw.as_ptr()) };
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SourceType {
    Input,
    Filter,
    Transition,
    Scene,
    Unknown(u32),
}

impl SourceType {
    fn from_native(ty: libobs_sys::obs_source_type::Type) -> Self {
        use libobs_sys::obs_source_type::*;

        match ty {
            OBS_SOURCE_TYPE_INPUT => Self::Input,
            OBS_SOURCE_TYPE_FILTER => Self::Filter,
            OBS_SOURCE_TYPE_TRANSITION => Self::Transition,
            OBS_SOURCE_TYPE_SCENE => Self::Scene,
            _ => Self::Unknown(ty as u32),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum IconType {
    Unknown,
    Image,
    Color,
    Slideshow,
    AudioInput,
    AudioOutput,
    DesktopCapture,
    WindowCapture,
    GameCapture,
    Camera,
    Text,
    Media,
    Browser,
    Custom,
}

impl IconType {
    fn from_native(icon_type: libobs_sys::obs_icon_type::Type) -> Self {
        use libobs_sys::obs_icon_type::*;

        match icon_type {
            OBS_ICON_TYPE_UNKNOWN => Self::Unknown,
            OBS_ICON_TYPE_IMAGE => Self::Image,
            OBS_ICON_TYPE_COLOR => Self::Color,
            OBS_ICON_TYPE_SLIDESHOW => Self::Slideshow,
            OBS_ICON_TYPE_AUDIO_INPUT => Self::AudioInput,
            OBS_ICON_TYPE_AUDIO_OUTPUT => Self::AudioOutput,
            OBS_ICON_TYPE_DESKTOP_CAPTURE => Self::DesktopCapture,
            OBS_ICON_TYPE_WINDOW_CAPTURE => Self::WindowCapture,
            OBS_ICON_TYPE_GAME_CAPTURE => Self::GameCapture,
            OBS_ICON_TYPE_CAMERA => Self::Camera,
            OBS_ICON_TYPE_TEXT => Self::Text,
            OBS_ICON_TYPE_MEDIA => Self::Media,
            OBS_ICON_TYPE_BROWSER => Self::Browser,
            OBS_ICON_TYPE_CUSTOM => Self::Custom,
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum MonitoringType {
    None,
    MonitorOnly,
    MonitorAndOutput,
}

impl MonitoringType {
    fn from_native(monitoring_type: libobs_sys::obs_monitoring_type::Type) -> Self {
        use libobs_sys::obs_monitoring_type::*;

        match monitoring_type {
            OBS_MONITORING_TYPE_NONE => Self::None,
            OBS_MONITORING_TYPE_MONITOR_ONLY => Self::MonitorOnly,
            OBS_MONITORING_TYPE_MONITOR_AND_OUTPUT => Self::MonitorAndOutput,
            _ => unreachable!(),
        }
    }
}

bitflags! {
    /// These flags determine what type of data the source outputs and expects.
    pub struct OutputFlags: u32 {
        /// Source has video.
        ///
        /// Unless [`Self::ASYNC_VIDEO`] is specified, the source must include the video_render
        /// callback in the source definition structure.
        const VIDEO = libobs_sys::OBS_SOURCE_VIDEO;
        /// Source has audio.
        ///
        /// Use the obs_source_output_audio function to pass raw audio data, which will be
        /// automatically converted and uploaded. If used with [`Self::ASYNC_VIDEO`], audio will
        /// automatically be synced up to the video output.
        const AUDIO = libobs_sys::OBS_SOURCE_AUDIO;
        /// Async video flag (use [`Self::ASYNC_VIDEO`]).
        const ASYNC = libobs_sys::OBS_SOURCE_ASYNC;
        /// Source passes raw video data via RAM.
        ///
        /// Use the obs_source_output_video function to pass raw video data, which will be
        /// automatically uploaded at the specified timestamp.
        ///
        /// If this flag is specified, it is not necessary to include the video_render callback.
        /// However, if you wish to use that function as well, you must call obs_source_getframe to
        /// get the current frame data, and obs_source_releaseframe to release the data when
        /// complete.
        const ASYNC_VIDEO = libobs_sys::OBS_SOURCE_ASYNC_VIDEO;
        /// Source uses custom drawing, rather that a default effect.
        ///
        /// If this flag is specified, the video_render callback will pass a NULL effect, and
        /// effect-based filters will not use direct rendering.
        const CUSTOM_DRAW = libobs_sys::OBS_SOURCE_CUSTOM_DRAW;
        /// Source supports interaction.
        ///
        /// When this is used, the source will receive interaction events if they provide the
        /// necessary callbacks in the source definition structure.
        const INTERACTION = libobs_sys::OBS_SOURCE_INTERACTION;
        /// Source composites sub-sources.
        ///
        /// When used specifies that the source composites one or more sub-sources. Sources that
        /// render sub-sources must implement the audio_render callback in order to perform custom
        /// mixing of sub-sources.
        ///
        /// This capability flag is always set for transitions.
        const COMPOSITE = libobs_sys::OBS_SOURCE_COMPOSITE;
        /// Source should not be fully duplicated.
        ///
        /// When this is used, specifies that the source should not be fully duplicated, and should
        /// prefer to duplicate via holding references rather than full duplication.
        const DO_NOT_DUPLICATE = libobs_sys::OBS_SOURCE_DO_NOT_DUPLICATE;
        /// Source is deprecated and should not be used.
        const DEPRECATED = libobs_sys::OBS_SOURCE_DEPRECATED;
        /// Source cannot have its audio monitored.
        ///
        /// Specifies that this source may cause a feedback loop if audio is monitored with a device
        /// selected as desktop audio.
        ///
        /// This is used primarily with desktop audio capture sources.
        const DO_NOT_SELF_MONITOR = libobs_sys::OBS_SOURCE_DO_NOT_SELF_MONITOR;
        /// Source type is currently disabled and should not be shown to the user.
        const CAP_DISABLED = libobs_sys::OBS_SOURCE_CAP_DISABLED;
        /// Source type is obsolete (has been updated with new defaults/properties/etc).
        const CAP_OBSOLETE = libobs_sys::OBS_SOURCE_CAP_OBSOLETE;
        /// Source should enable monitoring by default. Monitoring should be set by the frontend if
        /// this flag is set.
        const MONITOR_BY_DEFAULT = libobs_sys::OBS_SOURCE_MONITOR_BY_DEFAULT;
        /// Used internally for audio submixing.
        const SUBMIX = libobs_sys::OBS_SOURCE_SUBMIX;
        /// Source type can be controlled by media controls.
        const CONTROLLABLE_MEDIA = libobs_sys::OBS_SOURCE_CONTROLLABLE_MEDIA;
        /// Source type provides cea708 data.
        const CEA_708 = libobs_sys::OBS_SOURCE_CEA_708;
        /// Source understands SRGB rendering.
        const SRGB = libobs_sys::OBS_SOURCE_SRGB;
    }
}

/// List all available source types (inputs, filters, transitions, etc).
pub fn list_source_types() -> Vec<String> {
    util::list_types(libobs_sys::obs_enum_source_types)
}

/// List all available input source types.
pub fn list_input_types() -> Vec<String> {
    util::list_types(libobs_sys::obs_enum_input_types)
}

/// List all available filter source types.
pub fn list_filter_types() -> Vec<String> {
    util::list_types(libobs_sys::obs_enum_filter_types)
}

/// List all available transition source types.
pub fn list_transition_types() -> Vec<String> {
    util::list_types(libobs_sys::obs_enum_transition_types)
}
