#![allow(clippy::missing_panics_doc)]

use std::{collections::HashMap, sync::Arc};

use log::{debug, info, warn};
use obs::{
    frontend::events::{self, Event as ObsEvent},
    signal::{GlobalSignal, Handle, SignalHandler, SourceSignal},
    source::Source,
};
use parking_lot::Mutex as StdMutex;
use tokio::sync::{broadcast, mpsc, watch, Mutex};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

use self::{event_reply::Event, events_server::Events};

tonic::include_proto!("obs_remote.events");

pub struct Service {
    tx: Arc<Mutex<Option<broadcast::Sender<event_reply::Event>>>>,
}

impl Service {
    #[must_use]
    pub fn new(mut signal: watch::Receiver<()>) -> Self {
        let handler = SignalHandler::get().unwrap();
        let mut handles = Vec::new();
        let extra_handles = Arc::new(StdMutex::new(HashMap::new()));
        let (tx, _) = broadcast::channel(10);

        let event_handle = {
            let tx = tx.clone();
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
            })
        };

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
                            connect_source_signals(&*source, tx.clone())
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
            let tx = tx.clone();
            let extra_handles = Arc::clone(&extra_handles);
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

        debug!("connecting source signals...");

        for source in obs::source::list() {
            handles.extend(connect_source_signals(&source, tx.clone()));
            debug!("connected signals for: {}", source.name());
        }

        debug!("all source signals connected");

        let tx = Arc::new(Mutex::new(Some(tx)));
        let tx2 = Arc::clone(&tx);

        tokio::spawn(async move {
            signal.changed().await.ok();
            drop(event_handle);
            drop(handler);
            drop(handles);

            tokio::task::spawn_blocking(move || {
                extra_handles.lock().clear();
            })
            .await
            .ok();

            tx2.lock().await.take();
        });

        Service { tx }
    }

    async fn subscribe(&self) -> Option<broadcast::Receiver<event_reply::Event>> {
        self.tx
            .lock()
            .await
            .as_ref()
            .map(broadcast::Sender::subscribe)
    }
}

#[tonic::async_trait]
impl Events for Service {
    type ListenStream = ReceiverStream<Result<EventReply, Status>>;

    async fn listen(&self, request: Request<()>) -> Result<Response<Self::ListenStream>, Status> {
        if let Some(mut event_rx) = self.subscribe().await {
            let (tx, rx) = mpsc::channel(10);

            tokio::spawn(async move {
                while let Ok(event) = event_rx.recv().await {
                    info!("sending event to {:?}: {:?}", request.remote_addr(), event);

                    tx.send(Ok(EventReply {
                        stream_timecode: None,
                        rec_timecode: None,
                        event: Some(event),
                    }))
                    .await
                    .ok();
                }
            });

            Ok(Response::new(ReceiverStream::new(rx)))
        } else {
            Err(Status::aborted("server shutting down"))
        }
    }
}

fn recording_filename() -> String {
    let output = obs::frontend::recording::output();
    let settings = output.settings();

    settings
        .item_by_name("url")
        .or_else(|| settings.item_by_name("path"))
        .and_then(|item| item.string())
        .unwrap_or_default()
}

macro_rules! get_data {
    ($data:expr, $signal:expr, $name:literal) => {
        if let Some(data) = $data {
            data
        } else {
            warn!("{:?}: {} missing", $signal, $name);
            return;
        }
    };
}

#[must_use]
fn connect_source_signals(source: &Source, tx: broadcast::Sender<Event>) -> Vec<Handle> {
    let handler = source.signal_handler();
    let mut handles = Vec::new();

    handles.push({
        let tx = tx.clone();
        handler.connect(SourceSignal::Rename, move |data| {
            let source = get_data!(data.get_source(), SourceSignal::Rename, "source");
            let new_name = get_data!(data.string("new_name"), SourceSignal::Rename, "new_name");
            let previous_name =
                get_data!(data.string("prev_name"), SourceSignal::Rename, "prev_name");

            tx.send(Event::SourceRenamed({
                let mut event = SourceRenamed {
                    previous_name,
                    new_name,
                    source_type: Default::default(),
                };
                event.set_source_type(source.ty().into());
                event
            }))
            .ok();
        })
    });

    handles.push({
        handler.connect(SourceSignal::Mute, move |data| {
            let source = get_data!(data.get_source(), SourceSignal::Mute, "source");
            let muted = get_data!(data.bool("muted"), SourceSignal::Mute, "muted");

            tx.send(Event::SourceMuteStateChanged(SourceMuteStateChanged {
                name: source.name(),
                muted,
            }))
            .ok();
        })
    });

    handles
}
