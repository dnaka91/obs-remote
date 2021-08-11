#![deny(rust_2018_idioms, clippy::all, clippy::pedantic)]
#![allow(
    unused_variables,
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss
)]
#![recursion_limit = "256"]

use std::thread::JoinHandle;

use anyhow::Result;
use log::{error, info, Level};
use obs::{declare_module, logger::ObsLogger, module_use_default_locale, Plugin};
use tokio::sync::watch;
use tonic::transport::Server;

pub mod obs_remote;

macro_rules! new_service {
    ($server:ident, $service:expr) => {
        tonic_web::enable($server::new($service).accept_gzip().send_gzip())
    };
}

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
            vec![
                (env!("CARGO_CRATE_NAME"), Level::Trace),
                ("obs", Level::Trace),
            ],
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
    #[allow(clippy::wildcard_imports)]
    use crate::obs_remote::*;

    let addr = if ipv6 {
        "[::1]:50051"
    } else {
        "127.0.0.1:50051"
    }
    .parse()
    .unwrap();

    let reflection = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(obs_remote::FILE_DESCRIPTOR_SET_V4)
        .build()?;

    info!("OBS Remote server (v4) starting up at {} ...", addr);

    let result = Server::builder()
        .accept_http1(true)
        .add_service(reflection)
        .add_service(new_service!(
            EventsServer,
            EventsService::new(signal.clone())
        ))
        .add_service(new_service!(GeneralServer, GeneralService))
        .add_service(new_service!(MediaControlServer, MediaControlService))
        .add_service(new_service!(OutputsServer, OutputsService))
        .add_service(new_service!(ProfilesServer, ProfilesService))
        .add_service(new_service!(RecordingServer, RecordingService))
        .add_service(new_service!(ReplayBufferServer, ReplayBufferService))
        .add_service(new_service!(
            SceneCollectionsServer,
            SceneCollectionsService
        ))
        .add_service(new_service!(SceneItemsServer, SceneItemsService))
        .add_service(new_service!(ScenesServer, ScenesService))
        .add_service(new_service!(SourcesServer, SourcesService))
        .add_service(new_service!(StreamingServer, StreamingService))
        .add_service(new_service!(StudioModeServer, StudioModeService))
        .add_service(new_service!(TransitionsServer, TransitionsService))
        .add_service(new_service!(VirtualCamServer, VirtualCamService))
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
    #[allow(clippy::wildcard_imports)]
    use crate::obs_remote::v5::*;

    let addr = if ipv6 {
        "[::1]:50052"
    } else {
        "127.0.0.1:50052"
    }
    .parse()
    .unwrap();

    let reflection = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(obs_remote::FILE_DESCRIPTOR_SET_V5)
        .build()?;

    info!("OBS Remote server (v5) starting up at {} ...", addr);

    let result = Server::builder()
        .accept_http1(true)
        .add_service(reflection)
        .add_service(new_service!(ConfigServer, ConfigService))
        .add_service(new_service!(EventsServer, EventsService))
        .add_service(new_service!(FiltersServer, FiltersService))
        .add_service(new_service!(GeneralServer, GeneralService))
        .add_service(new_service!(HotkeysServer, HotkeysService))
        .add_service(new_service!(InputsServer, InputsService))
        .add_service(new_service!(MediaInputsServer, MediaInputsService))
        .add_service(new_service!(OutputsServer, OutputsService))
        .add_service(new_service!(ProfilesServer, ProfilesService))
        .add_service(new_service!(ProjectorsServer, ProjectorsService))
        .add_service(new_service!(RecordingServer, RecordingService))
        .add_service(new_service!(ReplayBufferServer, ReplayBufferService))
        .add_service(new_service!(
            SceneCollectionsServer,
            SceneCollectionsService
        ))
        .add_service(new_service!(SceneItemsServer, SceneItemsService))
        .add_service(new_service!(ScenesServer, ScenesService))
        .add_service(new_service!(SourcesServer, SourcesService))
        .add_service(new_service!(StreamingServer, StreamingService))
        .add_service(new_service!(TransitionsServer, TransitionsService))
        .add_service(new_service!(VirtualCamServer, VirtualCamService))
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
