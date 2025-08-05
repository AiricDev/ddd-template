use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApplicationMessage {
    pub payload: Value,
    pub timestamp: DateTime<Utc>,
}
