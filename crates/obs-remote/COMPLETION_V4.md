# Implementation completion status

- 👍 Done
- 🚫 Not yet implemented
- 🗑️ Removed

## Events

### General events

| Method                 | Status |
| ---------------------- | ------ |
| Heartbeat              | 🚫      |
| BroadcastCustomMessage | 🚫      |

### Media events

| Method         | Status |
| -------------- | ------ |
| MediaPlaying   | 🚫      |
| MediaPaused    | 🚫      |
| MediaRestarted | 🚫      |
| MediaStopped   | 🚫      |
| MediaNext      | 🚫      |
| MediaPrevious  | 🚫      |
| MediaStarted   | 🚫      |
| MediaEnded     | 🚫      |

### Other events

| Method  | Status |
| ------- | ------ |
| Exiting | 👍      |

### Profiles events

| Method             | Status |
| ------------------ | ------ |
| ProfileChanged     | 👍      |
| ProfileListChanged | 👍      |

### Recording events

| Method            | Status |
| ----------------- | ------ |
| RecordingStarting | 👍      |
| RecordingStarted  | 👍      |
| RecordingStopping | 👍      |
| RecordingStopped  | 👍      |
| RecordingPaused   | 👍      |
| RecordingResumed  | 👍      |

### Replay Buffer events

| Method         | Status |
| -------------- | ------ |
| ReplayStarting | 👍      |
| ReplayStarted  | 👍      |
| ReplayStopping | 👍      |
| ReplayStopped  | 👍      |

### Scene Items events

| Method                     | Status |
| -------------------------- | ------ |
| SourceOrderChanged         | 🚫      |
| SceneItemAdded             | 🚫      |
| SceneItemRemoved           | 🚫      |
| SceneItemVisibilityChanged | 🚫      |
| SceneItemLockChanged       | 🚫      |
| SceneItemTransformChanged  | 🚫      |
| SceneItemSelected          | 🚫      |
| SceneItemDeselected        | 🚫      |

### Scenes events

| Method                     | Status |
| -------------------------- | ------ |
| SwitchScenes               | 🚫      |
| ScenesChanged              | 🚫      |
| SceneCollectionChanged     | 👍      |
| SceleCollectionListChanged | 👍      |

### Sources events

| Method                        | Status |
| ----------------------------- | ------ |
| SourceCreated                 | 🚫      |
| SourceDestroyed               | 🚫      |
| SourceVolumeChanged           | 🚫      |
| SourceMuteStateChanged        | 🚫      |
| SourceAudioDeactivated        | 🚫      |
| SourceAudioActivated          | 🚫      |
| SourceAudioSyncOffsetChanged  | 🚫      |
| SourceAudioMixersChanged      | 🚫      |
| SourceRenamed                 | 🚫      |
| SourceFilterAdded             | 🚫      |
| SourceFilterRemove            | 🚫      |
| SourceFilterVisibilityChanged | 🚫      |
| SourceFiltersReordered        | 🚫      |

### Streaming events

| Method         | Status |
| -------------- | ------ |
| StreamStarting | 👍      |
| StreamStarted  | 👍      |
| StreamStopping | 👍      |
| StreamStopped  | 👍      |
| StreamStatus   | 🚫      |

### Studio Mode events

| Method              | Status |
| ------------------- | ------ |
| PreviewSceneChanged | 🚫      |
| StudioModeSwitched  | 👍      |

### Transitions events

| Method                    | Status |
| ------------------------- | ------ |
| SwitchTransition          | 👍      |
| TransitionListChanged     | 👍      |
| TransitionDurationChanged | 🚫      |
| TransitionBegin           | 🚫      |
| TransitionEnd             | 🚫      |
| TransitionVideoEnd        | 🚫      |

### Virtual Cam events

| Method            | Status |
| ----------------- | ------ |
| VirtualCamStarted | 👍      |
| VirtualCamStopped | 👍      |

## Requests

### General

| Method                  | Status |
| ----------------------- | ------ |
| GetVersion              | 🚫      |
| GetAuthRequired         | 🚫      |
| Authenticate            | 🚫      |
| SetFilenameFormatting   | 👍      |
| GetFilenameFormatting   | 👍      |
| GetStats                | 👍      |
| GetVideoInfo            | 👍      |
| OpenProjector           | 👍      |
| TriggerHotkeyByName     | 👍      |
| TriggerHotkeyBySequence | 👍      |

### Media Control

| Method   | Status |
| -------- | ------ |
| Toggle   | 👍      |
| Restart  | 👍      |
| Stop     | 👍      |
| Next     | 👍      |
| Previous | 👍      |
| Duration | 👍      |
| GetTime  | 👍      |
| SetTime  | 👍      |
| Scrub    | 👍      |
| State    | 👍      |

### Outputs

| Method | Status |
| ------ | ------ |
| List   | 👍      |
| Info   | 👍      |
| Start  | 👍      |
| Stop   | 👍      |

### Profiles

| Method     | Status |
| ---------- | ------ |
| SetCurrent | 👍      |
| GetCurrent | 👍      |
| List       | 👍      |

### Recording

| Method             | Status |
| ------------------ | ------ |
| Status             | 👍      |
| Toggle             | 👍      |
| Start              | 👍      |
| Stop               | 👍      |
| Pause              | 👍      |
| Resume             | 👍      |
| SetRecordingFolder | 👍      |
| GetRecordingFolder | 👍      |

### Replay Buffer

| Method | Status |
| ------ | ------ |
| Status | 👍      |
| Toggle | 👍      |
| Start  | 👍      |
| Stop   | 👍      |
| Save   | 👍      |

### Scene Collections

| Method     | Status |
| ---------- | ------ |
| SetCurrent | 👍      |
| GetCurrent | 👍      |
| List       | 👍      |

### Scene Items

| Method        | Status |
| ------------- | ------ |
| List          | 👍      |
| GetProperties | 👍      |
| SetProperties | 👍      |
| Reset         | 👍      |
| SetRender     | 🗑️      |
| SetPosition   | 🗑️      |
| SetTransform  | 🗑️      |
| SetCrop       | 🗑️      |
| Delete        | 👍      |
| Add           | 👍      |
| Duplicate     | 👍      |

**Note:** Functionality of `SetRender`, `SetPosition`, `SetTransform` and `SetCrop` can be
accomplished with `SetProperties` and are therefore dropped.

### Scenes

| Method                   | Status |
| ------------------------ | ------ |
| SetCurrent               | 👍      |
| GetCurrent               | 👍      |
| List                     | 👍      |
| Create                   | 👍      |
| Reorder                  | 🚫      |
| SetTransitionOverride    | 👍      |
| RemoveTransitionOverride | 👍      |
| GetTransitionOverride    | 👍      |

### Sources

| Method                     | Status |
| -------------------------- | ------ |
| GetMediaSourcesList        | 🚫      |
| Create                     | 🚫      |
| GetSourcesList             | 🚫      |
| ListTypes                  | 👍      |
| GetVolume                  | 👍      |
| SetVolume                  | 👍      |
| GetTracks                  | 👍      |
| SetTracks                  | 👍      |
| GetMute                    | 👍      |
| SetMute                    | 👍      |
| ToggleMute                 | 👍      |
| GetSourceActive            | 👍      |
| GetAudioActive             | 👍      |
| SetName                    | 🚫      |
| SetSyncOffset              | 👍      |
| GetSyncOffset              | 👍      |
| GetSettings                | 🚫      |
| SetSettings                | 🚫      |
| GetTextGdiPlusProperties   | 🚫      |
| SetTextGdiPlusProperties   | 🚫      |
| GetTextFreetype2Properties | 🚫      |
| SetTextFreetype2Properties | 🚫      |
| GetSpecialSources          | 👍      |
| GetSourceFilters           | 🚫      |
| GetSourceFilterInfo        | 🚫      |
| AddFilterToSource          | 🚫      |
| RemoveFilterFromSource     | 🚫      |
| ReorderSourceFilter        | 🚫      |
| MoveSourceFilter           | 🚫      |
| SetSourceFilterSettings    | 🚫      |
| SetSourceFilterVisibility  | 🚫      |
| GetAudioMonitorType        | 🚫      |
| SetAudioMonitorType        | 🚫      |
| GetDefaultSettings         | 🚫      |
| TakeSourceScreenshot       | 🚫      |
| RefreshBrowserSource       | 🚫      |

### Streaming

| Method       | Status |
| ------------ | ------ |
| Status       | 👍      |
| Toggle       | 👍      |
| Start        | 🚫      |
| Stop         | 👍      |
| SetSettings  | 👍      |
| GetSettings  | 👍      |
| SaveSettings | 👍      |
| SendCaptions | 👍      |

### Studio Mode

| Method              | Status |
| ------------------- | ------ |
| Status              | 👍      |
| GetPreviewScene     | 👍      |
| SetPreviewScene     | 👍      |
| TransitionToProgram | 👍      |
| Enable              | 👍      |
| Disable             | 👍      |
| Toggle              | 👍      |

### Transitions

| Method          | Status |
| --------------- | ------ |
| List            | 👍      |
| GetCurrent      | 👍      |
| SetCurrent      | 👍      |
| SetDuration     | 👍      |
| GetDuration     | 👍      |
| Position        | 👍      |
| GetSettings     | 👍      |
| SetSettings     | 👍      |
| ReleaseTBar     | 👍      |
| SetTBarPosition | 👍      |

### Virtual Cam

| Method | Status |
| ------ | ------ |
| Status | 👍      |
| Toggle | 👍      |
| Start  | 👍      |
| Stop   | 👍      |
