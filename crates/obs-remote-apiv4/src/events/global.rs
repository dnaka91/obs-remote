use obs::{
    callback::signal::Handle as SignalHandle,
    frontend::events::{self, Event as ObsEvent, Handle},
};
use parking_lot::Mutex;
use tokio::sync::broadcast;

use super::{event_reply::Event, sources, *};

#[must_use]
pub fn add_callback(tx: broadcast::Sender<Event>) -> Handle {
    #[allow(clippy::match_same_arms)]
    events::add_callback(move |event| match event {
        ObsEvent::StreamingStarting => {
            tx.send(Event::StreamStarting(StreamStarting {})).ok();
        }
        ObsEvent::StreamingStarted => {
            tx.send(Event::StreamStarted(StreamStarted {})).ok();
        }
        ObsEvent::StreamingStopping => {
            tx.send(Event::StreamStopping(StreamStopping {})).ok();
        }
        ObsEvent::StreamingStopped => {
            tx.send(Event::StreamStopped(StreamStopped {})).ok();
        }
        ObsEvent::RecordingStarting => {
            tx.send(Event::RecordingStarting(RecordingStarting {})).ok();
        }
        ObsEvent::RecordingStarted => {
            tx.send(Event::RecordingStarted(RecordingStarted {
                filename: recording_filename(),
            }))
            .ok();
        }
        ObsEvent::RecordingStopping => {
            tx.send(Event::RecordingStopping(RecordingStopping {
                filename: recording_filename(),
            }))
            .ok();
        }
        ObsEvent::RecordingStopped => {
            tx.send(Event::RecordingStopped(RecordingStopped {
                filename: recording_filename(),
            }))
            .ok();
        }
        ObsEvent::SceneChanged => {}
        ObsEvent::SceneListChanged => {}
        ObsEvent::TransitionChanged => {
            let name = obs::frontend::transitions::current().name();

            tx.send(Event::SwitchTransition(SwitchTransition { name }))
                .ok();
        }
        ObsEvent::TransitionStopped => {}
        ObsEvent::TransitionListChanged => {
            let transitions = obs::frontend::transitions::list()
                .into_iter()
                .map(|transition| transition.name())
                .collect();

            tx.send(Event::TransitionListChanged(TransitionListChanged {
                transitions,
            }))
            .ok();
        }
        ObsEvent::SceneCollectionChanged => {
            let name = obs::frontend::scene_collections::current();

            tx.send(Event::SceneCollectionChanged(SceneCollectionChanged {
                name,
            }))
            .ok();
        }
        ObsEvent::SceneCollectionListChanged => {
            let collections = obs::frontend::scene_collections::list();

            tx.send(Event::SceneCollectionListChanged(
                SceneCollectionListChanged { collections },
            ))
            .ok();
        }
        ObsEvent::ProfileChanged => {
            let name = obs::frontend::profiles::current();

            tx.send(Event::ProfileChanged(ProfileChanged { name })).ok();
        }
        ObsEvent::ProfileListChanged => {
            let profiles = obs::frontend::profiles::list();

            tx.send(Event::ProfileListChanged(ProfileListChanged { profiles }))
                .ok();
        }
        ObsEvent::Exit => {
            tx.send(Event::Exiting(Exiting {})).ok();
        }
        ObsEvent::ReplayBufferStarting => {
            tx.send(Event::ReplayStarting(ReplayStarting {})).ok();
        }
        ObsEvent::ReplayBufferStarted => {
            tx.send(Event::ReplayStarted(ReplayStarted {})).ok();
        }
        ObsEvent::ReplayBufferStopping => {
            tx.send(Event::ReplayStopping(ReplayStopping {})).ok();
        }
        ObsEvent::ReplayBufferStopped => {
            tx.send(Event::ReplayStopped(ReplayStopped {})).ok();
        }
        ObsEvent::StudioModeEnabled => {
            tx.send(Event::StudioModeSwitched(StudioModeSwitched {
                enabled: true,
            }))
            .ok();
        }
        ObsEvent::StudioModeDisabled => {
            tx.send(Event::StudioModeSwitched(StudioModeSwitched {
                enabled: false,
            }))
            .ok();
        }
        ObsEvent::PreviewSceneChanged => {}
        ObsEvent::SceneCollectionCleanup => {}
        ObsEvent::FinishedLoading => {}
        ObsEvent::RecordingPaused => {
            tx.send(Event::RecordingPaused(RecordingPaused {})).ok();
        }
        ObsEvent::RecordingUnpaused => {
            tx.send(Event::RecordingResumed(RecordingResumed {})).ok();
        }
        ObsEvent::TransitionDurationChanged => {}
        ObsEvent::ReplayBufferSaved => {}
        ObsEvent::VirtualcamStarted => {
            tx.send(Event::VirtualCamStarted(VirtualCamStarted {})).ok();
        }
        ObsEvent::VirtualcamStopped => {
            tx.send(Event::VirtualCamStopped(VirtualCamStopped {})).ok();
        }
        ObsEvent::TBarValueChanged => {}
        ObsEvent::SceneCollectionChanging => {}
        ObsEvent::ProfileChanging => {}
        ObsEvent::ScriptingShutdown => {}
        ObsEvent::Unknown(_) => {}
    })
}

#[must_use]
pub fn connect_signals(
    tx: broadcast::Sender<Event>,
    extra_handles: Arc<Mutex<HashMap<String, Vec<SignalHandle>>>>,
) -> Vec<SignalHandle> {
    let handler = SignalHandler::get().unwrap();
    let mut handles = Vec::new();

    handles.push({
        let tx = tx.clone();
        let extra_handles = Arc::clone(&extra_handles);
        handler.connect(GlobalSignal::SourceCreate, move |data| {
            if let Some(source) = data.get_source() {
                extra_handles
                    .lock()
                    .entry(source.name())
                    .or_insert_with_key(|name| {
                        debug!("connecting signals to new source `{}`", name);
                        sources::connect_signals(&source, tx.clone())
                    });

                tx.send(Event::SourceCreated(SourceCreated {
                    name: source.name(),
                    ty: format!("{:?}", source.ty()),
                    kind: source.id(),
                    settings: source.settings().to_json(),
                }))
                .ok();
            }
        })
    });

    handles.push({
        handler.connect(GlobalSignal::SourceDestroy, move |data| {
            if let Some(source) = data.get_source() {
                let name = source.name();
                if extra_handles.lock().remove(&name).is_some() {
                    debug!("disconnected signal from old source `{}`", name);
                }

                tx.send(Event::SourceDestroyed(SourceDestroyed {
                    name: source.name(),
                    ty: format!("{:?}", source.ty()),
                    kind: source.id(),
                }))
                .ok();
            }
        })
    });

    handles
}
