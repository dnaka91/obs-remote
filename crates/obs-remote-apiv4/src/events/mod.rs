#![allow(clippy::missing_panics_doc)]

use std::{collections::HashMap, sync::Arc};

use log::{debug, info};
use obs::callback::signal::{GlobalSignal, SignalHandler};
use parking_lot::Mutex as StdMutex;
use tokio::sync::{broadcast, mpsc, watch, Mutex};
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

use self::events_server::Events;

mod filters;
mod global;
mod sources;
mod transitions;

tonic::include_proto!("obs_remote.events");

pub struct Service {
    tx: Arc<Mutex<Option<broadcast::Sender<event_reply::Event>>>>,
}

impl Service {
    #[must_use]
    pub fn new(mut signal: watch::Receiver<()>) -> Self {
        let extra_handles = Arc::new(StdMutex::new(HashMap::new()));
        let (tx, _) = broadcast::channel(10);

        let event_handle = global::add_callback(tx.clone());
        let mut handles = global::connect_signals(tx.clone(), Arc::clone(&extra_handles));

        debug!("connecting source signals...");

        for source in obs::source::list() {
            handles.extend(sources::connect_signals(&source, tx.clone()));
            debug!("connected signals for: {}", source.name());
        }

        debug!("all source signals connected");

        let tx = Arc::new(Mutex::new(Some(tx)));
        let tx2 = Arc::clone(&tx);

        tokio::spawn(async move {
            signal.changed().await.ok();
            drop(event_handle);
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

#[doc(hidden)]
#[macro_export]
macro_rules! get_data {
    ($data:expr, $signal:expr, $name:literal) => {
        if let Some(data) = $data {
            data
        } else {
            log::warn!("{:?}: {} missing", $signal, $name);
            return;
        }
    };
}
