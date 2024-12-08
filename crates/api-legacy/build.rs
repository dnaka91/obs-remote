use std::{env, fs, path::PathBuf};

use protox::prost::Message;

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let descriptor = protox::compile(
        [
            "proto/events.proto",
            "proto/general.proto",
            "proto/media_control.proto",
            "proto/outputs.proto",
            "proto/profiles.proto",
            "proto/recording.proto",
            "proto/replay_buffer.proto",
            "proto/scene_collections.proto",
            "proto/scene_items.proto",
            "proto/scenes.proto",
            "proto/sources.proto",
            "proto/streaming.proto",
            "proto/studio_mode.proto",
            "proto/transitions.proto",
            "proto/virtual_cam.proto",
        ],
        ["proto"],
    )
    .unwrap();

    fs::write(
        out_dir.join("obs_remote_legacy_descriptor.bin"),
        descriptor.encode_to_vec(),
    )
    .unwrap();

    tonic_build::configure()
        .build_client(false)
        .compile_fds(descriptor)
        .unwrap();
}
