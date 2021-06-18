use std::{env, path::PathBuf};

fn main() {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    tonic_build::configure()
        .build_client(false)
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

    tonic_build::configure()
        .build_client(false)
        .file_descriptor_set_path(out_dir.join("obs_remote_v5_descriptor.bin"))
        .compile(
            &[
                "proto/v5/config.proto",
                "proto/v5/events.proto",
                "proto/v5/filters.proto",
                "proto/v5/general.proto",
                "proto/v5/hotkeys.proto",
                "proto/v5/inputs.proto",
                "proto/v5/media_inputs.proto",
                "proto/v5/outputs.proto",
                "proto/v5/profiles.proto",
                "proto/v5/projectors.proto",
                "proto/v5/recording.proto",
                "proto/v5/replay_buffer.proto",
                "proto/v5/scene_collections.proto",
                "proto/v5/scene_items.proto",
                "proto/v5/scenes.proto",
                "proto/v5/sources.proto",
                "proto/v5/streaming.proto",
                "proto/v5/transitions.proto",
                "proto/v5/virtual_cam.proto",
            ],
            &["proto"],
        )
        .unwrap();
}
