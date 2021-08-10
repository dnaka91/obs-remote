use std::{ffi::c_void, ptr::NonNull};

pub fn add_callback<C: FnMut(Event)>(handler: C) -> Handle<C> {
    let data = unsafe { NonNull::new_unchecked(Box::into_raw(Box::new(handler))) };

    unsafe {
        libobs_sys::obs_frontend_add_event_callback(Some(event_callback::<C>), data.as_ptr() as _)
    };

    Handle { data }
}

pub struct Handle<C: FnMut(Event)> {
    data: NonNull<C>,
}

impl<C: FnMut(Event)> Drop for Handle<C> {
    fn drop(&mut self) {
        unsafe {
            libobs_sys::obs_frontend_remove_event_callback(
                Some(event_callback::<C>),
                self.data.as_ptr() as _,
            );
            Box::from_raw(self.data.as_ptr());
        };
    }
}

unsafe impl<C: FnMut(Event)> Send for Handle<C> {}

unsafe impl<C: FnMut(Event)> Sync for Handle<C> {}

unsafe extern "C" fn event_callback<C: FnMut(Event)>(
    event: libobs_sys::obs_frontend_event::Type,
    private_data: *mut c_void,
) {
    let handler = &mut *private_data.cast::<C>();
    (handler)(Event::from_native(event));
}

#[derive(Clone, Copy, Debug)]
pub enum Event {
    StreamingStarting,
    StreamingStarted,
    StreamingStopping,
    StreamingStopped,
    RecordingStarting,
    RecordingStarted,
    RecordingStopping,
    RecordingStopped,
    SceneChanged,
    SceneListChanged,
    TransitionChanged,
    TransitionStopped,
    TransitionListChanged,
    SceneCollectionChanged,
    SceneCollectionListChanged,
    ProfileChanged,
    ProfileListChanged,
    Exit,
    ReplayBufferStarting,
    ReplayBufferStarted,
    ReplayBufferStopping,
    ReplayBufferStopped,
    StudioModeEnabled,
    StudioModeDisabled,
    PreviewSceneChanged,
    SceneCollectionCleanup,
    FinishedLoading,
    RecordingPaused,
    RecordingUnpaused,
    TransitionDurationChanged,
    ReplayBufferSaved,
    VirtualcamStarted,
    VirtualcamStopped,
    TBarValueChanged,
}

impl Event {
    fn from_native(value: libobs_sys::obs_frontend_event::Type) -> Self {
        use libobs_sys::obs_frontend_event::*;

        match value {
            OBS_FRONTEND_EVENT_STREAMING_STARTING => Self::StreamingStarting,
            OBS_FRONTEND_EVENT_STREAMING_STARTED => Self::StreamingStarted,
            OBS_FRONTEND_EVENT_STREAMING_STOPPING => Self::StreamingStopping,
            OBS_FRONTEND_EVENT_STREAMING_STOPPED => Self::StreamingStopped,
            OBS_FRONTEND_EVENT_RECORDING_STARTING => Self::RecordingStarting,
            OBS_FRONTEND_EVENT_RECORDING_STARTED => Self::RecordingStarted,
            OBS_FRONTEND_EVENT_RECORDING_STOPPING => Self::RecordingStopping,
            OBS_FRONTEND_EVENT_RECORDING_STOPPED => Self::RecordingStopped,
            OBS_FRONTEND_EVENT_SCENE_CHANGED => Self::SceneChanged,
            OBS_FRONTEND_EVENT_SCENE_LIST_CHANGED => Self::SceneListChanged,
            OBS_FRONTEND_EVENT_TRANSITION_CHANGED => Self::TransitionChanged,
            OBS_FRONTEND_EVENT_TRANSITION_STOPPED => Self::TransitionStopped,
            OBS_FRONTEND_EVENT_TRANSITION_LIST_CHANGED => Self::TransitionListChanged,
            OBS_FRONTEND_EVENT_SCENE_COLLECTION_CHANGED => Self::SceneCollectionChanged,
            OBS_FRONTEND_EVENT_SCENE_COLLECTION_LIST_CHANGED => Self::SceneCollectionListChanged,
            OBS_FRONTEND_EVENT_PROFILE_CHANGED => Self::ProfileChanged,
            OBS_FRONTEND_EVENT_PROFILE_LIST_CHANGED => Self::ProfileListChanged,
            OBS_FRONTEND_EVENT_EXIT => Self::Exit,
            OBS_FRONTEND_EVENT_REPLAY_BUFFER_STARTING => Self::ReplayBufferStarting,
            OBS_FRONTEND_EVENT_REPLAY_BUFFER_STARTED => Self::ReplayBufferStarted,
            OBS_FRONTEND_EVENT_REPLAY_BUFFER_STOPPING => Self::ReplayBufferStopping,
            OBS_FRONTEND_EVENT_REPLAY_BUFFER_STOPPED => Self::ReplayBufferStopped,
            OBS_FRONTEND_EVENT_STUDIO_MODE_ENABLED => Self::StudioModeEnabled,
            OBS_FRONTEND_EVENT_STUDIO_MODE_DISABLED => Self::StudioModeDisabled,
            OBS_FRONTEND_EVENT_PREVIEW_SCENE_CHANGED => Self::PreviewSceneChanged,
            OBS_FRONTEND_EVENT_SCENE_COLLECTION_CLEANUP => Self::SceneCollectionCleanup,
            OBS_FRONTEND_EVENT_FINISHED_LOADING => Self::FinishedLoading,
            OBS_FRONTEND_EVENT_RECORDING_PAUSED => Self::RecordingPaused,
            OBS_FRONTEND_EVENT_RECORDING_UNPAUSED => Self::RecordingUnpaused,
            OBS_FRONTEND_EVENT_TRANSITION_DURATION_CHANGED => Self::TransitionDurationChanged,
            OBS_FRONTEND_EVENT_REPLAY_BUFFER_SAVED => Self::ReplayBufferSaved,
            OBS_FRONTEND_EVENT_VIRTUALCAM_STARTED => Self::VirtualcamStarted,
            OBS_FRONTEND_EVENT_VIRTUALCAM_STOPPED => Self::VirtualcamStopped,
            OBS_FRONTEND_EVENT_TBAR_VALUE_CHANGED => Self::TBarValueChanged,
            _ => unreachable!(),
        }
    }
}
