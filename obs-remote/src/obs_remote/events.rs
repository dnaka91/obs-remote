#![allow(clippy::missing_panics_doc)]

use std::sync::Arc;

use obs::{frontend::events, signal::SignalHandler};
use tokio::sync::{broadcast, mpsc, watch, Mutex};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

use self::events_server::Events;

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
            events::add_callback(move |event| match event {
                events::Event::SceneCollectionChanged => {
                    let name = obs::frontend::scene_collections::current();

                    tx.send(event_reply::Event::SceneCollectionChanged(
                        SceneCollectionChanged { name },
                    ))
                    .ok();
                }
                events::Event::SceneCollectionListChanged => {
                    let collections = obs::frontend::scene_collections::list();

                    tx.send(event_reply::Event::SceneCollectionListChanged(
                        SceneCollectionListChanged { collections },
                    ))
                    .ok();
                }
                _ => {}
            })
        };

        let source_create_handle = {
            let tx = tx.clone();
            handler.connect("source_create", move |data| {
                if let Some(source) = data.get_source() {
                    tx.send(event_reply::Event::SourceCreated(SourceCreated {
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
                    tx.send(event_reply::Event::SourceDestroyed(SourceDestroyed {
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
