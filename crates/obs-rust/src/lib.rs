use std::{thread, time::Duration};

use libobs_sys::{LIBOBS_API_MAJOR_VER, LIBOBS_API_MINOR_VER, LIBOBS_API_PATCH_VER};
use log::{info, warn, Level};
use obs::{
    audio::AudioInfo,
    encoder::EncoderType,
    logger::ObsLogger,
    properties::{ComboFormat, TypedProperty},
    source::Source,
    Plugin,
};

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
                    info!("Sync offset: {:?}", source.sync_offset());
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
                obs::frontend::scene_collections::list()
            );
            info!("Scene names: {:?}", obs::frontend::scenes::names());

            obs::frontend::add_tools_menu_item("OBS Rust!");
            // obs::frontend::events::add_callback();

            list_encoders();

            info!("-----------------");
            info!("Service types: {:?}", obs::service::list_service_types());

            list_services();

            list_outputs();
            list_hotkeys();
        });

        true
    }
}

// obs::module_use_default_locale!("en-US");

fn list_modules() {
    info!("-----------------");
    info!("Modules");

    for module in obs::module::list_modules() {
        info!("  -----------------");
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

    for (ty, props) in obs::source::list_source_types()
        .into_iter()
        .filter(|ty| {
            ty != "av_capture_input"
                && ty != "av_capture_input_v2"
                && ty != "screen_capture"
                && ty != "vst_filter"
                && ty != "wipe_transition"
        })
        .filter_map(|ty| obs::source::properties(&ty).map(|p| (ty, p)))
    {
        info!("-----------------");
        info!("{ty} Properties");

        for p in props.iter() {
            info!("  -----------------");
            info!("  Name: {}", p.name());
            info!("  Type: {:?}", p.ty());
            info!("  Description: {}", p.description());
            info!("  Long description: {:?}", p.long_description());
            info!("  Enabled: {}", p.enabled());
            info!("  Visible: {}", p.visible());

            if let Some(tp) = p.as_typed() {
                match tp {
                    TypedProperty::Int(ip) => {
                        info!("  Min: {}", ip.min());
                        info!("  Max: {}", ip.max());
                        info!("  Step: {}", ip.step());
                        info!("  Suffix: {:?}", ip.suffix());
                        info!("  Number type: {:?}", ip.ty());
                    }
                    TypedProperty::Float(fp) => {
                        info!("  Min: {}", fp.min());
                        info!("  Max: {}", fp.max());
                        info!("  Step: {}", fp.step());
                        info!("  Suffix: {:?}", fp.suffix());
                        info!("  Number type: {:?}", fp.ty());
                    }
                    TypedProperty::Text(tp) => {
                        info!("  Monospace: {}", tp.monospace());
                        info!("  Text type: {:?}", tp.ty());
                    }
                    TypedProperty::Path(pp) => {
                        info!("  Default path: {:?}", pp.default_path());
                        info!("  Filter: {}", pp.filter());
                        info!("  Path type: {:?}", pp.ty());
                    }
                    TypedProperty::List(lp) => {
                        info!("  Count: {}", lp.count());
                        info!("  List format: {:?}", lp.format());
                        info!("  List type: {:?}", lp.ty());

                        info!("  -----------------");
                        info!("  List items");

                        for i in 0..lp.count() {
                            info!("    -----------------");
                            info!("    Name: {}", lp.item_name(i));
                            info!("    Disabled: {}", lp.item_disabled(i));

                            match lp.format() {
                                ComboFormat::Invalid => {
                                    info!("    Value: <invalid>");
                                }
                                ComboFormat::Int => {
                                    info!("    Value: {}", lp.item_int(i));
                                }
                                ComboFormat::Float => {
                                    info!("    Value: {}", lp.item_float(i));
                                }
                                ComboFormat::String => {
                                    info!("    Value: {}", lp.item_string(i));
                                }
                                ComboFormat::Unknown(v) => {
                                    info!("    Value: <unknown({v})>");
                                }
                            }
                        }
                    }
                    TypedProperty::Button(bp) => {
                        info!("  URL: {:?}", bp.url());
                        info!("  Button type: {:?}", bp.ty());
                    }
                    TypedProperty::EditableList(elp) => {
                        info!("  Default path: {:?}", elp.default_path());
                        info!("  Filter: {}", elp.filter());
                        info!("  Editable list type: {:?}", elp.ty());
                    }
                    TypedProperty::FrameRate(frp) => {
                        info!("  FPS range count: {}", frp.fps_ranges_count());
                        info!("  Option count: {}", frp.options_count());

                        info!("  -----------------");
                        info!("  FPS ranges");

                        for i in 0..frp.fps_ranges_count() {
                            info!("    -----------------");
                            info!("    Min: {:?}", frp.fps_range_min(i));
                            info!("    Max: {:?}", frp.fps_range_max(i));
                        }

                        info!("  -----------------");
                        info!("  Options");

                        for i in 0..frp.options_count() {
                            info!("    -----------------");
                            info!("    Name: {}", frp.option_name(i));
                            info!("    Description: {}", frp.option_description(i));
                        }
                    }
                    TypedProperty::Group(gp) => {
                        gp.content();
                        info!("  Group type: {:?}", gp.ty());
                    }
                }
            }
        }
    }
}

fn list_encoders() {
    info!("-----------------");
    info!("Encoders");

    for encoder in obs::encoder::list() {
        info!("  -----------------");
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

fn list_services() {
    info!("-----------------");
    info!("Services");

    for service in obs::service::list_services() {
        info!("  -----------------");
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
        info!(
            "  Supported video codecs: {:?}",
            service.supported_video_codecs()
        );
    }
}

fn list_outputs() {
    info!("-----------------");
    info!("Outputs");

    for output in obs::output::list_outputs() {
        info!("  -----------------");
        info!("  Name: {}", output.name());
        info!("  Flags: {:?}", output.flags());
    }
}

fn list_hotkeys() {
    info!("-----------------");
    info!("Hotkeys");

    for hotkey in obs::hotkeys::list() {
        info!("  -----------------");
        info!("  ID: {}", hotkey.id());
        info!("  Name: {}", hotkey.name());
        info!("  Description: {}", hotkey.description());
        info!("  Registerer: {:?}", hotkey.registerer_type());
    }
}
