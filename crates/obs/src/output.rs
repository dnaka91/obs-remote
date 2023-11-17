use std::{marker::PhantomData, ptr::NonNull, time::Duration};

use bitflags::bitflags;

use crate::{
    audio::Audio,
    callback::proc::ProcHandler,
    data::Data,
    encoder::Encoder,
    properties::Properties,
    service::Service,
    util::{self, FfiToString, StringToFfi},
    video::Video,
};

pub struct Output<'a> {
    raw: NonNull<libobs_sys::obs_output_t>,
    life: PhantomData<&'a ()>,
}

impl<'a> Drop for Output<'a> {
    fn drop(&mut self) {
        unsafe { libobs_sys::obs_output_release(self.raw.as_ptr()) }
    }
}

impl<'a> Output<'a> {
    pub(crate) fn from_raw(raw: *mut libobs_sys::obs_output_t) -> Self {
        Self {
            raw: unsafe { NonNull::new_unchecked(raw) },
            life: PhantomData,
        }
    }

    pub(crate) fn as_ptr(&self) -> *mut libobs_sys::obs_output_t {
        self.raw.as_ptr()
    }

    /// Get an output by its name.
    pub fn by_name(name: &str) -> Option<Self> {
        let name = name.cstr();
        let raw = unsafe { libobs_sys::obs_get_output_by_name(name.as_ptr()) };
        (!raw.is_null()).then(|| Self::from_raw(raw))
    }

    /// Returns whether the output is active.
    pub fn active(&self) -> bool {
        unsafe { libobs_sys::obs_output_active(self.raw.as_ptr()) }
    }

    // TODO: can ben null?
    /// Returns the audio media context associated with this output.
    pub fn audio(&self) -> Audio<'_> {
        let raw = unsafe { libobs_sys::obs_output_audio(self.raw.as_ptr()) };
        Audio::from_raw(raw)
    }

    /// Specifies whether the output can be paused.
    pub fn can_pause(&self) -> bool {
        unsafe { libobs_sys::obs_output_can_pause(self.raw.as_ptr()) }
    }

    /// Forces the output to stop.
    pub fn force_stop(&self) {
        unsafe { libobs_sys::obs_output_force_stop(self.raw.as_ptr()) };
    }

    /// If delay is active, gets the currently active delay value.
    pub fn active_delay(&self) -> Duration {
        let raw = unsafe { libobs_sys::obs_output_get_active_delay(self.raw.as_ptr()) };
        Duration::from_secs(raw.into())
    }

    // TODO: can ben null?
    /// Returns the current audio encoder associated with this output.
    ///
    /// The idx parameter specifies the audio encoder index. Only used with outputs that have
    /// multiple audio outputs, otherwise the parameter is ignored.
    pub fn audio_encoder(&self, idx: usize) -> Encoder<'_> {
        let raw = unsafe { libobs_sys::obs_output_get_audio_encoder(self.raw.as_ptr(), idx) };
        let raw = unsafe { libobs_sys::obs_encoder_get_ref(raw) };
        Encoder::from_raw(raw)
    }

    pub fn congestion(&self) -> f32 {
        unsafe { libobs_sys::obs_output_get_congestion(self.raw.as_ptr()) }
    }

    pub fn connect_time_ms(&self) -> i32 {
        unsafe { libobs_sys::obs_output_get_connect_time_ms(self.raw.as_ptr()) }
    }

    /// Gets the currently set delay value.
    pub fn delay(&self) -> Duration {
        let raw = unsafe { libobs_sys::obs_output_get_delay(self.raw.as_ptr()) };
        Duration::from_secs(raw.into())
    }

    /// Returns output capability flags.
    pub fn flags(&self) -> Flags {
        Flags::from_bits_truncate(unsafe { libobs_sys::obs_output_get_flags(self.raw.as_ptr()) })
    }

    pub fn frames_dropped(&self) -> u32 {
        unsafe { libobs_sys::obs_output_get_frames_dropped(self.raw.as_ptr()) as u32 }
    }

    /// For video outputs, returns the height of the encoded image.
    pub fn height(&self) -> u32 {
        unsafe { libobs_sys::obs_output_get_height(self.raw.as_ptr()) }
    }

    pub fn id(&self) -> String {
        unsafe { libobs_sys::obs_output_get_id(self.raw.as_ptr()) }.into_string()
    }

    pub fn name(&self) -> String {
        unsafe { libobs_sys::obs_output_get_name(self.raw.as_ptr()) }.into_string()
    }

    pub fn pause_offset(&self) -> u64 {
        unsafe { libobs_sys::obs_output_get_pause_offset(self.raw.as_ptr()) }
    }

    // TODO: can be null?
    /// Gets the current service associated with this output.
    pub fn service(&self) -> Service<'_> {
        let raw = unsafe { libobs_sys::obs_output_get_service(self.raw.as_ptr()) };
        let raw = unsafe { libobs_sys::obs_service_get_ref(raw) };
        Service::from_raw(raw)
    }

    pub fn supported_audio_codecs(&self) -> String {
        unsafe { libobs_sys::obs_output_get_supported_audio_codecs(self.raw.as_ptr()) }
            .into_string()
    }

    pub fn supported_video_codecs(&self) -> String {
        unsafe { libobs_sys::obs_output_get_supported_video_codecs(self.raw.as_ptr()) }
            .into_string()
    }

    pub fn total_bytes(&self) -> u64 {
        unsafe { libobs_sys::obs_output_get_total_bytes(self.raw.as_ptr()) }
    }

    pub fn total_frames(&self) -> u32 {
        unsafe { libobs_sys::obs_output_get_total_frames(self.raw.as_ptr()) as u32 }
    }

    // TODO: can be null?
    /// Returns the current video encoder associated with this output
    pub fn video_encoder(&self) -> Encoder<'_> {
        let raw = unsafe { libobs_sys::obs_output_get_video_encoder(self.raw.as_ptr()) };
        let raw = unsafe { libobs_sys::obs_encoder_get_ref(raw) };
        Encoder::from_raw(raw)
    }

    pub fn width(&self) -> u32 {
        unsafe { libobs_sys::obs_output_get_width(self.raw.as_ptr()) }
    }

    pub fn pause(&self, pause: bool) -> bool {
        unsafe { libobs_sys::obs_output_pause(self.raw.as_ptr(), pause) }
    }

    pub fn paused(&self) -> bool {
        unsafe { libobs_sys::obs_output_paused(self.raw.as_ptr()) }
    }

    /// Returns the property list of an existing output, if any.
    pub fn properties(&self) -> Option<Properties<'_>> {
        let raw = unsafe { libobs_sys::obs_output_properties(self.raw.as_ptr()) };
        if raw.is_null() {
            None
        } else {
            Some(Properties::from_raw(raw))
        }
    }

    pub fn settings(&self) -> Data<'_> {
        Data::from_raw(unsafe { libobs_sys::obs_output_get_settings(self.raw.as_ptr()) })
    }

    pub fn reconnecting(&self) -> bool {
        unsafe { libobs_sys::obs_output_reconnecting(self.raw.as_ptr()) }
    }

    /// Starts the output.
    pub fn start(&self) -> bool {
        unsafe { libobs_sys::obs_output_start(self.raw.as_ptr()) }
    }

    /// Stops the output.
    pub fn stop(&self) {
        unsafe { libobs_sys::obs_output_stop(self.raw.as_ptr()) }
    }

    pub fn video(&self) -> Video<'_> {
        Video::from_raw(unsafe { libobs_sys::obs_output_video(self.raw.as_ptr()) })
    }

    pub fn output_caption_text2(&self, caption: &str, display_duration: f64) {
        let caption = caption.cstr();

        unsafe {
            libobs_sys::obs_output_output_caption_text2(
                self.raw.as_ptr(),
                caption.as_ptr(),
                display_duration,
            )
        };
    }

    pub fn proc_handler(&self) -> ProcHandler<'_> {
        let raw = unsafe { libobs_sys::obs_output_get_proc_handler(self.raw.as_ptr()) };
        ProcHandler::from_raw(raw, false)
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    #[cfg_attr(feature = "serde", serde(transparent))]
    pub struct Flags: u32 {
        const VIDEO = libobs_sys::OBS_OUTPUT_VIDEO;
        const AUDIO = libobs_sys::OBS_OUTPUT_AUDIO;
        const AV = libobs_sys::OBS_OUTPUT_AV;
        const ENCODED = libobs_sys::OBS_OUTPUT_ENCODED;
        const SERVICE = libobs_sys::OBS_OUTPUT_SERVICE;
        const MULTI_TRACK = libobs_sys::OBS_OUTPUT_MULTI_TRACK;
        const CAN_PAUSE = libobs_sys::OBS_OUTPUT_CAN_PAUSE;
        const MULTI_TRACK_AUDIO = libobs_sys::OBS_OUTPUT_MULTI_TRACK_AUDIO;
        const MULTI_TRACK_VIDEO = libobs_sys::OBS_OUTPUT_MULTI_TRACK_VIDEO;
        const MULTI_TRACK_AV = libobs_sys::OBS_OUTPUT_MULTI_TRACK_AV;
    }
}

pub fn list_outputs() -> Vec<Output<'static>> {
    util::list_instances(
        libobs_sys::obs_enum_outputs,
        libobs_sys::obs_output_get_ref,
        Output::from_raw,
    )
}
