tonic::include_proto!("obs_remote.common");

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

pub(crate) trait DurationExt {
    fn from_proto(duration: prost_types::Duration) -> Self;
    fn into_proto(self) -> prost_types::Duration;
}
