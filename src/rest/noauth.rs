use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct HealthInfo {
    pub status: String,
}