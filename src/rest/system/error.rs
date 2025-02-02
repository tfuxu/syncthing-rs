use crate::rest::system::LogEntry;
use crate::utils;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Error {
    #[serde(deserialize_with = "utils::default_on_null")]
    pub errors: Vec<LogEntry>,
}
