use std::{thread, time::Duration};

use libobs_sys::{LIBOBS_API_MAJOR_VER, LIBOBS_API_MINOR_VER, LIBOBS_API_PATCH_VER};
use log::{info, warn, Level};
use obs::{Plugin, audio::AudioInfo, encoder::EncoderType, logger::ObsLogger, source::Source};

obs::declare_module!(MainPlugin);

struct MainPlugin;

impl Plugin for MainPlugin {
    fn new() -> Self {
        Self
    }

    fn load(&mut self) -> bool {
        ObsLogger::init(env!("CARGO_PKG_NAME"), Level::Info, vec![]).ok();

        info!("HELLO WORLD!!!");

        info!(
            "OBS version: {}.{}.{}",
            LIBOBS_API_MAJOR_VER, LIBOBS_API_MINOR_VER, LIBOBS_API_PATCH_VER
        );

        thread::spawn(|| {
            thread::sleep(Duration::from_secs(2));
            let get_source = |name| match Source::by_name(name) {
                Some(source) => {
                    info!("-----------------");
                    info!("Found source `{}`", name);
                    info!(
                        "Base size: {}x{}",
                        source.base_width(),
                        source.base_height()
                    );
                    info!("Audio mixers: {:?}", source.audio_mixers());
                    info!("Balance value: {}", source.balance_value());
                    info!("Flags: {:b}", source.flags());
                    info!("Icon type: {:?}", Source::icon_type(&source.id()));
                    info!("ID: {}", source.id());
                    info!("Monitoring type: {:?}", source.monitoring_type());
                    info!("Name: {}", source.name());
                    info!("Output flags: {:?}", source.output_flags());
                    info!("Sync offset: {}", source.sync_offset());
                    info!("Type: {:?}", source.ty());
                    info!("Unversioned ID: {}", source.unversioned_id());
                    info!("Volume: {}", source.volume());
                    info!("Size: {}x{}", source.width(), source.height());
                    info!("Settings: {}", source.settings().to_json());
                }
                None => warn!("Source not found!"),
            };

            info!("-----------------");
            info!("Audio info: {:?}", AudioInfo::get());

            info!("-----------------");
            info!("Commandline args: {:?}", obs::cmdline_args());

            get_source("OBWS-TEST-Text");
            get_source("BOOBS");
            get_source("OBWS-TEST-Media");

            list_modules();
            list_source_types();

            info!("-----------------");
            info!("Profiles: {:?}", obs::frontend::profiles::list());
            info!(
                "Scene collections: {:?}",
                obs::frontend::scene_collections()
            );
            info!("Scene names: {:?}", obs::frontend::scene_names());

            obs::frontend::add_tools_menu_item("OBS Rust!");
            // obs::frontend::events::add_callback();

            list_encoders();

            info!("-----------------");
            info!("Service types: {:?}", obs::service::list_service_types());

            info!("-----------------");
            info!("Services");

            for service in obs::service::list_services() {
                info!("-----------------");
                info!("  Name: {}", service.name());
                info!("  ID: {}", service.id());
                info!(
                    "  Display name: {}",
                    obs::service::Service::display_name(&service.id()).unwrap()
                );
                let (video, audio) = service.max_bitrate();
                info!("  Max Bitrate: video {} kbps / audio {} kbps", video, audio);
                info!("  Max FPS: {}", service.max_fps());
                info!("  URL: {}", service.url());
            }

            info!("-----------------");
            info!("Outputs");

            for output in obs::output::list_outputs() {
                info!("-----------------");
                info!("  Name: {}", output.name());
                info!("  Flags: {:?}", output.flags());
            }
        });

        true
    }
}

// obs::module_use_default_locale!("en-US");

fn list_modules() {
    info!("-----------------");
    info!("Modules");

    for module in obs::module::list_modules() {
        info!("-----------------");
        info!("  Name: {:?}", module.name());
        info!("  Author: {:?}", module.author());
        info!("  Description: {:?}", module.description());
        info!("  File name: {}", module.file_name());
        info!("  Binary path: {:?}", module.binary_path());
        info!("  Data path: {:?}", module.data_path());
    }
}

fn list_source_types() {
    info!("-----------------");
    info!("Source types: {:#?}", obs::source::list_source_types());
    info!("Input types: {:#?}", obs::source::list_input_types());
    info!("Filter types: {:#?}", obs::source::list_filter_types());
    info!(
        "Transition types: {:#?}",
        obs::source::list_transition_types()
    );
}

fn list_encoders() {
    info!("-----------------");
    info!("Encoders");

    for encoder in obs::encoder::list() {
        info!("-----------------");
        info!("  Name: {}", encoder.name());
        info!("  Type: {:?}", encoder.ty());
        info!("  Codec: {}", encoder.codec());

        match encoder.ty() {
            EncoderType::Audio => {
                info!("  Sample rate: {}", encoder.sample_rate());
            }
            EncoderType::Video => {
                info!("  Scaling: {}", encoder.scaling());
                info!("  Width: {}", encoder.width());
                info!("  Height: {}", encoder.height());
            }
            _ => {}
        }
    }
}
