use std::{
    collections::BTreeMap,
    fs::File,
    path::PathBuf,
    thread,
    time::{Duration, Instant},
};

use log::{info, Level, debug};
use obs::{
    audio::AudioInfo,
    encoder::EncoderType,
    hotkeys::HotkeyRegisterer,
    libobs_sys::{LIBOBS_API_MAJOR_VER, LIBOBS_API_MINOR_VER, LIBOBS_API_PATCH_VER},
    logger::ObsLogger,
    properties::{
        ButtonType, ComboFormat, ComboType, EditableListType, GroupType, MediaFps, NumberType,
        PathType, PropertyType, TextInfoType, TextType, TypedProperty,
    },
    source::{IconType, MonitoringType, OutputFlags, Source, SourceType, Volume},
    Plugin,
};
use serde::Serialize;
use serde_json::Value;

obs::declare_module!(MainPlugin);

struct MainPlugin;

impl Plugin for MainPlugin {
    fn new() -> Self {
        Self
    }

    fn load(&mut self) -> bool {
        ObsLogger::init(env!("CARGO_PKG_NAME"), Level::Info, vec![]).ok();

        info!(
            "OBS version: {}.{}.{}",
            LIBOBS_API_MAJOR_VER, LIBOBS_API_MINOR_VER, LIBOBS_API_PATCH_VER
        );

        thread::spawn(|| {
            thread::sleep(Duration::from_secs(2));

            // obs::frontend::add_tools_menu_item("OBS Rust!");
            // obs::frontend::events::add_callback();

            let start = Instant::now();

            let data = JsonData {
                sources: list_sources(),
                audio_info: AudioInfo::get(),
                cli_args: obs::cmdline_args(),
                modules: list_modules(),
                source_types: obs::source::list_source_types(),
                input_types: obs::source::list_input_types(),
                filter_types: obs::source::list_filter_types(),
                transition_types: obs::source::list_transition_types(),
                source_properties: list_source_properties(),
                profiles: { obs::frontend::profiles::list() },
                scene_collections: { obs::frontend::scene_collections::list() },
                scene_names: obs::frontend::scenes::names(),
                encoders: list_encoders(),
                service_types: obs::service::list_service_types(),
                services: list_services(),
                outputs: list_outputs(),
                hotkeys: list_hotkeys(),
            };

            info!("gathering data took {:?}", start.elapsed());

            let home = PathBuf::from(std::env::var_os("HOME").unwrap());

            let mut file = File::create(home.join("obs-rust.json")).unwrap();
            serde_json::to_writer_pretty(&mut file, &data).unwrap();
        });

        true
    }
}

// obs::module_use_default_locale!("en-US");

#[derive(Serialize)]
struct JsonData {
    sources: BTreeMap<String, JsonSource>,
    audio_info: Option<AudioInfo>,
    cli_args: Vec<String>,
    modules: Vec<JsonModule>,
    source_types: Vec<String>,
    input_types: Vec<String>,
    filter_types: Vec<String>,
    transition_types: Vec<String>,
    source_properties: BTreeMap<String, Vec<JsonProperty>>,
    profiles: Vec<String>,
    scene_collections: Vec<String>,
    scene_names: Vec<String>,
    encoders: Vec<JsonEncoder>,
    service_types: Vec<String>,
    services: Vec<JsonService>,
    outputs: Vec<JsonOutput>,
    hotkeys: Vec<JsonHotkey>,
}

#[derive(Serialize)]
struct JsonSource {
    base_size: JsonSize,
    audio_mixers: [bool; 6],
    balance_value: f32,
    flags: u32,
    icon_type: IconType,
    id: String,
    monitoring_type: MonitoringType,
    name: String,
    output_flags: OutputFlags,
    sync_offset: obs::Duration,
    r#type: SourceType,
    unversioned_id: String,
    volume: Volume,
    size: JsonSize,
    settings: Value,
}

#[derive(Serialize)]
struct JsonSize {
    width: u32,
    height: u32,
}

fn list_sources() -> BTreeMap<String, JsonSource> {
    obs::source::list()
        .into_iter()
        .map(|source| {
            (
                source.name(),
                JsonSource {
                    base_size: JsonSize {
                        width: source.base_width(),
                        height: source.base_height(),
                    },
                    audio_mixers: source.audio_mixers(),
                    balance_value: source.balance_value(),
                    flags: source.flags(),
                    icon_type: Source::icon_type(&source.id()),
                    id: source.id(),
                    monitoring_type: source.monitoring_type(),
                    name: source.name(),
                    output_flags: source.output_flags(),
                    sync_offset: source.sync_offset(),
                    r#type: source.ty(),
                    unversioned_id: source.unversioned_id(),
                    volume: source.volume(),
                    size: JsonSize {
                        width: source.width(),
                        height: source.height(),
                    },
                    settings: serde_json::from_str(&source.settings().to_json()).unwrap(),
                },
            )
        })
        .collect()
}

#[derive(Serialize)]
struct JsonModule {
    name: Option<String>,
    author: Option<String>,
    description: Option<String>,
    file_name: String,
    binary_path: PathBuf,
    data_path: PathBuf,
}

fn list_modules() -> Vec<JsonModule> {
    obs::module::list_modules()
        .into_iter()
        .map(|module| JsonModule {
            name: module.name(),
            author: module.author(),
            description: module.description(),
            file_name: module.file_name(),
            binary_path: module.binary_path(),
            data_path: module.data_path(),
        })
        .collect()
}

#[derive(Serialize)]
struct JsonProperty {
    name: String,
    r#type: PropertyType,
    description: String,
    long_description: Option<String>,
    enabled: bool,
    visible: bool,
    data: Option<JsonPropertyType>,
}

#[derive(Serialize)]
enum JsonPropertyType {
    Int {
        r#type: NumberType,
        min: i32,
        max: i32,
        step: i32,
        suffix: Option<String>,
    },
    Float {
        r#type: NumberType,
        min: f64,
        max: f64,
        step: f64,
        suffix: Option<String>,
    },
    Text {
        r#type: TextType,
        monospace: bool,
        info_type: TextInfoType,
        info_word_wrap: bool,
    },
    Path {
        r#type: PathType,
        default_path: Option<String>,
        filter: String,
    },
    List {
        r#type: ComboType,
        format: ComboFormat,
        count: usize,
        items: Vec<ListItem>,
    },
    Button {
        r#type: ButtonType,
        url: Option<String>,
    },
    EditableList {
        r#type: EditableListType,
        default_path: Option<String>,
        filter: String,
    },
    FrameRate {
        fps_range_count: usize,
        option_count: usize,
        fps_ranges: Vec<FpsRange>,
        options: Vec<FrameRateOption>,
    },
    Group {
        r#type: GroupType,
    },
}

#[derive(Serialize)]
struct ListItem {
    name: String,
    disabled: bool,
    value: ListItemValue,
}

#[derive(Serialize)]
enum ListItemValue {
    Null,
    Int(i64),
    Float(f64),
    String(String),
}

#[derive(Serialize)]
struct FpsRange {
    min: MediaFps,
    max: MediaFps,
}

#[derive(Serialize)]
struct FrameRateOption {
    name: String,
    description: String,
}

fn list_source_properties() -> BTreeMap<String, Vec<JsonProperty>> {
    const BUGGED_SOURCE_TYPES: &[&str] = &[
        "av_capture_input_v2",
        "av_capture_input",
        "expander_filter",
        "screen_capture",
        "upward_compressor_filter",
        "v4l2_input",
        "vst_filter",
        "wipe_transition",
        "xshm_input",
    ];

    obs::source::list_source_types()
        .into_iter()
        .filter(|ty| !BUGGED_SOURCE_TYPES.contains(&ty.as_str()))
        .filter_map(|ty| {
            debug!("list properties for: {ty}");
            obs::source::properties(&ty).map(|p| (ty, p))
        })
        .map(|(ty, props)| {
            (
                ty,
                props
                    .iter()
                    .map(|p| JsonProperty {
                        name: p.name(),
                        r#type: p.ty(),
                        description: p.description(),
                        long_description: p.long_description(),
                        enabled: p.enabled(),
                        visible: p.visible(),
                        data: p.as_typed().map(|typed| match typed {
                            TypedProperty::Int(ip) => JsonPropertyType::Int {
                                r#type: ip.ty(),
                                min: ip.min(),
                                max: ip.max(),
                                step: ip.step(),
                                suffix: ip.suffix(),
                            },
                            TypedProperty::Float(fp) => JsonPropertyType::Float {
                                r#type: fp.ty(),
                                min: fp.min(),
                                max: fp.max(),
                                step: fp.step(),
                                suffix: fp.suffix(),
                            },
                            TypedProperty::Text(tp) => JsonPropertyType::Text {
                                r#type: tp.ty(),
                                monospace: tp.monospace(),
                                info_type: tp.info_type(),
                                info_word_wrap: tp.info_word_wrap(),
                            },
                            TypedProperty::Path(pp) => JsonPropertyType::Path {
                                r#type: pp.ty(),
                                default_path: pp.default_path(),
                                filter: pp.filter(),
                            },
                            TypedProperty::List(lp) => JsonPropertyType::List {
                                r#type: lp.ty(),
                                format: lp.format(),
                                count: lp.count(),
                                items: (0..lp.count())
                                    .map(|i| ListItem {
                                        name: lp.item_name(i),
                                        disabled: lp.item_disabled(i),
                                        value: match lp.format() {
                                            ComboFormat::Invalid | ComboFormat::Unknown(_) => {
                                                ListItemValue::Null
                                            }
                                            ComboFormat::Int => ListItemValue::Int(lp.item_int(i)),
                                            ComboFormat::Float => {
                                                ListItemValue::Float(lp.item_float(i))
                                            }
                                            ComboFormat::String => {
                                                ListItemValue::String(lp.item_string(i))
                                            }
                                        },
                                    })
                                    .collect(),
                            },
                            TypedProperty::Button(bp) => JsonPropertyType::Button {
                                r#type: bp.ty(),
                                url: bp.url(),
                            },
                            TypedProperty::EditableList(elp) => JsonPropertyType::EditableList {
                                r#type: elp.ty(),
                                default_path: elp.default_path(),
                                filter: elp.filter(),
                            },
                            TypedProperty::FrameRate(frp) => JsonPropertyType::FrameRate {
                                fps_range_count: frp.fps_ranges_count(),
                                option_count: frp.options_count(),
                                fps_ranges: frp
                                    .fps_range_iter()
                                    .map(|(min, max)| FpsRange { min, max })
                                    .collect(),
                                options: frp
                                    .option_iter()
                                    .map(|(name, description)| FrameRateOption {
                                        name,
                                        description,
                                    })
                                    .collect(),
                            },
                            TypedProperty::Group(gp) => {
                                gp.content();
                                JsonPropertyType::Group { r#type: gp.ty() }
                            }
                        }),
                    })
                    .collect(),
            )
        })
        .collect()
}

#[derive(Serialize)]
struct JsonEncoder {
    name: String,
    r#type: EncoderType,
    codec: String,
    data: Option<JsonEncoderData>,
}

#[derive(Serialize)]
enum JsonEncoderData {
    Audio {
        sample_rate: u32,
    },
    Video {
        scaling: bool,
        width: u32,
        height: u32,
    },
}

fn list_encoders() -> Vec<JsonEncoder> {
    obs::encoder::list()
        .into_iter()
        .map(|encoder| JsonEncoder {
            name: encoder.name(),
            r#type: encoder.ty(),
            codec: encoder.codec(),
            data: match encoder.ty() {
                EncoderType::Audio => Some(JsonEncoderData::Audio {
                    sample_rate: encoder.sample_rate(),
                }),
                EncoderType::Video => Some(JsonEncoderData::Video {
                    scaling: encoder.scaling(),
                    width: encoder.width(),
                    height: encoder.height(),
                }),
                _ => None,
            },
        })
        .collect()
}

#[derive(Serialize)]
struct JsonService {
    name: String,
    id: String,
    display_name: Option<String>,
    max_bitrate: MaxBitrate,
    max_fps: u32,
    url: String,
    supported_video_codecs: Vec<String>,
}

#[derive(Serialize)]
struct MaxBitrate {
    video: u32,
    audio: u32,
}

fn list_services() -> Vec<JsonService> {
    obs::service::list_services()
        .into_iter()
        .map(|service| {
            let (video, audio) = service.max_bitrate();
            JsonService {
                name: service.name(),
                id: service.id(),
                display_name: obs::service::Service::display_name(&service.id()),
                max_bitrate: MaxBitrate { video, audio },
                max_fps: service.max_fps(),
                url: service.url(),
                supported_video_codecs: service.supported_video_codecs(),
            }
        })
        .collect()
}

#[derive(Serialize)]
struct JsonOutput {
    name: String,
    flags: obs::output::Flags,
}

fn list_outputs() -> Vec<JsonOutput> {
    obs::output::list_outputs()
        .into_iter()
        .map(|output| JsonOutput {
            name: output.name(),
            flags: output.flags(),
        })
        .collect()
}

#[derive(Serialize)]
struct JsonHotkey {
    id: usize,
    name: String,
    description: String,
    registerer: HotkeyRegisterer,
}

fn list_hotkeys() -> Vec<JsonHotkey> {
    obs::hotkeys::list()
        .into_iter()
        .map(|hotkey| JsonHotkey {
            id: hotkey.id(),
            name: hotkey.name(),
            description: hotkey.description(),
            registerer: hotkey.registerer_type(),
        })
        .collect()
}
