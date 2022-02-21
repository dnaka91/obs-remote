#![allow(dead_code)]

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

pub(crate) fn in_task_thread(ty: TaskType) -> bool {
    unsafe { libobs_sys::obs_in_task_thread(ty.to_native()) }
}

pub(crate) fn wait_for_destroy_queue() -> bool {
    unsafe { libobs_sys::obs_wait_for_destroy_queue() }
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum TaskType {
    Audio,
    Destroy,
    Graphics,
    Ui,
}

impl TaskType {
    fn to_native(self) -> libobs_sys::obs_task_type::Type {
        use libobs_sys::obs_task_type::*;

        match self {
            Self::Audio => OBS_TASK_AUDIO,
            Self::Destroy => OBS_TASK_DESTROY,
            Self::Graphics => OBS_TASK_GRAPHICS,
            Self::Ui => OBS_TASK_UI,
        }
    }
}
