use std::{env, fs, path::PathBuf};

use protox::prost::Message;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let descriptor = protox::compile(
        [
            "../../proto/config/v1/config.proto",
            "../../proto/events/v1/events.proto",
            "../../proto/filters/v1/filters.proto",
            "../../proto/general/v1/general.proto",
            "../../proto/hotkeys/v1/hotkeys.proto",
            "../../proto/inputs/v1/inputs.proto",
            "../../proto/media_inputs/v1/media_inputs.proto",
            "../../proto/outputs/v1/outputs.proto",
            "../../proto/profiles/v1/profiles.proto",
            "../../proto/projectors/v1/projectors.proto",
            "../../proto/recording/v1/recording.proto",
            "../../proto/replay_buffer/v1/replay_buffer.proto",
            "../../proto/scene_collections/v1/scene_collections.proto",
            "../../proto/scene_items/v1/scene_items.proto",
            "../../proto/scenes/v1/scenes.proto",
            "../../proto/sources/v1/sources.proto",
            "../../proto/streaming/v1/streaming.proto",
            "../../proto/transitions/v1/transitions.proto",
            "../../proto/virtual_cam/v1/virtual_cam.proto",
        ],
        ["../../proto"],
    )
    .unwrap();

    fs::write(out_dir.join("descriptor.bin"), descriptor.encode_to_vec()).unwrap();

    tonic_build::configure()
        .build_client(false)
        // .server_mod_attribute(".", "#[allow(clippy::return_self_not_must_use)]")
        .compile_fds(descriptor)
        .unwrap();
}
