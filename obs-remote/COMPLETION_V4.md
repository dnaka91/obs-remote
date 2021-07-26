# Implementation completion status

- 👍 Done
- 🚫 Not yet implemented
- 🗑️ Removed

## Events

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
| TriggerHotkeyByName     | 🚫      |
| TriggerHotkeyBySequence | 🚫      |

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
| ListTypes                  | 🚫      |
| GetVolume                  | 🚫      |
| SetVolume                  | 🚫      |
| GetTracks                  | 🚫      |
| SetTracks                  | 🚫      |
| GetMute                    | 🚫      |
| SetMute                    | 🚫      |
| ToggleMute                 | 🚫      |
| GetSourceActive            | 🚫      |
| GetAudioActive             | 🚫      |
| SetName                    | 🚫      |
| SetSyncOffset              | 🚫      |
| GetSyncOffset              | 🚫      |
| GetSettings                | 🚫      |
| SetSettings                | 🚫      |
| GetTextGdiPlusProperties   | 🚫      |
| SetTextGdiPlusProperties   | 🚫      |
| GetTextFreetype2Properties | 🚫      |
| SetTextFreetype2Properties | 🚫      |
| GetSpecialSources          | 🚫      |
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
