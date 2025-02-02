use crate::rest::DeviceID;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct TotalInfo {
    pub at: String,
    #[serde(rename = "inBytesTotal")]
    pub in_bytes_total: u64,
    #[serde(rename = "outBytesTotal")]
    pub out_bytes_total: u64,
}

#[derive(Debug, Deserialize)]
pub struct DeviceStats {
    pub address: String,
    pub at: String,
    //TODO: enum
    #[serde(rename = "clientVersion")]
    pub client_version: String,
    pub connected: bool,
    #[serde(rename = "inBytesTotal")]
    pub in_bytes_total: u64,
    #[serde(rename = "isLocal")]
    pub is_local: bool,
    #[serde(rename = "outBytesTotal")]
    pub out_bytes_total: u64,
    pub paused: bool,
    #[serde(rename = "startedAt")]
    pub started_at: String,
    #[serde(rename = "type")]
    pub device_type: String,
}

#[derive(Debug, Deserialize)]
pub struct Connections {
    pub total: TotalInfo,
    pub connections: HashMap<DeviceID, DeviceStats>,
}
