use std::{
    ffi::{c_void, CString},
    marker::PhantomData,
    mem::{self, ManuallyDrop},
    os::raw::{c_char, c_double, c_longlong},
    ptr::{self, NonNull},
};

use strum::AsRefStr;

use crate::{
    cstr, cstr_ptr,
    hotkeys::Hotkey,
    output::Output,
    scene::{Scene, SceneItem},
    source::{Source, Volume},
    util::StringConversion,
    Ref,
};

#[derive(Clone)]
pub struct SignalHandler<T> {
    raw: NonNull<libobs_sys::signal_handler_t>,
    signal: PhantomData<T>,
}

unsafe impl<T> Send for SignalHandler<T> {}

unsafe impl<T> Sync for SignalHandler<T> {}

impl<T: Signal> SignalHandler<T> {
    pub(crate) fn from_raw(raw: *mut libobs_sys::signal_handler_t) -> Self {
        Self {
            raw: unsafe { NonNull::new_unchecked(raw) },
            signal: PhantomData::default(),
        }
    }

    pub fn connect<C: FnMut(&Calldata) + 'static>(&self, signal: T, handler: C) -> Handle {
        let signal = cstr!(signal.as_ref());
        let mut data = Box::new(Box::new(handler) as Box<dyn FnMut(&Calldata)>);

        unsafe {
            libobs_sys::signal_handler_connect(
                self.raw.as_ptr(),
                signal.as_ptr(),
                Some(signal_callback),
                ((&mut *data) as *mut Box<dyn FnMut(&Calldata)>).cast(),
            )
        };

        Handle {
            handler: self.raw,
            signal,
            data,
        }
    }
}

impl SignalHandler<GlobalSignal> {
    pub fn get() -> Option<Self> {
        let raw = unsafe { libobs_sys::obs_get_signal_handler() };
        (!raw.is_null()).then(|| Self::from_raw(raw))
    }
}

pub struct Handle {
    handler: NonNull<libobs_sys::signal_handler_t>,
    signal: CString,
    data: Box<Box<dyn FnMut(&Calldata)>>,
}

impl Drop for Handle {
    fn drop(&mut self) {
        unsafe {
            libobs_sys::signal_handler_disconnect(
                self.handler.as_ptr(),
                self.signal.as_ptr(),
                Some(signal_callback),
                ((&mut *self.data) as *mut Box<dyn FnMut(&Calldata)>).cast(),
            );
        }
    }
}

unsafe impl Send for Handle {}

unsafe impl Sync for Handle {}

unsafe extern "C" fn signal_callback(param: *mut c_void, data: *mut libobs_sys::calldata_t) {
    let mut callback = ManuallyDrop::new(Box::from_raw(param.cast::<Box<dyn FnMut(&Calldata)>>()));
    (callback)(&Calldata::from_raw(data));
}

pub struct Calldata {
    raw: NonNull<libobs_sys::calldata_t>,
}

impl Calldata {
    fn from_raw(raw: *mut libobs_sys::calldata_t) -> Self {
        Self {
            raw: unsafe { NonNull::new_unchecked(raw) },
        }
    }

    pub fn int(&self, name: &str) -> Option<i64> {
        let mut val: c_longlong = 0;
        let success = unsafe {
            libobs_sys::calldata_get_data(
                self.raw.as_ptr(),
                cstr_ptr!(name),
                (&mut val as *mut c_longlong).cast(),
                mem::size_of::<c_longlong>() as u64,
            )
        };

        success.then(|| val as i64)
    }

    pub fn float(&self, name: &str) -> Option<f64> {
        let mut val: c_double = 0.0;
        let success = unsafe {
            libobs_sys::calldata_get_data(
                self.raw.as_ptr(),
                cstr_ptr!(name),
                (&mut val as *mut c_double).cast(),
                mem::size_of::<c_double>() as u64,
            )
        };

        success.then(|| val as f64)
    }

    pub fn bool(&self, name: &str) -> Option<bool> {
        let mut val = false;
        let success = unsafe {
            libobs_sys::calldata_get_data(
                self.raw.as_ptr(),
                cstr_ptr!(name),
                (&mut val as *mut bool).cast(),
                mem::size_of::<bool>() as u64,
            )
        };

        success.then(|| val)
    }

    fn ptr<T>(&self, name: &str) -> Option<NonNull<T>> {
        let mut val = ptr::null_mut::<c_void>();
        let success = unsafe {
            libobs_sys::calldata_get_data(
                self.raw.as_ptr(),
                cstr_ptr!(name),
                (&mut val as *mut *mut c_void).cast(),
                mem::size_of::<*mut c_void>() as u64,
            )
        };

        (success && !val.is_null()).then(|| unsafe { NonNull::new_unchecked(val.cast()) })
    }

    pub fn string(&self, name: &str) -> Option<String> {
        let mut val = ptr::null_mut::<c_char>();
        let success = unsafe {
            libobs_sys::calldata_get_data(
                self.raw.as_ptr(),
                cstr_ptr!(name),
                (&mut val as *mut *mut c_char).cast(),
                mem::size_of::<*mut c_char>() as u64,
            )
        };

        (success && !val.is_null()).then(|| val.into_string())
    }

    pub fn get_source(&self) -> Option<Ref<'_, Self, Source>> {
        self.ptr("source").map(|p| {
            // unsafe { libobs_sys::obs_source_addref(p.as_ptr()) };
            Ref::new(Source::from_raw_no_release(p.as_ptr()))
        })
    }

    pub fn get_filter(&self) -> Option<Ref<'_, Self, Source>> {
        self.ptr("filter").map(|p| {
            unsafe { libobs_sys::obs_source_addref(p.as_ptr()) };
            Ref::new(Source::from_raw(p.as_ptr()))
        })
    }

    pub fn get_scene(&self) -> Option<Ref<'_, Self, Scene>> {
        self.ptr("scene").map(|p| {
            unsafe { libobs_sys::obs_scene_addref(p.as_ptr()) };
            Ref::new(Scene::from_raw(p.as_ptr()))
        })
    }

    pub fn get_scene_item(&self) -> Option<Ref<'_, Self, SceneItem>> {
        self.ptr("item").map(|p| {
            unsafe { libobs_sys::obs_sceneitem_addref(p.as_ptr()) };
            Ref::new(SceneItem::from_raw(p.as_ptr()))
        })
    }

    pub fn get_output(&self) -> Option<Ref<'_, Self, Output>> {
        self.ptr("output").map(|p| {
            unsafe { libobs_sys::obs_output_addref(p.as_ptr()) };
            Ref::new(Output::from_raw(p.as_ptr()))
        })
    }
}

// #[derive(Clone, Copy)]
// pub enum Signal {
//     Global(GlobalSignal),
//     Output(OutputSignal),
//     Scene(SceneSignal),
//     Source(SourceSignal),
// }

// impl AsRef<str> for Signal {
//     fn as_ref(&self) -> &str {
//         match self {
//             Self::Global(global) => global.as_ref(),
//             Self::Output(output) => output.as_ref(),
//             Self::Scene(scene) => scene.as_ref(),
//             Self::Source(source) => source.as_ref(),
//         }
//     }
// }

pub trait Signal: AsRef<str> {}

#[derive(Clone, Copy, AsRefStr)]
#[strum(serialize_all = "snake_case")]
pub enum GlobalSignal {
    SourceCreate,
    SourceDestroy,
    SourceRemove,
    SourceSave,
    SourceLoad,
    SourceActivate,
    SourceDeactivate,
    SourceShow,
    SourceHide,
    SourceAudioActivate,
    SourceAudioDeactivate,
    SourceRename,
    SourceVolume,
    SourceVolumeLevel,
    SourceTransitionStart,
    SourceTransitionVideoStop,
    SourceTransitionStop,
    ChannelChange,
    MasterVolume,
    HotkeyLayoutChange,
    HotkeyRegister,
    HotkeyUnregister,
    HotkeyBindingsChanged,
}

impl Signal for GlobalSignal {}

pub enum GlobalCalldata {
    SourceCreate(Source),
    SourceDestroy(Source),
    SourceRemove(Source),
    SourceSave(Source),
    SourceLoad(Source),
    SourceActivate(Source),
    SourceDeactivate(Source),
    SourceShow(Source),
    SourceHide(Source),
    SourceAudioActivate(Source),
    SourceAudioDeactivate(Source),
    SourceRename {
        source: Source,
        new_name: String,
        prev_name: String,
    },
    SourceVolume {
        source: Source,
        volume: Volume,
    },
    SourceVolumeLevel {
        source: Source,
        level: f64,
        magnitude: f64,
        peak: f64,
    },
    SourceTransitionStart(Source),
    SourceTransitionVideoStop(Source),
    SourceTransitionStop(Source),
    ChannelChange {
        channel: u8,
        source: Source,
        prev_source: Source,
    },
    MasterVolume(Volume),
    HotkeyLayoutChange,
    HotkeyRegister(Hotkey),
    HotkeyUnregister(Hotkey),
    HotkeyBindingsChanged(Hotkey),
}

#[derive(Clone, Copy, AsRefStr)]
#[strum(serialize_all = "snake_case")]
pub enum OutputSignal {
    Start,
    Stop,
    Pause,
    Unpause,
    Starting,
    Stopping,
    Activate,
    Deactivate,
    Reconnect,
    ReconnectSuccess,
}

pub enum OutputCalldata {
    Start(Output),
    Stop {
        output: Output,
        code: i64,
        last_error: String,
    },
    Pause(Output),
    Unpause(Output),
    Starting(Output),
    Stopping(Output),
    Activate(Output),
    Deactivate(Output),
    Reconnect {
        output: Output,
        timeout_sec: i64,
    },
    ReconnectSuccess(Output),
}

impl Signal for OutputSignal {}

#[derive(Clone, Copy, AsRefStr)]
#[strum(serialize_all = "snake_case")]
pub enum SceneSignal {
    ItemAdd,
    ItemRemove,
    Reorder,
    Refresh,
    ItemVisible,
    ItemSelect,
    ItemDeselect,
    ItemTransform,
    ItemLocked,
}

impl Signal for SceneSignal {}

pub enum SceneCalldata {
    ItemAdd(Scene, SceneItem),
    ItemRemove(Scene, SceneItem),
    Reorder(Scene),
    Refresh(Scene),
    ItemVisible {
        scene: Scene,
        item: SceneItem,
        visible: bool,
    },
    ItemSelect(Scene, SceneItem),
    ItemDeselect(Scene, SceneItem),
    ItemTransform(Scene, SceneItem),
    ItemLocked {
        scene: Scene,
        item: SceneItem,
        locked: bool,
    },
}

#[derive(Clone, Copy, Debug, AsRefStr)]
#[strum(serialize_all = "snake_case")]
pub enum SourceSignal {
    Destroy,
    Remove,
    Save,
    Load,
    Activate,
    Deactivate,
    Show,
    Hide,
    Mute,
    PushToMuteChanged,
    PushToMuteDelay,
    PushToTalkChanged,
    PushToTalkDelay,
    Enable,
    Rename,
    Volume,
    UpdateProperties,
    UpdateFlags,
    AudioSync,
    AudioMixers,
    AudioActivate,
    AudioDeactivate,
    FilterAdd,
    FilterRemove,
    ReorderFilters,
    TransitionStart,
    TransitionVideoStop,
    TransitionStop,
    MediaPlay,
    MediaPause,
    MediaRestart,
    MediaStopped,
    MediaNext,
    MediaPrevious,
    MediaStarted,
    MediaEnded,
}

impl Signal for SourceSignal {}

pub enum SourceCalldata {
    Destroy(Source),
    Remove(Source),
    Save(Source),
    Load(Source),
    Activate(Source),
    Deactivate(Source),
    Show(Source),
    Hide(Source),
    Mute {
        source: Source,
        muted: bool,
    },
    PushToMuteChanged {
        source: Source,
        enabled: bool,
    },
    PushToMuteDelay {
        source: Source,
        delay: u64,
    },
    PushToTalkChanged {
        source: Source,
        enabled: bool,
    },
    PushToTalkDelay {
        source: Source,
        delay: u64,
    },
    Enable {
        source: Source,
        enabled: bool,
    },
    Rename {
        source: Source,
        new_name: String,
        prev_name: String,
    },
    Volume {
        source: Source,
        volume: Volume,
    },
    UpdateProperties(Source),
    UpdateFlags {
        source: Source,
        flags: u32,
    },
    AudioSync {
        source: Source,
        offset: i64,
    },
    AudioMixers {
        source: Source,
        mixers: u32,
    },
    AudioActivate(Source),
    AudioDeactivate(Source),
    FilterAdd {
        source: Source,
        filter: Source,
    },
    FilterRemove {
        source: Source,
        filter: Source,
    },
    ReorderFilters(Source),
    TransitionStart(Source),
    TransitionVideoStop(Source),
    TransitionStop(Source),
    MediaPlay(Source),
    MediaPause(Source),
    MediaRestart(Source),
    MediaStopped(Source),
    MediaNext(Source),
    MediaPrevious(Source),
    MediaStarted(Source),
    MediaEnded(Source),
}
