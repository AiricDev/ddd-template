use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WireMessage {
    pub ciphertext: String,
    pub sender_device_id: Uuid,
}
