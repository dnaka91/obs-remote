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
use log::{error, info, warn, Level};
use obs::{
    declare_module,
    logger::{self, ObsLogger},
    module_use_default_locale, Plugin,
};
use tokio::sync::watch;
use tonic::{codec::CompressionEncoding, transport::Server};

macro_rules! new_service {
    ($server:ident, $service:expr) => {
        $server::new($service)
            .accept_compressed(CompressionEncoding::Gzip)
            .send_compressed(CompressionEncoding::Gzip)
    };
}

const MIN_VERSION: obs::Version = obs::Version {
    major: 28,
    minor: 0,
    patch: 0,
};

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
        if !init_logger() {
            return false;
        }

        let version = obs::obs_version();
        if version < MIN_VERSION {
            warn!("expected OBS v{MIN_VERSION}, but running in v{version}. Some APIs might fail");
        }

        let (shutdown, signal) = watch::channel(());
        let handle = std::thread::spawn(|| {
            tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .unwrap()
                .block_on(async {
                    let ipv4 = run_server(signal.clone(), false);
                    let ipv6 = run_server(signal, true);

                    tokio::try_join!(ipv4, ipv6).map(|_| ())
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

fn init_logger() -> bool {
    let result = ObsLogger::init(
        env!("CARGO_PKG_NAME"),
        Level::Warn,
        vec![
            (env!("CARGO_CRATE_NAME"), Level::Trace),
            ("obs", Level::Trace),
            ("obs_remote_apiv5", Level::Trace),
        ],
    );

    if let Err(e) = result {
        logger::blog(Level::Error, &format!("failed setting up logger: {e:?}"));
        false
    } else {
        log_panics::init();
        true
    }
}

async fn run_server(mut signal: watch::Receiver<()>, ipv6: bool) -> Result<()> {
    #[allow(clippy::wildcard_imports)]
    use api::*;

    let addr = if ipv6 {
        "[::1]:50052"
    } else {
        "127.0.0.1:50052"
    }
    .parse()
    .unwrap();

    let reflection = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build()?;

    info!("OBS Remote server starting up at {} ...", addr);

    let result = Server::builder()
        .accept_http1(true)
        .layer(tonic_web::GrpcWebLayer::new())
        .add_service(reflection)
        .add_service(new_service!(ConfigServiceServer, ConfigService))
        .add_service(new_service!(EventsServiceServer, EventsService))
        .add_service(new_service!(FiltersServiceServer, FiltersService))
        .add_service(new_service!(GeneralServiceServer, GeneralService))
        .add_service(new_service!(HotkeysServiceServer, HotkeysService))
        .add_service(new_service!(InputsServiceServer, InputsService))
        .add_service(new_service!(MediaInputsServiceServer, MediaInputsService))
        .add_service(new_service!(OutputsServiceServer, OutputsService))
        .add_service(new_service!(ProfilesServiceServer, ProfilesService))
        .add_service(new_service!(ProjectorsServiceServer, ProjectorsService))
        .add_service(new_service!(RecordingServiceServer, RecordingService))
        .add_service(new_service!(ReplayBufferServiceServer, ReplayBufferService))
        .add_service(new_service!(
            SceneCollectionsServiceServer,
            SceneCollectionsService
        ))
        .add_service(new_service!(SceneItemsServiceServer, SceneItemsService))
        .add_service(new_service!(ScenesServiceServer, ScenesService))
        .add_service(new_service!(SourcesServiceServer, SourcesService))
        .add_service(new_service!(StreamingServiceServer, StreamingService))
        .add_service(new_service!(TransitionsServiceServer, TransitionsService))
        .add_service(new_service!(VirtualCamServiceServer, VirtualCamService))
        .serve_with_shutdown(addr, async {
            signal.changed().await.ok();
        })
        .await;

    if let Err(e) = &result {
        error!("{}", e);
    } else {
        info!("server shut down");
    }

    result.map_err(Into::into)
}

declare_module!(ObsRemotePlugin);
module_use_default_locale!("en-US");
