use std::{
    fmt::{self, Display},
    marker::PhantomData,
    ptr::NonNull,
};

use bitflags::bitflags;
use time::Duration;

use crate::{
    callback::signal::{SignalHandler, SourceSignal},
    data::Data,
    filter::Filter,
    properties::Properties,
    util::{self, FfiToString, StringToFfi},
};

#[derive(PartialEq, Eq)]
pub struct Source<'a> {
    raw: NonNull<libobs_sys::obs_source_t>,
    life: PhantomData<&'a ()>,
    release: bool,
}

impl<'a> Drop for Source<'a> {
    fn drop(&mut self) {
        if self.release {
            unsafe { libobs_sys::obs_source_release(self.raw.as_ptr()) };
            self.release = false;
        }
    }
}

impl<'a> Source<'a> {
    pub(crate) fn from_raw(raw: *mut libobs_sys::obs_source_t) -> Self {
        Self {
            raw: unsafe { NonNull::new_unchecked(raw) },
            life: PhantomData::default(),
            release: true,
        }
    }

    pub(crate) fn from_raw_no_release(raw: *mut libobs_sys::obs_source_t) -> Self {
        Self {
            raw: unsafe { NonNull::new_unchecked(raw) },
            life: PhantomData::default(),
            release: false,
        }
    }

    pub(crate) fn as_ptr(&self) -> *mut libobs_sys::obs_source_t {
        self.raw.as_ptr()
    }

    pub fn by_name(name: &str) -> Option<Self> {
        let name = name.cstr();
        let raw = unsafe { libobs_sys::obs_get_source_by_name(name.as_ptr()) };

        (!raw.is_null()).then(|| Self::from_raw(raw))
    }

    pub fn transition_by_name(name: &str) -> Option<Self> {
        let name = name.cstr();
        let raw = unsafe { libobs_sys::obs_get_transition_by_name(name.as_ptr()) };

        (!raw.is_null()).then(|| Self::from_raw(raw))
    }

    pub fn by_output_channel(channel: u32) -> Option<Self> {
        let raw = unsafe { libobs_sys::obs_get_output_source(channel) };
        (!raw.is_null()).then(|| Self::from_raw(raw))
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

    pub fn set_audio_mixers(&mut self, mixers: [bool; 6]) {
        let mixers = u32::from(mixers[0])
            | u32::from(mixers[1]) << 1
            | u32::from(mixers[2]) << 2
            | u32::from(mixers[3]) << 3
            | u32::from(mixers[4]) << 4
            | u32::from(mixers[5]) << 5;
        unsafe { libobs_sys::obs_source_set_audio_mixers(self.raw.as_ptr(), mixers) };
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

    pub fn filters(&self) -> Vec<Self> {
        util::list_instances_of(
            self.raw.as_ptr(),
            libobs_sys::obs_source_enum_filters,
            libobs_sys::obs_source_get_ref,
            Self::from_raw,
        )
    }

    pub fn filter_by_name(&self, name: &str) -> Option<Filter<'_>> {
        let name = name.cstr();
        let raw =
            unsafe { libobs_sys::obs_source_get_filter_by_name(self.raw.as_ptr(), name.as_ptr()) };

        if raw.is_null() {
            None
        } else {
            Some(Filter::from_raw(raw, self))
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
        let id = id.cstr();

        IconType::from_native(unsafe { libobs_sys::obs_source_get_icon_type(id.as_ptr()) })
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

    pub fn set_name(&self, name: &str) {
        let name = name.cstr();

        unsafe { libobs_sys::obs_source_set_name(self.raw.as_ptr(), name.as_ptr()) };
    }

    pub fn output_flags(&self) -> OutputFlags {
        OutputFlags::from_bits_truncate(unsafe {
            libobs_sys::obs_source_get_output_flags(self.raw.as_ptr())
        })
    }

    pub fn settings(&self) -> Data<'_> {
        let raw = unsafe { libobs_sys::obs_source_get_settings(self.raw.as_ptr()) };
        Data::from_raw(raw)
    }

    pub fn private_settings(&self) -> Data<'_> {
        let raw = unsafe { libobs_sys::obs_source_get_private_settings(self.raw.as_ptr()) };
        Data::from_raw(raw)
    }

    pub fn sync_offset(&self) -> Duration {
        Duration::nanoseconds(unsafe { libobs_sys::obs_source_get_sync_offset(self.raw.as_ptr()) })
    }

    pub fn set_sync_offset(&mut self, offset: Duration) {
        // TODO: This conversions will always fail.
        let offset = i64::try_from(offset.whole_nanoseconds())
            .or_else(|_| i64::try_from(offset.whole_microseconds() * 1_000))
            .or_else(|_| i64::try_from(offset.whole_milliseconds() * 1_000_000))
            .unwrap_or_else(|_| offset.whole_seconds().saturating_mul(1_000_000_000));

        unsafe { libobs_sys::obs_source_set_sync_offset(self.raw.as_ptr(), offset) };
    }

    pub fn ty(&self) -> SourceType {
        SourceType::from_native(unsafe { libobs_sys::obs_source_get_type(self.raw.as_ptr()) })
    }

    pub fn unversioned_id(&self) -> String {
        unsafe { libobs_sys::obs_source_get_unversioned_id(self.raw.as_ptr()) }.into_string()
    }

    pub fn volume(&self) -> Volume {
        Volume::Mul(unsafe { libobs_sys::obs_source_get_volume(self.raw.as_ptr()) })
    }

    pub fn set_volume(&self, volume: Volume) {
        unsafe { libobs_sys::obs_source_set_volume(self.raw.as_ptr(), volume.as_mul()) };
    }

    pub fn width(&self) -> u32 {
        unsafe { libobs_sys::obs_source_get_width(self.raw.as_ptr()) }
    }

    pub fn muted(&self) -> bool {
        unsafe { libobs_sys::obs_source_muted(self.raw.as_ptr()) }
    }

    pub fn set_muted(&self, muted: bool) {
        unsafe { libobs_sys::obs_source_set_muted(self.raw.as_ptr(), muted) };
    }

    // TODO: Move to transitions area.
    pub fn transition_fixed(&self) -> bool {
        unsafe { libobs_sys::obs_transition_fixed(self.raw.as_ptr()) }
    }

    // TODO: Move to transitions area.
    pub fn transition_time(&self) -> f32 {
        unsafe { libobs_sys::obs_transition_get_time(self.raw.as_ptr()) }
    }

    pub fn filter_count(&self) -> usize {
        unsafe { libobs_sys::obs_source_filter_count(self.raw.as_ptr()) }
    }

    pub fn update(&self, settings: Data<'_>) {
        unsafe { libobs_sys::obs_source_update(self.raw.as_ptr(), settings.as_ptr()) };
    }

    pub fn update_properties(&self) {
        unsafe { libobs_sys::obs_source_update_properties(self.raw.as_ptr()) };
    }

    pub fn reset_settings(&self, settings: Data<'_>) {
        unsafe { libobs_sys::obs_source_reset_settings(self.raw.as_ptr(), settings.as_ptr()) };
    }

    pub fn is_group(&self) -> bool {
        unsafe { libobs_sys::obs_source_is_group(self.raw.as_ptr()) }
    }

    pub fn remove(&self) {
        unsafe { libobs_sys::obs_source_remove(self.raw.as_ptr()) };
    }

    pub fn active(&self) -> bool {
        unsafe { libobs_sys::obs_source_active(self.raw.as_ptr()) }
    }

    pub fn enabled(&self) -> bool {
        unsafe { libobs_sys::obs_source_enabled(self.raw.as_ptr()) }
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        unsafe { libobs_sys::obs_source_set_enabled(self.raw.as_ptr(), enabled) };
    }

    pub fn audio_active(&self) -> bool {
        unsafe { libobs_sys::obs_source_audio_active(self.raw.as_ptr()) }
    }

    pub fn showing(&self) -> bool {
        unsafe { libobs_sys::obs_source_showing(self.raw.as_ptr()) }
    }

    pub fn inc_showing(&self) {
        unsafe { libobs_sys::obs_source_inc_showing(self.raw.as_ptr()) };
    }

    pub fn dec_showing(&self) {
        unsafe { libobs_sys::obs_source_dec_showing(self.raw.as_ptr()) };
    }

    pub fn video_render(&self) {
        unsafe { libobs_sys::obs_source_video_render(self.raw.as_ptr()) };
    }

    // --- Media controls ---

    pub fn media_play_pause(&self, pause: bool) {
        unsafe { libobs_sys::obs_source_media_play_pause(self.raw.as_ptr(), pause) };
    }

    pub fn media_restart(&self) {
        unsafe { libobs_sys::obs_source_media_restart(self.raw.as_ptr()) };
    }

    pub fn media_stop(&self) {
        unsafe { libobs_sys::obs_source_media_stop(self.raw.as_ptr()) };
    }

    pub fn media_next(&self) {
        unsafe { libobs_sys::obs_source_media_next(self.raw.as_ptr()) };
    }

    pub fn media_previous(&self) {
        unsafe { libobs_sys::obs_source_media_previous(self.raw.as_ptr()) };
    }

    pub fn media_duration(&self) -> Duration {
        Duration::milliseconds(unsafe {
            libobs_sys::obs_source_media_get_duration(self.raw.as_ptr())
        })
    }

    pub fn media_time(&self) -> Duration {
        Duration::milliseconds(unsafe { libobs_sys::obs_source_media_get_time(self.raw.as_ptr()) })
    }

    pub fn media_set_time(&self, duration: Duration) {
        let ms = i64::try_from(duration.whole_milliseconds())
            .unwrap_or_else(|_| duration.whole_seconds().saturating_mul(1000));

        unsafe { libobs_sys::obs_source_media_set_time(self.raw.as_ptr(), ms) };
    }

    pub fn media_state(&self) -> MediaState {
        MediaState::from_native(unsafe {
            libobs_sys::obs_source_media_get_state(self.raw.as_ptr())
        })
    }

    pub fn media_started(&self) {
        unsafe { libobs_sys::obs_source_media_started(self.raw.as_ptr()) };
    }

    pub fn media_ended(&self) {
        unsafe { libobs_sys::obs_source_media_ended(self.raw.as_ptr()) };
    }

    pub fn signal_handler(&self) -> SignalHandler<SourceSignal> {
        SignalHandler::from_raw(unsafe {
            libobs_sys::obs_source_get_signal_handler(self.raw.as_ptr())
        })
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SourceType {
    Input,
    Filter,
    Transition,
    Scene,
    Unknown(u32),
}

impl SourceType {
    fn from_native(value: libobs_sys::obs_source_type::Type) -> Self {
        use libobs_sys::obs_source_type::*;

        match value {
            OBS_SOURCE_TYPE_INPUT => Self::Input,
            OBS_SOURCE_TYPE_FILTER => Self::Filter,
            OBS_SOURCE_TYPE_TRANSITION => Self::Transition,
            OBS_SOURCE_TYPE_SCENE => Self::Scene,
            #[allow(clippy::unnecessary_cast)]
            _ => Self::Unknown(value as u32),
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
    ProcessAudioOutput,
    UnknownValue(u32),
}

impl IconType {
    fn from_native(ty: libobs_sys::obs_icon_type::Type) -> Self {
        use libobs_sys::obs_icon_type::*;

        match ty {
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
            OBS_ICON_TYPE_PROCESS_AUDIO_OUTPUT => Self::ProcessAudioOutput,
            _ => Self::UnknownValue(ty as _),
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug)]
pub enum MonitoringType {
    None,
    MonitorOnly,
    MonitorAndOutput,
}

impl MonitoringType {
    fn from_native(value: libobs_sys::obs_monitoring_type::Type) -> Self {
        use libobs_sys::obs_monitoring_type::*;

        match value {
            OBS_MONITORING_TYPE_NONE => Self::None,
            OBS_MONITORING_TYPE_MONITOR_ONLY => Self::MonitorOnly,
            OBS_MONITORING_TYPE_MONITOR_AND_OUTPUT => Self::MonitorAndOutput,
            _ => unreachable!(),
        }
    }
}

bitflags! {
    /// These flags determine what type of data the source outputs and expects.
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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
        /// Source type prefers not to have its properties shown on creation (prefers to rely on
        /// defaults first).
        const CAP_DONT_SHOW_PROPERTIES = libobs_sys::OBS_SOURCE_CAP_DONT_SHOW_PROPERTIES;
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

pub fn list_input_types2() -> Vec<(String, String)> {
    use std::{os::raw::c_char, ptr};

    let mut id = ptr::null::<c_char>();
    let mut unversioned_id = ptr::null::<c_char>();
    let raw = (&mut id) as *mut _;
    let unversioned_raw = (&mut unversioned_id) as *mut _;
    let mut idx = 0;
    let mut values = Vec::new();

    while unsafe { libobs_sys::obs_enum_input_types2(idx, raw, unversioned_raw) } {
        values.push((id.into_string(), unversioned_id.into_string()));
        idx += 1;
    }

    values
}

/// List all available filter source types.
pub fn list_filter_types() -> Vec<String> {
    util::list_types(libobs_sys::obs_enum_filter_types)
}

/// List all available transition source types.
pub fn list_transition_types() -> Vec<String> {
    util::list_types(libobs_sys::obs_enum_transition_types)
}

pub fn list() -> Vec<Source<'static>> {
    util::list_instances(
        libobs_sys::obs_enum_sources,
        libobs_sys::obs_source_get_ref,
        Source::from_raw,
    )
}

pub fn display_name(id: &str) -> String {
    let id = id.cstr();

    unsafe { libobs_sys::obs_source_get_display_name(id.as_ptr()) }.into_string()
}

pub fn output_flags(id: &str) -> OutputFlags {
    let id = id.cstr();

    OutputFlags::from_bits_truncate(unsafe { libobs_sys::obs_get_source_output_flags(id.as_ptr()) })
}

pub fn defaults(id: &str) -> Option<Data<'static>> {
    let id = id.cstr();
    let raw = unsafe { libobs_sys::obs_get_source_defaults(id.as_ptr()) };
    (!raw.is_null()).then(|| {
        unsafe { libobs_sys::obs_data_addref(raw) };
        Data::from_raw(raw)
    })
}

pub fn properties(id: &str) -> Option<Properties<'static>> {
    let id = id.cstr();
    let raw = unsafe { libobs_sys::obs_get_source_properties(id.as_ptr()) };
    (!raw.is_null()).then(|| Properties::from_raw(raw))
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug)]
pub enum Volume {
    Mul(f32),
    Db(f32),
}

impl Volume {
    pub fn as_mul(self) -> f32 {
        match self {
            Self::Mul(v) => v,
            Self::Db(v) => unsafe { libobs_sys::obs_db_to_mul(v) },
        }
    }

    pub fn as_db(self) -> f32 {
        match self {
            Self::Mul(v) => unsafe { libobs_sys::obs_mul_to_db(v) },
            Self::Db(v) => v,
        }
    }

    #[must_use]
    pub fn into_mul(self) -> Self {
        match self {
            Self::Mul(_) => self,
            Self::Db(_) => Self::Db(self.as_db()),
        }
    }

    #[must_use]
    pub fn into_db(self) -> Self {
        match self {
            Self::Mul(_) => Self::Mul(self.as_mul()),
            Self::Db(_) => self,
        }
    }
}

impl Display for Volume {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Mul(v) => write!(f, "{:.0}%", v * 100.0),
            Self::Db(v) => write!(f, "{:.1} dB", v),
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MediaState {
    None,
    Playing,
    Opening,
    Buffering,
    Paused,
    Stopped,
    Ended,
    Error,
    Unknown(u32),
}

impl MediaState {
    fn from_native(value: libobs_sys::obs_media_state::Type) -> Self {
        use libobs_sys::obs_media_state::*;

        match value {
            OBS_MEDIA_STATE_NONE => Self::None,
            OBS_MEDIA_STATE_PLAYING => Self::Playing,
            OBS_MEDIA_STATE_OPENING => Self::Opening,
            OBS_MEDIA_STATE_BUFFERING => Self::Buffering,
            OBS_MEDIA_STATE_PAUSED => Self::Paused,
            OBS_MEDIA_STATE_STOPPED => Self::Stopped,
            OBS_MEDIA_STATE_ENDED => Self::Ended,
            OBS_MEDIA_STATE_ERROR => Self::Error,
            #[allow(clippy::unnecessary_cast)]
            _ => Self::Unknown(value as u32),
        }
    }
}
