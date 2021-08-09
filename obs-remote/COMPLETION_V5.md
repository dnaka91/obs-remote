# Implementation completion status

- 👍 Done
- 🚧 Partially working
- 🚫 Not yet implemented
- ❓ Missing in obs-websocket

## Events

### Config events

| Method                        | Status |
| ----------------------------- | ------ |
| CurrentSceneCollectionChanged | 🚫      |
| SceneCollectionListChanged    | 🚫      |
| CurrentProfileChanged         | 🚫      |
| ProfileListChanged            | 🚫      |

### Filters events

| Method              | Status |
| ------------------- | ------ |
| Created             | ❓      |
| Removed             | ❓      |
| NameChanged         | ❓      |
| AddedToSource       | ❓      |
| RemovedFromSource   | ❓      |
| SourceListReindexed | ❓      |

### General events

| Method                 | Status |
| ---------------------- | ------ |
| ExitStarted            | 🚫      |
| StudioModeStateChanged | 🚫      |
| Custom                 | ❓      |

### High Volume events

| Method            | Status |
| ----------------- | ------ |
| InputVolumeMeters | ❓      |

### Inputs events

| Method                 | Status |
| ---------------------- | ------ |
| Created                | 🚫      |
| Removed                | 🚫      |
| NameChanged            | 🚫      |
| ActiveStateChanged     | 🚫      |
| ShowStateChanged       | 🚫      |
| MuteStateChanged       | 🚫      |
| VolumeChanged          | 🚫      |
| AudioSyncOffsetChanged | 🚫      |
| AudioTracksChanged     | 🚫      |

### Media Inputs events

| Method          | Status |
| --------------- | ------ |
| PlaybackStarted | 🚫      |
| PlaybackEnded   | 🚫      |
| ActionTriggered | 🚫      |

### Outputs events

| Method                   | Status |
| ------------------------ | ------ |
| StreamStateChanged       | 🚫      |
| RecordStateChanged       | 🚫      |
| ReplayBufferStateChanged | 🚫      |
| VirtualCamStateChanged   | 🚫      |
| ReplayBufferSaved        | 🚫      |

### Scene Items events

| Method             | Status |
| ------------------ | ------ |
| Created            | 🚫      |
| Removed            | 🚫      |
| ListReindexed      | 🚫      |
| EnableStateChanged | 🚫      |
| LockStateChanged   | 🚫      |
| TransformChanged   | 🚫      |

### Scenes events

| Method                | Status |
| --------------------- | ------ |
| Created               | 🚫      |
| Removed               | 🚫      |
| NameChanged           | 🚫      |
| CurrentChanged        | 🚫      |
| CurrentPreviewChanged | 🚫      |
| ListReindexed         | 🚫      |

### Transitions events

| Method                 | Status |
| ---------------------- | ------ |
| Created                | 🚫      |
| Removed                | 🚫      |
| NameChanged            | 🚫      |
| CurrentChanged         | ❓      |
| CurrentDurationChanged | ❓      |
| Started                | ❓      |
| Ended                  | ❓      |

## Requests

### Config

| Method                  | Status |
| ----------------------- | ------ |
| GlobalPersistentData    | ❓      |
| SetGlobalPersistentData | ❓      |
| VideoSettings           | ❓      |
| SetVideoSettings        | ❓      |

### Filters

| Method          | Status |
| --------------- | ------ |
| List            | ❓      |
| DefaultSettings | ❓      |
| Get             | ❓      |
| SetIndex        | ❓      |
| SetSettings     | ❓      |
| SetEnabled      | ❓      |
| Create          | ❓      |
| Delete          | ❓      |

### General

| Method               | Status |
| -------------------- | ------ |
| Version              | 🚫      |
| BroadcastEvent       | 🚫      |
| SystemStats          | 🚫      |
| IsStudioModeEnabled  | 🚫      |
| SetStudioModeEnabled | 🚫      |
| Sleep                | 🚫      |

### Hotkeys

| Method            | Status |
| ----------------- | ------ |
| List              | 👍      |
| TriggerByName     | 👍      |
| TriggerBySequence | 👍      |

### Inputs

| Method                      | Status |
| --------------------------- | ------ |
| List                        | 👍      |
| ListKinds                   | 👍      |
| ListSpecial                 | ❓      |
| DefaultSettings             | 👍      |
| Settings                    | 👍      |
| SetSettings                 | 🚫      |
| Mute                        | 👍      |
| SetMute                     | 👍      |
| ToggleMute                  | 👍      |
| Volume                      | 👍      |
| SetVolume                   | 👍      |
| AudioSyncOffset             | ❓      |
| SetAudioSyncOffset          | ❓      |
| Tracks                      | ❓      |
| SetTracks                   | ❓      |
| MonitorMode                 | ❓      |
| SetMonitorMode              | ❓      |
| IsActive                    | ❓      |
| PropertiesListPropertyItems | ❓      |
| PressPropertiesButton       | ❓      |
| SetName                     | 🚫      |
| Create                      | 🚫      |
| Delete                      | ❓      |

### Media Inputs

| Method         | Status |
| -------------- | ------ |
| Status         | ❓      |
| OffsetTimecode | ❓      |
| SetTimecode    | ❓      |
| IsPaused       | ❓      |
| Stop           | ❓      |
| Restart        | ❓      |
| PlayNext       | ❓      |
| PlayPrevious   | ❓      |

### Outputs

| Method      | Status |
| ----------- | ------ |
| List        | ❓      |
| Status      | ❓      |
| Toggle      | ❓      |
| Start       | ❓      |
| Stop        | ❓      |
| Settings    | ❓      |
| SetSettings | ❓      |

### Profiles

| Method            | Status |
| ----------------- | ------ |
| List              | 👍      |
| Current           | 👍      |
| SetCurrent        | 👍      |
| Parameter         | 👍      |
| SetParameter      | 👍      |
| PersistentData    | ❓      |
| SetPersistentData | ❓      |
| Create            | ❓      |
| Delete            | ❓      |

### Projectors

| Method | Status |
| ------ | ------ |
| List   | ❓      |
| Open   | ❓      |
| Close  | ❓      |

### Recording

| Method                | Status |
| --------------------- | ------ |
| Status                | ❓      |
| Toggle                | ❓      |
| Start                 | ❓      |
| Stop                  | ❓      |
| TogglePause           | ❓      |
| Pause                 | ❓      |
| Resume                | ❓      |
| Directory             | ❓      |
| SetDirectory          | ❓      |
| FilenameFormatting    | ❓      |
| SetFilenameFormatting | ❓      |

### Replay Buffer

| Method  | Status |
| ------- | ------ |
| Status  | ❓      |
| Toggle  | ❓      |
| Start   | ❓      |
| Stop    | ❓      |
| Save    | ❓      |
| Time    | ❓      |
| SetTime | ❓      |

### Scene Collections

| Method     | Status |
| ---------- | ------ |
| List       | 👍      |
| Current    | 👍      |
| SetCurrent | 👍      |
| Create     | ❓      |
| Delete     | ❓      |

### Scene Items

| Method       | Status |
| ------------ | ------ |
| List         | ❓      |
| ListGroup    | ❓      |
| Transform    | ❓      |
| SetTransform | ❓      |
| IsEnabled    | ❓      |
| SetEnabled   | ❓      |
| IsLocked     | ❓      |
| SetLocked    | ❓      |
| Color        | ❓      |
| SetColor     | ❓      |
| SetIndex     | ❓      |
| Create       | ❓      |
| Delete       | ❓      |
| Duplicate    | ❓      |

### Scenes

| Method                   | Status |
| ------------------------ | ------ |
| List                     | 👍      |
| Current                  | 👍      |
| SetCurrent               | 👍      |
| CurrentPreview           | 👍      |
| SetCurrentPreview        | 👍      |
| SetIndex                 | ❓      |
| SetName                  | 👍      |
| Create                   | 👍      |
| Delete                   | 👍      |
| TransitionOverride       | ❓      |
| CreateTransitionOverride | ❓      |
| DeleteTransitionOverride | ❓      |

### Sources

| Method         | Status |
| -------------- | ------ |
| List           | ❓      |
| IsActive       | 👍      |
| Screenshot     | 🚧      |
| SaveScreenshot | 🚧      |

### Streaming

| Method       | Status |
| ------------ | ------ |
| Status       | ❓      |
| Toggle       | ❓      |
| Start        | ❓      |
| Stop         | ❓      |
| SendCaptions | ❓      |
| Bitrate      | ❓      |
| SetBitrate   | ❓      |
| Settings     | ❓      |
| SetSettings  | ❓      |

### Transitions

| Method                      | Status |
| --------------------------- | ------ |
| List                        | ❓      |
| Current                     | ❓      |
| SetCurrent                  | ❓      |
| SetCurrentDuration          | ❓      |
| Settings                    | ❓      |
| SetSettings                 | ❓      |
| ReleaseTBar                 | ❓      |
| SetTBarPosition             | ❓      |
| TriggerStudioModeTransition | ❓      |
| Create                      | ❓      |
| Delete                      | ❓      |

### Virtual Cam

| Method | Status |
| ------ | ------ |
| Status | ❓      |
| Toggle | ❓      |
| Start  | ❓      |
| Stop   | ❓      |
