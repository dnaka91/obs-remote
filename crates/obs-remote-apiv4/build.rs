use std::{env, path::PathBuf};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    tonic_build::configure()
        .build_client(false)
        .server_mod_attribute(".", "#[allow(clippy::return_self_not_must_use)]")
        .file_descriptor_set_path(out_dir.join("obs_remote_v4_descriptor.bin"))
        .compile(
            &[
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
            &["proto"],
        )
        .unwrap();
}
