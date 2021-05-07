use std::ffi::c_void;

pub(crate) fn queue<T>(ty: TaskType, param: T, task: fn(T), wait: bool) {
    struct Param<T> {
        task: fn(T),
        param: T,
    }

    unsafe extern "C" fn callback<T>(param: *mut c_void) {
        let param = Box::from_raw(param.cast::<Param<T>>());
        let t = param.task;
        t(param.param);
    }

    let mut param = Param { task, param };

    unsafe {
        libobs_sys::obs_queue_task(
            ty.to_native(),
            Some(callback::<T>),
            (&mut param as *mut Param<_>).cast(),
            wait,
        )
    };
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum TaskType {
    #[allow(dead_code)]
    Graphics,
    Ui,
}

impl TaskType {
    fn to_native(self) -> libobs_sys::obs_task_type::Type {
        use libobs_sys::obs_task_type::*;

        match self {
            Self::Graphics => OBS_TASK_GRAPHICS,
            Self::Ui => OBS_TASK_UI,
        }
    }
}
