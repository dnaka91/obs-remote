fn main() {
    tonic_build::configure()
        .build_server(false)
        .compile(
            &[
                "../obs-remote-apiv4/proto/events.proto",
                "../obs-remote-apiv4/proto/general.proto",
                "../obs-remote-apiv4/proto/media_control.proto",
                "../obs-remote-apiv4/proto/outputs.proto",
                "../obs-remote-apiv4/proto/profiles.proto",
                "../obs-remote-apiv4/proto/recording.proto",
                "../obs-remote-apiv4/proto/replay_buffer.proto",
                "../obs-remote-apiv4/proto/scene_collections.proto",
                "../obs-remote-apiv4/proto/scene_items.proto",
                "../obs-remote-apiv4/proto/scenes.proto",
                "../obs-remote-apiv4/proto/sources.proto",
                "../obs-remote-apiv4/proto/streaming.proto",
                "../obs-remote-apiv4/proto/studio_mode.proto",
                "../obs-remote-apiv4/proto/transitions.proto",
                "../obs-remote-apiv4/proto/virtual_cam.proto",
            ],
            &["../obs-remote-apiv4/proto"],
        )
        .unwrap();

    tonic_build::configure()
        .build_server(false)
        .compile(
            &[
                "../obs-remote-apiv5/proto/config.proto",
                "../obs-remote-apiv5/proto/events.proto",
                "../obs-remote-apiv5/proto/filters.proto",
                "../obs-remote-apiv5/proto/general.proto",
                "../obs-remote-apiv5/proto/hotkeys.proto",
                "../obs-remote-apiv5/proto/inputs.proto",
                "../obs-remote-apiv5/proto/media_inputs.proto",
                "../obs-remote-apiv5/proto/outputs.proto",
                "../obs-remote-apiv5/proto/profiles.proto",
                "../obs-remote-apiv5/proto/projectors.proto",
                "../obs-remote-apiv5/proto/recording.proto",
                "../obs-remote-apiv5/proto/replay_buffer.proto",
                "../obs-remote-apiv5/proto/scene_collections.proto",
                "../obs-remote-apiv5/proto/scene_items.proto",
                "../obs-remote-apiv5/proto/scenes.proto",
                "../obs-remote-apiv5/proto/sources.proto",
                "../obs-remote-apiv5/proto/streaming.proto",
                "../obs-remote-apiv5/proto/transitions.proto",
                "../obs-remote-apiv5/proto/virtual_cam.proto",
            ],
            &["../obs-remote-apiv5/proto"],
        )
        .unwrap();
}
