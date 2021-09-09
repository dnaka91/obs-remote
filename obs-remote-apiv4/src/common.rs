use obs::output::Output;

tonic::include_proto!("obs_remote.common");

impl From<obs::source::SourceType> for SourceType {
    fn from(value: obs::source::SourceType) -> Self {
        match value {
            obs::source::SourceType::Input => Self::Input,
            obs::source::SourceType::Filter => Self::Filter,
            obs::source::SourceType::Transition => Self::Transition,
            obs::source::SourceType::Scene => Self::Scene,
            obs::source::SourceType::Unknown(_) => Self::Unknown,
        }
    }
}

impl From<obs::source::MediaState> for MediaState {
    fn from(value: obs::source::MediaState) -> Self {
        match value {
            obs::source::MediaState::None => Self::None,
            obs::source::MediaState::Playing => Self::Playing,
            obs::source::MediaState::Opening => Self::Opening,
            obs::source::MediaState::Buffering => Self::Buffering,
            obs::source::MediaState::Paused => Self::Paused,
            obs::source::MediaState::Stopped => Self::Stopped,
            obs::source::MediaState::Ended => Self::Ended,
            obs::source::MediaState::Error => Self::Error,
            obs::source::MediaState::Unknown(_) => Self::UnknownState,
        }
    }
}

impl From<obs::scene::Alignment> for Alignment {
    fn from(value: obs::scene::Alignment) -> Self {
        Self {
            raw: value.bits(),
            left: value.contains(obs::scene::Alignment::LEFT),
            right: value.contains(obs::scene::Alignment::RIGHT),
            top: value.contains(obs::scene::Alignment::TOP),
            bottom: value.contains(obs::scene::Alignment::BOTTOM),
        }
    }
}

impl From<Alignment> for obs::scene::Alignment {
    fn from(value: Alignment) -> Self {
        Self::from_bits_truncate(value.raw)
    }
}

impl From<((f32, f32), obs::scene::Alignment)> for Position {
    fn from((pos, alignment): ((f32, f32), obs::scene::Alignment)) -> Self {
        Self {
            x: pos.0,
            y: pos.1,
            alignment: Some(alignment.into()),
        }
    }
}

impl From<obs::video::ScaleType> for self::scale::Filter {
    fn from(value: obs::video::ScaleType) -> Self {
        match value {
            obs::video::ScaleType::Disable | obs::video::ScaleType::Unknown(_) => Self::Disabled,
            obs::video::ScaleType::Point => Self::Point,
            obs::video::ScaleType::Bicubic => Self::Bicubic,
            obs::video::ScaleType::Bilinear => Self::Bilinear,
            obs::video::ScaleType::Lanczos => Self::Lanczos,
            obs::video::ScaleType::Area => Self::Area,
        }
    }
}

impl From<self::scale::Filter> for Option<obs::video::ScaleType> {
    fn from(value: self::scale::Filter) -> Self {
        Some(match value {
            self::scale::Filter::Unspecified => return None,
            self::scale::Filter::Disabled => obs::video::ScaleType::Disable,
            self::scale::Filter::Point => obs::video::ScaleType::Point,
            self::scale::Filter::Bicubic => obs::video::ScaleType::Bicubic,
            self::scale::Filter::Bilinear => obs::video::ScaleType::Bilinear,
            self::scale::Filter::Lanczos => obs::video::ScaleType::Lanczos,
            self::scale::Filter::Area => obs::video::ScaleType::Area,
        })
    }
}

impl From<((f32, f32), obs::video::ScaleType)> for Scale {
    fn from((scale, filter): ((f32, f32), obs::video::ScaleType)) -> Self {
        let mut scale = Self {
            x: scale.0,
            y: scale.1,
            ..Default::default()
        };
        scale.set_filter(filter.into());
        scale
    }
}

impl From<(i32, i32, i32, i32)> for Crop {
    fn from((left, top, right, bottom): (i32, i32, i32, i32)) -> Self {
        Self {
            left: left as u32,
            top: top as u32,
            right: right as u32,
            bottom: bottom as u32,
        }
    }
}

impl From<obs::scene::BoundsType> for self::bounds::BoundsType {
    fn from(value: obs::scene::BoundsType) -> Self {
        match value {
            obs::scene::BoundsType::None => Self::None,
            obs::scene::BoundsType::Stretch => Self::Stretch,
            obs::scene::BoundsType::ScaleInner => Self::ScaleInner,
            obs::scene::BoundsType::ScaleOuter => Self::ScaleOuter,
            obs::scene::BoundsType::ScaleToWidth => Self::ScaleToWidth,
            obs::scene::BoundsType::ScaleToHeight => Self::ScaleToHeight,
            obs::scene::BoundsType::MaxOnly => Self::MaxOnly,
            obs::scene::BoundsType::Unknown(_) => Self::Unknown,
        }
    }
}

impl From<self::bounds::BoundsType> for Option<obs::scene::BoundsType> {
    fn from(value: self::bounds::BoundsType) -> Self {
        Some(match value {
            self::bounds::BoundsType::Unspecified => return None,
            self::bounds::BoundsType::None => obs::scene::BoundsType::None,
            self::bounds::BoundsType::Stretch => obs::scene::BoundsType::Stretch,
            self::bounds::BoundsType::ScaleInner => obs::scene::BoundsType::ScaleInner,
            self::bounds::BoundsType::ScaleOuter => obs::scene::BoundsType::ScaleOuter,
            self::bounds::BoundsType::ScaleToWidth => obs::scene::BoundsType::ScaleToWidth,
            self::bounds::BoundsType::ScaleToHeight => obs::scene::BoundsType::ScaleToHeight,
            self::bounds::BoundsType::MaxOnly => obs::scene::BoundsType::MaxOnly,
            self::bounds::BoundsType::Unknown => obs::scene::BoundsType::Unknown(0),
        })
    }
}

impl From<((f32, f32), obs::scene::BoundsType, obs::scene::Alignment)> for Bounds {
    fn from(
        (bounds, ty, alignment): ((f32, f32), obs::scene::BoundsType, obs::scene::Alignment),
    ) -> Self {
        let mut bounds = Self {
            alignment: Some(alignment.into()),
            x: bounds.0,
            y: bounds.1,
            ..Default::default()
        };
        bounds.set_ty(ty.into());
        bounds
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! precondition {
    ($cond:expr, $msg:literal) => {
        if !($cond) {
            return Err(Status::failed_precondition($msg));
        }
    };
    ($cond:expr, $($arg:tt)*) => {
        if !($cond) {
            return Err(Status::failed_precondition(format!($($arg)*)));
        }
    };
}

pub(crate) fn recording_time(output: &Output<'_>) -> u64 {
    output
        .active()
        .then(|| {
            let video = output.video();
            u64::from(video.total_frames()) * video.frame_time()
        })
        .unwrap_or_default()
}

#[allow(clippy::cast_possible_truncation)]
pub(crate) const fn ns_to_timestamp(ns: u64) -> Timecode {
    let ms = ns / 1_000_000;
    let secs = ms / 1000;
    let minutes = secs / 60;

    Timecode {
        hours: (minutes / 60) as u32,
        minutes: (minutes % 60) as u32,
        seconds: (secs % 60) as u32,
        milliseconds: (ms % 1000) as u32,
    }
}

pub(crate) trait DurationExt {
    fn from_proto(duration: prost_types::Duration) -> Self;
    fn into_proto(self) -> prost_types::Duration;
}

impl DurationExt for obs::Duration {
    fn from_proto(duration: prost_types::Duration) -> Self {
        Self::seconds(duration.seconds) + Self::nanoseconds(duration.nanos.into())
    }

    #[allow(clippy::cast_possible_truncation)]
    fn into_proto(self) -> prost_types::Duration {
        prost_types::Duration {
            seconds: self.num_seconds(),
            nanos: (self - Self::seconds(self.num_seconds()))
                .num_nanoseconds()
                .expect("nanoseconds should never overflow") as i32,
        }
    }
}
