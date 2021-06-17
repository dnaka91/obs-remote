use std::sync::Arc;

use obs::{
    frontend::events,
    signal::{self, Calldata, SignalHandler},
};
use tokio::sync::{broadcast, mpsc, watch, Mutex};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

use self::events_server::Events;

tonic::include_proto!("obs_remote.events");

pub struct Service {
    event: Arc<Mutex<Option<EventInfo>>>,
    source_create: Arc<Mutex<Option<SignalInfo<SourceCreateHandler, SourceCreated>>>>,
    source_destroy: Arc<Mutex<Option<SignalInfo<SourceDestroyHandler, SourceDestroyed>>>>,
}

#[allow(dead_code)]
struct EventInfo {
    handler: events::Handle<EventHandler>,
    tx: broadcast::Sender<events::Event>,
}

#[allow(dead_code)]
struct SignalInfo<H: signal::Callback, T> {
    handler: SignalHandler,
    tx: broadcast::Sender<T>,
    callback_handle: signal::Handle<H>,
}

struct EventHandler {
    tx: broadcast::Sender<events::Event>,
}

impl events::Handler for EventHandler {
    fn handle(&mut self, event: events::Event) {
        self.tx.send(event).ok();
    }
}

struct SourceCreateHandler {
    tx: broadcast::Sender<SourceCreated>,
}

impl signal::Callback for SourceCreateHandler {
    fn call(&mut self, data: &Calldata) {
        if let Some(source) = data.get_source() {
            self.tx
                .send(SourceCreated {
                    name: source.name(),
                    ty: format!("{:?}", source.ty()),
                    kind: source.id(),
                    settings: source.settings().to_json(),
                })
                .ok();
        }
    }
}

struct SourceDestroyHandler {
    tx: broadcast::Sender<SourceDestroyed>,
}

impl signal::Callback for SourceDestroyHandler {
    fn call(&mut self, data: &Calldata) {
        if let Some(source) = data.get_source() {
            self.tx
                .send(SourceDestroyed {
                    name: source.name(),
                    ty: format!("{:?}", source.ty()),
                    kind: source.id(),
                })
                .ok();
        }
    }
}

impl Service {
    #[must_use]
    pub fn new(mut signal: watch::Receiver<()>) -> Self {
        let (tx_event, _) = broadcast::channel(10);
        let handle_event = events::add_callback(EventHandler {
            tx: tx_event.clone(),
        });

        let (source_create, source_destroy) = if let Some(handler) = SignalHandler::get() {
            let created = {
                let (tx, _) = broadcast::channel(10);
                let callback_handle =
                    handler.connect("source_create", SourceCreateHandler { tx: tx.clone() });

                SignalInfo {
                    handler: handler.clone(),
                    tx,
                    callback_handle,
                }
            };

            let destroyed = {
                let (tx, _) = broadcast::channel(10);
                let callback_handle =
                    handler.connect("source_destroy", SourceDestroyHandler { tx: tx.clone() });

                SignalInfo {
                    handler,
                    tx,
                    callback_handle,
                }
            };

            (Some(created), Some(destroyed))
        } else {
            (None, None)
        };

        let event = Arc::new(Mutex::new(Some(EventInfo {
            handler: handle_event,
            tx: tx_event,
        })));
        let source_create = Arc::new(Mutex::new(source_create));
        let source_destroy = Arc::new(Mutex::new(source_destroy));

        let he2 = Arc::clone(&event);
        let sc2 = Arc::clone(&source_create);
        let sd2 = Arc::clone(&source_destroy);

        tokio::spawn(async move {
            signal.changed().await.ok();
            he2.lock().await.take();
            sc2.lock().await.take();
            sd2.lock().await.take();
        });

        Service {
            event,
            source_create,
            source_destroy,
        }
    }
}

#[tonic::async_trait]
impl Events for Service {
    type ListenStream = ReceiverStream<Result<EventReply, Status>>;

    async fn listen(&self, request: Request<()>) -> Result<Response<Self::ListenStream>, Status> {
        use self::event_reply::Event;

        let (tx, rx) = mpsc::channel(5);

        if let Some(event) = self.event.lock().await.as_ref() {
            let mut rx = event.tx.subscribe();
            let tx2 = tx.clone();

            tokio::spawn(async move {
                while let Ok(event) = rx.recv().await {
                    match event {
                        events::Event::SceneCollectionChanged => {
                            let name = obs::frontend::scene_collections::current();

                            tx2.send(Ok(EventReply {
                                event: Some(Event::SceneCollectionChanged(
                                    SceneCollectionChanged { name },
                                )),
                            }))
                            .await
                            .ok();
                        }
                        events::Event::SceneCollectionListChanged => {
                            let collections = obs::frontend::scene_collections::list();

                            tx2.send(Ok(EventReply {
                                event: Some(Event::SceneCollectionListChanged(
                                    SceneCollectionListChanged { collections },
                                )),
                            }))
                            .await
                            .ok();
                        }
                        _ => {}
                    }
                }
            });
        }

        if let Some(source_create) = self.source_create.lock().await.as_ref() {
            let mut rx = source_create.tx.subscribe();
            let tx2 = tx.clone();

            tokio::spawn(async move {
                while let Ok(event) = rx.recv().await {
                    tx2.send(Ok(EventReply {
                        event: Some(Event::SourceCreated(event)),
                    }))
                    .await
                    .ok();
                }
            });
        }

        if let Some(source_destroy) = self.source_destroy.lock().await.as_ref() {
            let mut rx = source_destroy.tx.subscribe();

            tokio::spawn(async move {
                while let Ok(event) = rx.recv().await {
                    tx.send(Ok(EventReply {
                        event: Some(Event::SourceDestroyed(event)),
                    }))
                    .await
                    .ok();
                }
            });
        }

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}
