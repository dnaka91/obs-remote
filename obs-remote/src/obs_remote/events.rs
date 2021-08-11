#![allow(clippy::missing_panics_doc)]

use std::sync::Arc;

use obs::{
    frontend::events::{self, Event as ObsEvent},
    signal::SignalHandler,
};
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

        let source_create_handle = {
            let tx = tx.clone();
            handler.connect("source_create", move |data| {
                if let Some(source) = data.get_source() {
                    tx.send(Event::SourceCreated(SourceCreated {
                        name: source.name(),
                        ty: format!("{:?}", source.ty()),
                        kind: source.id(),
                        settings: source.settings().to_json(),
                    }))
                    .ok();
                }
            })
        };

        let source_destroy_handle = {
            let tx = tx.clone();
            handler.connect("source_destroy", move |data| {
                if let Some(source) = data.get_source() {
                    tx.send(Event::SourceDestroyed(SourceDestroyed {
                        name: source.name(),
                        ty: format!("{:?}", source.ty()),
                        kind: source.id(),
                    }))
                    .ok();
                }
            })
        };

        let tx = Arc::new(Mutex::new(Some(tx)));
        let tx2 = Arc::clone(&tx);

        tokio::spawn(async move {
            signal.changed().await.ok();
            drop(event_handle);
            drop(handler);
            // drop(handles);
            drop(source_create_handle);
            drop(source_destroy_handle);

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
