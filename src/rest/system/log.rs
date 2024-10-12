use crate::rest::system::LogEntry;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Log {
    pub messages: Vec<LogEntry>,
}
