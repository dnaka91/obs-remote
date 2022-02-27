use std::{
    convert::TryFrom,
    ffi::{c_void, CString},
    marker::PhantomData,
    mem::ManuallyDrop,
    ptr::NonNull,
};

use anyhow::Context;
use strum::AsRefStr;

use super::calldata::Calldata;
use crate::{
    cstr,
    hotkeys::Hotkey,
    output::Output,
    scene::{Scene, SceneItem},
    source::{Source, Volume},
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

    pub fn connect<C: Fn(&Calldata) + 'static>(&self, signal: T, handler: C) -> Handle {
        let signal = cstr!(signal.as_ref());
        let mut data = Box::new(Box::new(handler) as Box<dyn Fn(&Calldata)>);

        unsafe {
            libobs_sys::signal_handler_connect(
                self.raw.as_ptr(),
                signal.as_ptr(),
                Some(signal_callback),
                ((&mut *data) as *mut Box<dyn Fn(&Calldata)>).cast(),
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
    data: Box<Box<dyn Fn(&Calldata)>>,
}

impl Drop for Handle {
    fn drop(&mut self) {
        unsafe {
            libobs_sys::signal_handler_disconnect(
                self.handler.as_ptr(),
                self.signal.as_ptr(),
                Some(signal_callback),
                ((&mut *self.data) as *mut Box<dyn Fn(&Calldata)>).cast(),
            );
        }
    }
}

unsafe impl Send for Handle {}

unsafe impl Sync for Handle {}

unsafe extern "C" fn signal_callback(param: *mut c_void, data: *mut libobs_sys::calldata_t) {
    let callback = ManuallyDrop::new(Box::from_raw(param.cast::<Box<dyn Fn(&Calldata)>>()));
    (callback)(&Calldata::from_raw(data, false));
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

pub enum GlobalCalldata<'a> {
    SourceCreate(Source<'a>),
    SourceDestroy(Source<'a>),
    SourceRemove(Source<'a>),
    SourceSave(Source<'a>),
    SourceLoad(Source<'a>),
    SourceActivate(Source<'a>),
    SourceDeactivate(Source<'a>),
    SourceShow(Source<'a>),
    SourceHide(Source<'a>),
    SourceAudioActivate(Source<'a>),
    SourceAudioDeactivate(Source<'a>),
    SourceRename {
        source: Source<'a>,
        new_name: String,
        prev_name: String,
    },
    SourceVolume {
        source: Source<'a>,
        volume: Volume,
    },
    SourceVolumeLevel {
        source: Source<'a>,
        level: f64,
        magnitude: f64,
        peak: f64,
    },
    SourceTransitionStart(Source<'a>),
    SourceTransitionVideoStop(Source<'a>),
    SourceTransitionStop(Source<'a>),
    ChannelChange {
        channel: u8,
        source: Source<'a>,
        prev_source: Source<'a>,
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

pub enum OutputCalldata<'a> {
    Start(Output<'a>),
    Stop {
        output: Output<'a>,
        code: i64,
        last_error: String,
    },
    Pause(Output<'a>),
    Unpause(Output<'a>),
    Starting(Output<'a>),
    Stopping(Output<'a>),
    Activate(Output<'a>),
    Deactivate(Output<'a>),
    Reconnect {
        output: Output<'a>,
        timeout_sec: i64,
    },
    ReconnectSuccess(Output<'a>),
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

pub enum SceneCalldata<'a> {
    ItemAdd(Scene<'a>, SceneItem<'a>),
    ItemRemove(Scene<'a>, SceneItem<'a>),
    Reorder(Scene<'a>),
    Refresh(Scene<'a>),
    ItemVisible {
        scene: Scene<'a>,
        item: SceneItem<'a>,
        visible: bool,
    },
    ItemSelect(Scene<'a>, SceneItem<'a>),
    ItemDeselect(Scene<'a>, SceneItem<'a>),
    ItemTransform(Scene<'a>, SceneItem<'a>),
    ItemLocked {
        scene: Scene<'a>,
        item: SceneItem<'a>,
        locked: bool,
    },
}

impl<'a> TryFrom<(SceneSignal, &'a Calldata)> for SceneCalldata<'a> {
    type Error = anyhow::Error;

    fn try_from((signal, data): (SceneSignal, &'a Calldata)) -> Result<Self, Self::Error> {
        let scene = data
            .get_scene()
            .with_context(|| format!("`{}` event missing `scene` data", signal.as_ref()))?;

        Ok(match signal {
            SceneSignal::ItemAdd => {
                let item = data.get_scene_item().with_context(|| {
                    format!("`{}` event missing `scene_item` data", signal.as_ref())
                })?;
                Self::ItemAdd(scene, item)
            }
            SceneSignal::ItemRemove => {
                let item = data.get_scene_item().with_context(|| {
                    format!("`{}` event missing `scene_item` data", signal.as_ref())
                })?;
                Self::ItemRemove(scene, item)
            }
            SceneSignal::Reorder => Self::Reorder(scene),
            SceneSignal::Refresh => Self::Refresh(scene),
            SceneSignal::ItemVisible => {
                let item = data.get_scene_item().with_context(|| {
                    format!("`{}` event missing `scene_item` data", signal.as_ref())
                })?;
                let visible = data.bool("visible").with_context(|| {
                    format!("`{}` event missing `visible` data", signal.as_ref())
                })?;
                Self::ItemVisible {
                    scene,
                    item,
                    visible,
                }
            }
            SceneSignal::ItemSelect => {
                let item = data.get_scene_item().with_context(|| {
                    format!("`{}` event missing `scene_item` data", signal.as_ref())
                })?;
                Self::ItemSelect(scene, item)
            }
            SceneSignal::ItemDeselect => {
                let item = data.get_scene_item().with_context(|| {
                    format!("`{}` event missing `scene_item` data", signal.as_ref())
                })?;
                Self::ItemDeselect(scene, item)
            }
            SceneSignal::ItemTransform => {
                let item = data.get_scene_item().with_context(|| {
                    format!("`{}` event missing `scene_item` data", signal.as_ref())
                })?;
                Self::ItemTransform(scene, item)
            }
            SceneSignal::ItemLocked => {
                let item = data.get_scene_item().with_context(|| {
                    format!("`{}` event missing `scene_item` data", signal.as_ref())
                })?;
                let locked = data.bool("locked").with_context(|| {
                    format!("`{}` event missing `locked` data", signal.as_ref())
                })?;
                Self::ItemLocked {
                    scene,
                    item,
                    locked,
                }
            }
        })
    }
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

pub enum SourceCalldata<'a> {
    Destroy(Source<'a>),
    Remove(Source<'a>),
    Save(Source<'a>),
    Load(Source<'a>),
    Activate(Source<'a>),
    Deactivate(Source<'a>),
    Show(Source<'a>),
    Hide(Source<'a>),
    Mute {
        source: Source<'a>,
        muted: bool,
    },
    PushToMuteChanged {
        source: Source<'a>,
        enabled: bool,
    },
    PushToMuteDelay {
        source: Source<'a>,
        delay: u64,
    },
    PushToTalkChanged {
        source: Source<'a>,
        enabled: bool,
    },
    PushToTalkDelay {
        source: Source<'a>,
        delay: u64,
    },
    Enable {
        source: Source<'a>,
        enabled: bool,
    },
    Rename {
        source: Source<'a>,
        new_name: String,
        prev_name: String,
    },
    Volume {
        source: Source<'a>,
        volume: Volume,
    },
    UpdateProperties(Source<'a>),
    UpdateFlags {
        source: Source<'a>,
        flags: u32,
    },
    AudioSync {
        source: Source<'a>,
        offset: i64,
    },
    AudioMixers {
        source: Source<'a>,
        mixers: u32,
    },
    AudioActivate(Source<'a>),
    AudioDeactivate(Source<'a>),
    FilterAdd {
        source: Source<'a>,
        filter: Source<'a>,
    },
    FilterRemove {
        source: Source<'a>,
        filter: Source<'a>,
    },
    ReorderFilters(Source<'a>),
    TransitionStart(Source<'a>),
    TransitionVideoStop(Source<'a>),
    TransitionStop(Source<'a>),
    MediaPlay(Source<'a>),
    MediaPause(Source<'a>),
    MediaRestart(Source<'a>),
    MediaStopped(Source<'a>),
    MediaNext(Source<'a>),
    MediaPrevious(Source<'a>),
    MediaStarted(Source<'a>),
    MediaEnded(Source<'a>),
}

impl<'a> TryFrom<(SourceSignal, &'a Calldata)> for SourceCalldata<'a> {
    type Error = anyhow::Error;

    fn try_from((signal, data): (SourceSignal, &'a Calldata)) -> Result<Self, Self::Error> {
        let source = data
            .get_source()
            .with_context(|| format!("`{}` event missing `source` data", signal.as_ref()))?;

        Ok(match signal {
            SourceSignal::Destroy => Self::Destroy(source),
            SourceSignal::Remove => Self::Remove(source),
            SourceSignal::Save => Self::Save(source),
            SourceSignal::Load => Self::Load(source),
            SourceSignal::Activate => Self::Activate(source),
            SourceSignal::Deactivate => Self::Deactivate(source),
            SourceSignal::Show => Self::Show(source),
            SourceSignal::Hide => Self::Hide(source),
            SourceSignal::Mute => {
                let muted = data
                    .bool("muted")
                    .with_context(|| format!("`{}` event missing `muted` data", signal.as_ref()))?;
                Self::Mute { source, muted }
            }
            SourceSignal::PushToMuteChanged => {
                let enabled = data.bool("enabled").with_context(|| {
                    format!("`{}` event missing `enabled` data", signal.as_ref())
                })?;
                Self::PushToMuteChanged { source, enabled }
            }
            SourceSignal::PushToMuteDelay => {
                let delay = data
                    .int("delay")
                    .with_context(|| format!("`{}` event missing `delay` data", signal.as_ref()))?
                    as u64;
                Self::PushToMuteDelay { source, delay }
            }
            SourceSignal::PushToTalkChanged => {
                let enabled = data.bool("enabled").with_context(|| {
                    format!("`{}` event missing `enabled` data", signal.as_ref())
                })?;
                Self::PushToTalkChanged { source, enabled }
            }
            SourceSignal::PushToTalkDelay => {
                let delay = data
                    .int("delay")
                    .with_context(|| format!("`{}` event missing `delay` data", signal.as_ref()))?
                    as u64;
                Self::PushToTalkDelay { source, delay }
            }
            SourceSignal::Enable => {
                let enabled = data.bool("enabled").with_context(|| {
                    format!("`{}` event missing `enabled` data", signal.as_ref())
                })?;
                Self::Enable { source, enabled }
            }
            SourceSignal::Rename => {
                let new_name = data.string("new_name").with_context(|| {
                    format!("`{}` event missing `new_name` data", signal.as_ref())
                })?;
                let prev_name = data.string("prev_name").with_context(|| {
                    format!("`{}` event missing `prev_name` data", signal.as_ref())
                })?;
                Self::Rename {
                    source,
                    new_name,
                    prev_name,
                }
            }
            SourceSignal::Volume => {
                let volume = data.float("volume").with_context(|| {
                    format!("`{}` event missing `volume` data", signal.as_ref())
                })?;
                let volume = Volume::Mul(volume as f32);
                Self::Volume { source, volume }
            }
            SourceSignal::UpdateProperties => Self::UpdateProperties(source),
            SourceSignal::UpdateFlags => {
                let flags = data
                    .int("flags")
                    .with_context(|| format!("`{}` event missing `flags` data", signal.as_ref()))?
                    as u32;
                Self::UpdateFlags { source, flags }
            }
            SourceSignal::AudioSync => {
                let offset = data.int("offset").with_context(|| {
                    format!("`{}` event missing `offset` data", signal.as_ref())
                })?;
                Self::AudioSync { source, offset }
            }
            SourceSignal::AudioMixers => {
                let mixers = data
                    .int("mixers")
                    .with_context(|| format!("`{}` event missing `mixers` data", signal.as_ref()))?
                    as u32;
                Self::AudioMixers { source, mixers }
            }
            SourceSignal::AudioActivate => Self::AudioActivate(source),
            SourceSignal::AudioDeactivate => Self::AudioDeactivate(source),
            SourceSignal::FilterAdd => {
                let filter = data.get_filter().with_context(|| {
                    format!("`{}` event missing `filter` data", signal.as_ref())
                })?;
                Self::FilterAdd { source, filter }
            }
            SourceSignal::FilterRemove => {
                let filter = data.get_filter().with_context(|| {
                    format!("`{}` event missing `filter` data", signal.as_ref())
                })?;
                Self::FilterRemove { source, filter }
            }
            SourceSignal::ReorderFilters => Self::ReorderFilters(source),
            SourceSignal::TransitionStart => Self::TransitionStart(source),
            SourceSignal::TransitionVideoStop => Self::TransitionVideoStop(source),
            SourceSignal::TransitionStop => Self::TransitionStop(source),
            SourceSignal::MediaPlay => Self::MediaPlay(source),
            SourceSignal::MediaPause => Self::MediaPause(source),
            SourceSignal::MediaRestart => Self::MediaRestart(source),
            SourceSignal::MediaStopped => Self::MediaStopped(source),
            SourceSignal::MediaNext => Self::MediaNext(source),
            SourceSignal::MediaPrevious => Self::MediaPrevious(source),
            SourceSignal::MediaStarted => Self::MediaStarted(source),
            SourceSignal::MediaEnded => Self::MediaEnded(source),
        })
    }
}
