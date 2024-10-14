use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::value::RawValue;

use crate::rest::{DeviceID, FileName, FolderName, Folder};

#[derive(Debug, Deserialize)]
pub struct ClusterConfigReceivedEvent {
    #[serde(rename = "device")]
    pub device_id: DeviceID,
}

//FIXME: complete
#[derive(Debug, Deserialize)]
pub struct ConfigSavedEvent {
    pub version: u64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct DeviceConnectedEvent {
    #[serde(rename = "addr")]
    pub address: String,
    #[serde(rename = "id")]
    pub device_id: DeviceID,
    pub device_name: String,
    pub client_name: String,
    pub client_version: String,
    #[serde(rename = "type")]
    pub client_type: String, //FIXME: use enum
}

#[derive(Debug, Deserialize)]
pub struct DeviceDisconnectedEvent {
    #[serde(rename = "id")]
    pub device_id: DeviceID,
    pub error: String,
}

#[derive(Debug, Deserialize)]
pub struct DeviceDiscoveredEvent {
    #[serde(rename = "device")]
    pub device_id: DeviceID,
    pub addrs: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct DevicePausedEvent {
    #[serde(rename = "device")]
    pub device_id: DeviceID,
}

/// NOTE: This event is deprecated; You should use a replacement PendingDevicesChanged event instead.
#[derive(Debug, Deserialize)]
pub struct DeviceRejectedEvent {
    #[serde(rename = "device")]
    pub device_id: DeviceID,
    pub name: String,
    pub address: String,
}

#[derive(Debug, Deserialize)]
pub struct DeviceResumedEvent {
    #[serde(rename = "device")]
    pub device_id: DeviceID,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct FolderCompletionEvent {
    pub completion: f64,
    #[serde(rename = "device")]
    pub device_id: DeviceID,
    #[serde(rename = "folder")]
    pub folder_id: String,
    pub global_bytes: u64,
    pub global_items: u64,
    pub need_bytes: u64,
    pub need_deletes: u64,
    pub need_items: u64,
    pub remote_state: String, //FIXME: Use enum here
    pub sequence: u64,
}

#[derive(Debug, Deserialize)]
pub struct FolderErrorsEvent {
    pub folder: String,
    pub errors: Vec<FolderError>,
}

#[derive(Debug, Deserialize)]
pub struct FolderError {
    pub error: String,
    pub path: String,
}

#[derive(Debug, Deserialize)]
pub struct FolderPausedEvent {
    pub id: String,
    pub label: String,
}

#[derive(Debug, Deserialize)]
pub struct FolderRejectedEvent {
    #[serde(rename = "device")]
    pub device_id: DeviceID,
    #[serde(rename = "folder")]
    pub folder_id: String,
    #[serde(rename = "folderLabel")]
    pub folder_label: String,
}

#[derive(Debug, Deserialize)]
pub struct FolderResumedEvent {
    #[serde(rename = "id")]
    pub folder_id: String,
    #[serde(rename = "label")]
    pub folder_label: String,
}

#[derive(Debug, Deserialize)]
pub struct FolderScanProgressEvent {
    pub total: u64,
    pub rate: u64,
    pub current: u64,
    #[serde(rename = "folder")]
    pub folder_id: String,
}

#[derive(Debug, Deserialize)]
pub struct FolderSummaryEvent {
    #[serde(rename = "folder")]
    pub folder_id: String,
    pub summary: FolderSummaryData,
}

// FIXME: Add `remoteSequence`
#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct FolderSummaryData {
    pub error: String,
    pub errors: u64,
    pub global_bytes: u64,
    pub global_deleted: u64,
    pub global_directories: u64,
    pub global_files: u64,
    pub global_symlinks: u64,
    pub global_total_items: u64,
    pub ignore_patterns: bool,
    pub in_sync_bytes: u64,
    pub in_sync_files: u64,
    pub invalid: Option<String>,
    pub local_bytes: u64,
    pub local_deleted: u64,
    pub local_directories: u64,
    pub local_files: u64,
    pub local_symlinks: u64,
    pub local_total_items: u64,
    pub need_bytes: u64,
    pub need_deletes: u64,
    pub need_directories: u64,
    pub need_files: u64,
    pub need_symlinks: u64,
    pub need_total_items: u64,
    pub pull_errors: u64,
    pub receive_only_changed_bytes: u64,
    pub receive_only_changed_deletes: u64,
    pub receive_only_changed_directories: u64,
    pub receive_only_changed_files: u64,
    pub receive_only_changed_symlinks: u64,
    pub receive_only_total_items: u64,
    pub sequence: u64,
    pub state: String, //FIXME: Use enum here
    pub state_changed: String,
    pub version: u64,
    pub watch_error: String,
}

#[derive(Debug, Deserialize)]
pub struct FolderWatchStateChangedEvent {
    #[serde(rename = "folder")]
    pub folder_id: String,
    pub from: Option<String>,
    pub to: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "lowercase"))]
pub enum ItemAction {
    Update,
    Metadata,
    Delete,
}

#[derive(Debug, Deserialize)]
pub struct ItemFinishedEvent {
    pub item: String,
    #[serde(rename = "folder")]
    pub folder_id: String,
    pub error: Option<String>,
    #[serde(rename = "type")]
    pub item_type: String, //FIXME: use enum
    pub action: ItemAction,
}

#[derive(Debug, Deserialize)]
pub struct ItemStartedEvent {
    pub item: String,
    #[serde(rename = "folder")]
    pub folder_id: String,
    #[serde(rename = "type")]
    pub item_type: String, //FIXME: use enum
    pub action: ItemAction,
}

// FIXME: Finish
#[derive(Debug, Deserialize)]
pub struct ListenAddressesChangedEvent {}

/// NOTE: `folderID` field is deprecated. Use `folder` field
#[derive(Debug, Deserialize)]
pub struct LocalChangeDetectedEvent {
    pub action: String, //FIXME: use enum
    #[serde(rename = "folder")]
    pub folder_id: String,
    #[serde(rename = "folderID")]
    pub deprecated_folder_id: String,
    pub label: String,
    pub path: String,
    #[serde(rename = "type")]
    pub item_type: String, //FIXME: use enum
}

/// NOTE: `version` field is deprecated. Use `sequence` field
#[derive(Debug, Deserialize)]
pub struct LocalIndexUpdatedEvent {
    #[serde(rename = "folder")]
    pub folder_id: String,
    pub items: u64,
    pub filenames: Vec<FileName>,
    pub sequence: u64,
    #[serde(rename = "version")]
    pub deprecated_version: u64,
}

#[derive(Debug, Deserialize)]
pub struct LoginAttemptEvent {
    #[serde(rename = "remoteAddress")]
    pub remote_address: String,
    pub username: String,
    pub success: bool,
}

// FIXME: Finish
#[derive(Debug, Deserialize)]
pub struct PendingDevicesChangedEvent {}

// FIXME: Finish
#[derive(Debug, Deserialize)]
pub struct PendingFoldersChangedEvent {}

#[derive(Debug, Deserialize)]
pub struct RemoteChangeDetectedEvent {
    #[serde(rename = "type")]
    pub item_type: String, //FIXME: use enum
    pub action: String, //FIXME: use enum
    #[serde(rename = "folder")]
    pub folder_id: String,
    #[serde(rename = "folderID")]
    pub deprecated_folder_id: String,
    pub path: String,
    pub label: String,
    #[serde(rename = "modifiedBy")]
    pub modified_by: String,
}

#[derive(Debug, Deserialize)]
pub struct RemoteDownloadProgressEvent {
    #[serde(rename = "device")]
    pub device_id: DeviceID,
    #[serde(rename = "folder")]
    pub folder_id: String,
    pub state: HashMap<FileName, u64>,
}

#[derive(Debug, Deserialize)]
pub struct RemoteIndexUpdatedEvent {
    #[serde(rename = "device")]
    pub device_id: DeviceID,
    #[serde(rename = "folder")]
    pub folder_id: String,
    pub items: u64,
}

#[derive(Debug, Deserialize)]
pub struct StartingEvent {
    #[serde(rename = "home")]
    pub home_path: String,
}

// TODO: Event API states there are only 4 states. Check https://docs.syncthing.net/events/statechanged.html and source code
#[derive(Debug, Deserialize)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub enum FolderState {
    Idle,
    Scanning,
    ScanWaiting,
    SyncPreparing,
    SyncWaiting,
    Syncing,
    Error,
    Unknown,
}

#[derive(Debug, Deserialize)]
pub struct StateChangedEvent {
    #[serde(rename = "folder")]
    pub folder_id: String,
    pub duration: Option<f64>,
    pub from: FolderState,
    pub to: FolderState,
}

#[derive(Debug, Deserialize)]
pub enum EventData {
    ClusterConfigReceived(ClusterConfigReceivedEvent),
    ConfigSaved(ConfigSavedEvent),
    DeviceConnected(DeviceConnectedEvent),
    DeviceDisconnected(DeviceDisconnectedEvent),
    DeviceDiscovered(DeviceDiscoveredEvent),
    DevicePaused(DevicePausedEvent),
    DeviceRejected(DeviceRejectedEvent),
    DeviceResumed(DeviceResumedEvent),
    DownloadProgress(HashMap<FolderName, Folder>),
    Failure(String),
    FolderCompletion(FolderCompletionEvent),
    FolderErrors(FolderErrorsEvent),
    FolderPaused(FolderPausedEvent),
    FolderRejected(FolderRejectedEvent),
    FolderResumed(FolderResumedEvent),
    FolderScanProgress(FolderScanProgressEvent),
    FolderSummary(Box<FolderSummaryEvent>),
    FolderWatchStateChanged(FolderWatchStateChangedEvent),
    ItemFinished(ItemFinishedEvent),
    ItemStarted(ItemStartedEvent),
    ListenAddressesChanged(ListenAddressesChangedEvent),
    LocalChangeDetected(LocalChangeDetectedEvent),
    LocalIndexUpdated(LocalIndexUpdatedEvent),
    LoginAttempt(LoginAttemptEvent),
    PendingDevicesChanged(),
    PendingFoldersChanged(),
    RemoteChangeDetected(RemoteChangeDetectedEvent),
    RemoteDownloadProgress(RemoteDownloadProgressEvent),
    RemoteIndexUpdated(RemoteIndexUpdatedEvent),
    Starting(StartingEvent),
    StartupComplete,
    StateChanged(StateChangedEvent),
}

#[derive(Debug, Deserialize)]
pub(super) struct RawEvent {
    pub id: u64,
    #[serde(rename = "globalID")]
    pub global_id: u64,
    #[serde(rename = "type")]
    pub event_type: EventType,
    pub time: String,
    pub data: Box<RawValue>,
}

#[derive(Debug, Copy, Clone, PartialEq, Deserialize, Serialize)]
pub enum EventType {
    ClusterConfigReceived,
    ConfigSaved,
    DeviceConnected,
    DeviceDisconnected,
    DeviceDiscovered,
    DevicePaused,
    DeviceRejected,
    DeviceResumed,
    DownloadProgress,
    Failure,
    FolderCompletion,
    FolderErrors,
    FolderPaused,
    FolderRejected,
    FolderResumed,
    FolderScanProgress,
    FolderSummary,
    FolderWatchStateChanged,
    ItemFinished,
    ItemStarted,
    ListenAddressesChanged,
    LocalChangeDetected,
    LocalIndexUpdated,
    LoginAttempt,
    PendingDevicesChanged,
    PendingFoldersChanged,
    RemoteChangeDetected,
    RemoteDownloadProgress,
    RemoteIndexUpdated,
    Starting,
    StartupComplete,
    StateChanged,
}

#[derive(Debug, Deserialize)]
#[serde(try_from = "RawEvent")]
pub struct Event {
    pub id: u64,
    pub global_id: u64,
    pub time: String,
    pub data: EventData,
}

impl core::convert::TryFrom<RawEvent> for Event {
    type Error = serde_json::Error;

    fn try_from(raw_event: RawEvent) -> Result<Self, Self::Error> {
        use EventData::*;
        let RawEvent {
            id,
            global_id,
            event_type,
            time,
            data,
        } = raw_event;
        let data = data.get();
        Ok(Event {
            id,
            global_id,
            time,
            data: match event_type {
                EventType::ClusterConfigReceived => ClusterConfigReceived(serde_json::from_str(data)?),
                EventType::ConfigSaved => ConfigSaved(serde_json::from_str(data)?),
                EventType::DeviceConnected => DeviceConnected(serde_json::from_str(data)?),
                EventType::DeviceDisconnected => DeviceDisconnected(serde_json::from_str(data)?),
                EventType::DeviceDiscovered => DeviceDiscovered(serde_json::from_str(data)?),
                EventType::DevicePaused => DevicePaused(serde_json::from_str(data)?),
                EventType::DeviceRejected => DeviceRejected(serde_json::from_str(data)?),
                EventType::DeviceResumed => DeviceResumed(serde_json::from_str(data)?),
                EventType::DownloadProgress => DownloadProgress(serde_json::from_str(data)?),
                EventType::Failure => Failure(serde_json::from_str(data)?),
                EventType::FolderCompletion => FolderCompletion(serde_json::from_str(data)?),
                EventType::FolderErrors => FolderErrors(serde_json::from_str(data)?),
                EventType::FolderPaused => FolderPaused(serde_json::from_str(data)?),
                EventType::FolderRejected => FolderRejected(serde_json::from_str(data)?),
                EventType::FolderResumed => FolderResumed(serde_json::from_str(data)?),
                EventType::FolderScanProgress => FolderScanProgress(serde_json::from_str(data)?),
                EventType::FolderSummary => FolderSummary(serde_json::from_str(data)?),
                EventType::FolderWatchStateChanged => FolderWatchStateChanged(serde_json::from_str(data)?),
                EventType::ItemFinished => ItemFinished(serde_json::from_str(data)?),
                EventType::ItemStarted => ItemStarted(serde_json::from_str(data)?),
                EventType::ListenAddressesChanged => {
                    ListenAddressesChanged(serde_json::from_str(data)?)
                }
                EventType::LocalChangeDetected => LocalChangeDetected(serde_json::from_str(data)?),
                EventType::LocalIndexUpdated => LocalIndexUpdated(serde_json::from_str(data)?),
                EventType::LoginAttempt => LoginAttempt(serde_json::from_str(data)?),
                EventType::PendingDevicesChanged => PendingDevicesChanged(),
                EventType::PendingFoldersChanged => PendingFoldersChanged(),
                EventType::RemoteChangeDetected => {
                    RemoteChangeDetected(serde_json::from_str(data)?)
                }
                EventType::RemoteDownloadProgress => {
                    RemoteDownloadProgress(serde_json::from_str(data)?)
                }
                EventType::RemoteIndexUpdated => RemoteIndexUpdated(serde_json::from_str(data)?),
                EventType::Starting => Starting(serde_json::from_str(data)?),
                EventType::StartupComplete => StartupComplete,
                EventType::StateChanged => StateChanged(serde_json::from_str(data)?),
            },
        })
    }
}
