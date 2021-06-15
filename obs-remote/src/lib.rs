#![deny(rust_2018_idioms, clippy::all, clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(unused_variables)]
#![recursion_limit = "256"]

use std::thread::JoinHandle;

use anyhow::Result;
use log::{error, info, Level};
use obs::{declare_module, logger::ObsLogger, module_use_default_locale, Plugin};
use tokio::sync::watch;
use tonic::transport::Server;

use crate::obs_remote::{
    events::events_server::EventsServer, general::general_server::GeneralServer,
    media_control::media_control_server::MediaControlServer,
    outputs::outputs_server::OutputsServer, profiles::profiles_server::ProfilesServer,
    recording::recording_server::RecordingServer,
    replay_buffer::replay_buffer_server::ReplayBufferServer,
    scene_collections::scene_collections_server::SceneCollectionsServer,
    scene_items::scene_items_server::SceneItemsServer, scenes::scenes_server::ScenesServer,
    sources::sources_server::SourcesServer, streaming::streaming_server::StreamingServer,
    studio_mode::studio_mode_server::StudioModeServer,
    transitions::transitions_server::TransitionsServer,
    virtual_cam::virtual_cam_server::VirtualCamServer,
};

pub mod obs_remote;

struct ObsRemotePlugin {
    handle: Option<JoinHandle<Result<()>>>,
    shutdown: Option<watch::Sender<()>>,
}

impl Plugin for ObsRemotePlugin {
    fn new() -> Self {
        Self {
            handle: None,
            shutdown: None,
        }
    }

    fn load(&mut self) -> bool {
        ObsLogger::init(
            env!("CARGO_PKG_NAME"),
            Level::Warn,
            vec![(env!("CARGO_CRATE_NAME"), Level::Trace)],
        )
        .ok();

        let (shutdown, signal) = watch::channel(());
        let handle = std::thread::spawn(|| {
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async {
                    let v4_ipv4 = run_server(signal.clone(), false);
                    let v4_ipv6 = run_server(signal.clone(), true);
                    let v5_ipv4 = run_server_v5(signal.clone(), false);
                    let v5_ipv6 = run_server_v5(signal, true);

                    tokio::try_join!(v4_ipv4, v4_ipv6, v5_ipv4, v5_ipv6).map(|_| ())
                })
        });

        self.handle = Some(handle);
        self.shutdown = Some(shutdown);

        true
    }

    fn unload(&mut self) {
        if let Some(shutdown) = self.shutdown.take() {
            info!("shutting down server...");
            shutdown.send(()).ok();
        }

        if let Some(handle) = self.handle.take() {
            if let Ok(Err(e)) = handle.join() {
                error!("{}", e);
            }
        }
    }
}

async fn run_server(mut signal: watch::Receiver<()>, ipv6: bool) -> Result<()> {
    let addr = if ipv6 {
        "[::1]:50051"
    } else {
        "127.0.0.1:50051"
    }
    .parse()
    .unwrap();

    info!("OBS Remote server (v4) starting up at {} ...", addr);

    let signal2 = signal.clone();

    let result = Server::builder()
        .add_service(EventsServer::new(crate::obs_remote::events::Service::new(
            signal2,
        )))
        .add_service(GeneralServer::new(crate::obs_remote::general::Service))
        .add_service(MediaControlServer::new(
            crate::obs_remote::media_control::Service,
        ))
        .add_service(OutputsServer::new(crate::obs_remote::outputs::Service))
        .add_service(ProfilesServer::new(crate::obs_remote::profiles::Service))
        .add_service(RecordingServer::new(crate::obs_remote::recording::Service))
        .add_service(ReplayBufferServer::new(
            crate::obs_remote::replay_buffer::Service,
        ))
        .add_service(SceneCollectionsServer::new(
            crate::obs_remote::scene_collections::Service,
        ))
        .add_service(SceneItemsServer::new(
            crate::obs_remote::scene_items::Service,
        ))
        .add_service(ScenesServer::new(crate::obs_remote::scenes::Service))
        .add_service(SourcesServer::new(crate::obs_remote::sources::Service))
        .add_service(StreamingServer::new(crate::obs_remote::streaming::Service))
        .add_service(StudioModeServer::new(
            crate::obs_remote::studio_mode::Service,
        ))
        .add_service(TransitionsServer::new(
            crate::obs_remote::transitions::Service,
        ))
        .add_service(VirtualCamServer::new(
            crate::obs_remote::virtual_cam::Service,
        ))
        .serve_with_shutdown(addr, async {
            signal.changed().await.ok();
        })
        .await;

    if let Err(e) = &result {
        error!("{}", e);
    } else {
        info!("server (v4) shut down");
    }

    result.map_err(Into::into)
}

async fn run_server_v5(mut signal: watch::Receiver<()>, ipv6: bool) -> Result<()> {
    use crate::obs_remote::v5::*;

    let addr = if ipv6 {
        "[::1]:50052"
    } else {
        "127.0.0.1:50052"
    }
    .parse()
    .unwrap();

    info!("OBS Remote server (v5) starting up at {} ...", addr);

    let result = Server::builder()
        .add_service(ConfigServer::new(ConfigService))
        .add_service(EventsServer::new(EventsService))
        .add_service(FiltersServer::new(FiltersService))
        .add_service(GeneralServer::new(GeneralService))
        .add_service(HotkeysServer::new(HotkeysService))
        .add_service(InputsServer::new(InputsService))
        .add_service(MediaInputsServer::new(MediaInputsService))
        .add_service(OutputsServer::new(OutputsService))
        .add_service(ProfilesServer::new(ProfilesService))
        .add_service(ProjectorsServer::new(ProjectorsService))
        .add_service(RecordingServer::new(RecordingService))
        .add_service(ReplayBufferServer::new(ReplayBufferService))
        .add_service(SceneCollectionsServer::new(SceneCollectionsService))
        .add_service(SceneItemsServer::new(SceneItemsService))
        .add_service(ScenesServer::new(ScenesService))
        .add_service(SourcesServer::new(SourcesService))
        .add_service(StreamingServer::new(StreamingService))
        .add_service(TransitionsServer::new(TransitionsService))
        .add_service(VirtualCamServer::new(VirtualCamService))
        .serve_with_shutdown(addr, async {
            signal.changed().await.ok();
        })
        .await;

    if let Err(e) = &result {
        error!("{}", e);
    } else {
        info!("server (v5) shut down");
    }

    result.map_err(Into::into)
}

declare_module!(ObsRemotePlugin);
module_use_default_locale!("en-US");
