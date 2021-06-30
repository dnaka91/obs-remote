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

pub(crate) fn recording_time(output: &Output) -> u64 {
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
