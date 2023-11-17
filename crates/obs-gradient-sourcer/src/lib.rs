use log::{info, Level};
use obs::{
    libobs_sys::{LIBOBS_API_MAJOR_VER, LIBOBS_API_MINOR_VER, LIBOBS_API_PATCH_VER},
    logger::ObsLogger,
    register::SourceInfo,
    source::{OutputFlags, SourceType},
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

        info!("Loaded (version {})", env!("CARGO_PKG_VERSION"));
        info!(
            "OBS version (headers: {}.{}.{} | runtime: {})",
            LIBOBS_API_MAJOR_VER,
            LIBOBS_API_MINOR_VER,
            LIBOBS_API_PATCH_VER,
            obs::obs_version()
        );

        obs::register::source(
            &SourceInfo::new("gradient_sourcer\0", SourceType::Input, OutputFlags::VIDEO)
                .get_name(|| "Color Gradient (Rust)\0")
                .create(|_, _| std::ptr::null_mut())
                .get_width(|_| 0)
                .get_height(|_| 0),
        );

        true
    }
}
