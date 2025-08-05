use serde::{Deserialize, Serialize};
use uuid::Uuid;
use vodozemac::olm::OlmMessage;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct WireMessage {
    pub ciphertext: OlmMessage,
    pub sender_device_id: Uuid,
}
