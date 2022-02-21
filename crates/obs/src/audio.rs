use std::{marker::PhantomData, ptr::NonNull};

use crate::util::StringConversion;

pub struct Audio<'a> {
    raw: NonNull<libobs_sys::audio_t>,
    life: PhantomData<&'a ()>,
}

impl<'a> Audio<'a> {
    pub(crate) fn from_raw(raw: *mut libobs_sys::audio_t) -> Self {
        Self {
            raw: unsafe { NonNull::new_unchecked(raw) },
            life: PhantomData::default(),
        }
    }

    pub fn active(&self) -> bool {
        unsafe { libobs_sys::audio_output_active(self.raw.as_ptr()) }
    }

    pub fn close(&self) {
        unsafe { libobs_sys::audio_output_close(self.raw.as_ptr()) };
    }

    // TODO: audio_output_connect
    // TODO: audio_output_disconnect

    pub fn block_size(&self) -> u64 {
        unsafe { libobs_sys::audio_output_get_block_size(self.raw.as_ptr()) }
    }

    pub fn channels(&self) -> u64 {
        unsafe { libobs_sys::audio_output_get_channels(self.raw.as_ptr()) }
    }

    pub fn info(&self) -> AudioOutputInfo {
        AudioOutputInfo::from_raw(unsafe { libobs_sys::audio_output_get_info(self.raw.as_ptr()) })
    }

    pub fn planes(&self) -> u64 {
        unsafe { libobs_sys::audio_output_get_planes(self.raw.as_ptr()) }
    }

    pub fn sample_rate(&self) -> u32 {
        unsafe { libobs_sys::audio_output_get_sample_rate(self.raw.as_ptr()) }
    }
}

pub struct AudioOutputInfo {
    pub name: String,
    pub samples_per_sec: u32,
    pub format: Format,
    pub speakers: SpeakerLayout,
}

impl AudioOutputInfo {
    pub(crate) fn from_raw(raw: *const libobs_sys::audio_output_info) -> Self {
        let raw = unsafe { *raw };

        Self {
            name: raw.name.into_string(),
            samples_per_sec: raw.samples_per_sec,
            format: Format::from_native(raw.format),
            speakers: SpeakerLayout::from_native(raw.speakers),
        }
    }
}

#[derive(Debug)]
pub struct AudioInfo {
    pub samples_per_sec: u32,
    pub speakers: SpeakerLayout,
}

impl AudioInfo {
    pub fn get() -> Option<Self> {
        let mut info = libobs_sys::obs_audio_info::default();

        if unsafe { libobs_sys::obs_get_audio_info(&mut info) } {
            Some(Self {
                samples_per_sec: info.samples_per_sec,
                speakers: SpeakerLayout::from_native(info.speakers),
            })
        } else {
            None
        }
    }
}

/// The speaker layout describes where the speakers are located in the room.
///
/// For OBS it indicates:
/// - how many channels are available and
/// - which channels are used for which speakers.
///
/// Standard channel layouts where retrieved from [ffmpeg documentation].
///
/// [ffmpeg documentation]: https://trac.ffmpeg.org/wiki/AudioChannelManipulation
#[derive(Clone, Copy, Debug)]
pub enum SpeakerLayout {
    /// Unknown setting, fallback is [`Self::Stereo`].
    Unknown,
    /// Channels: MONO.
    Mono,
    /// Channels: FL, FR.
    Stereo,
    /// Channels: FL, FR, LFE.
    TwoPointOne,
    /// Channels: FL, FR, FC, RC.
    FourPointZero,
    /// Channels: FL, FR, FC, LFE, RC.
    FourPointOne,
    /// Channels: FL, FR, FC, LFE, RL, RR.
    FivePointOne,
    /// Channels: FL, FR, FC, LFE, RL, RR, SL, SR.
    SevenPointOne,
}

impl SpeakerLayout {
    fn from_native(value: libobs_sys::speaker_layout::Type) -> Self {
        use libobs_sys::speaker_layout::*;

        match value {
            SPEAKERS_UNKNOWN => Self::Unknown,
            SPEAKERS_MONO => Self::Mono,
            SPEAKERS_STEREO => Self::Stereo,
            SPEAKERS_2POINT1 => Self::TwoPointOne,
            SPEAKERS_4POINT0 => Self::FourPointZero,
            SPEAKERS_4POINT1 => Self::FourPointOne,
            SPEAKERS_5POINT1 => Self::FivePointOne,
            SPEAKERS_7POINT1 => Self::SevenPointOne,
            _ => unreachable!(),
        }
    }
}

pub enum Format {
    Unknown,
    U8Bit,
    SixteenBit,
    ThirtyTwoBit,
    Float,
    U8BitPlanar,
    SixteenBitPlanar,
    ThirtyTwoBitPlanar,
    FloatPlanar,
}

impl Format {
    fn from_native(value: libobs_sys::audio_format::Type) -> Self {
        use libobs_sys::audio_format::*;

        match value {
            AUDIO_FORMAT_UNKNOWN => Self::Unknown,
            AUDIO_FORMAT_U8BIT => Self::U8Bit,
            AUDIO_FORMAT_16BIT => Self::SixteenBit,
            AUDIO_FORMAT_32BIT => Self::ThirtyTwoBit,
            AUDIO_FORMAT_FLOAT => Self::Float,
            AUDIO_FORMAT_U8BIT_PLANAR => Self::U8BitPlanar,
            AUDIO_FORMAT_16BIT_PLANAR => Self::SixteenBitPlanar,
            AUDIO_FORMAT_32BIT_PLANAR => Self::ThirtyTwoBitPlanar,
            AUDIO_FORMAT_FLOAT_PLANAR => Self::FloatPlanar,
            _ => unreachable!(),
        }
    }
}

/// Convert a volume from mul/amplitude to decibel.
pub fn mul_to_db(volume: f32) -> f32 {
    unsafe { libobs_sys::obs_mul_to_db(volume) }
}

/// Convert a volume from decibel back to mul/amplitude.
pub fn db_to_mul(volume: f32) -> f32 {
    unsafe { libobs_sys::obs_db_to_mul(volume) }
}
