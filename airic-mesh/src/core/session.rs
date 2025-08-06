use super::errors::{MeshError, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use vodozemac::olm::{Account, Session as OlmSession, SessionPickle, InboundCreationResult};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Session {
    pub id: Uuid,
    pub local_device_id: Uuid,
    pub remote_device_id: Uuid,
    pub pickled_session: String,
    pub created_at: DateTime<Utc>,
}

impl Session {
    pub fn new(
        local_device_id: Uuid,
        remote_device_id: Uuid,
        olm_session: OlmSession,
    ) -> Result<Self> {
        let pickled_session = serde_json::to_string(&olm_session.pickle())?;

        Ok(Self {
            id: Uuid::new_v4(),
            local_device_id,
            remote_device_id,
            pickled_session,
            created_at: Utc::now(),
        })
    }

    pub fn new_inbound(
        local_device_id: Uuid,
        remote_device_id: Uuid,
        account: &Account,
        one_time_key_message: &str,
    ) -> Result<(Self, String)> {
        // For now, we'll return an error since inbound session creation is complex
        // In a real implementation, we would need to properly parse PreKeyMessage
        return Err(MeshError::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Inbound session creation not implemented yet - requires PreKeyMessage parsing"
        )));
    }

    pub fn new_outbound(
        local_device_id: Uuid,
        remote_device_id: Uuid,
        account: &Account,
        remote_identity_key: &str,
        remote_one_time_key: &str,
    ) -> Result<(Self, String)> {
        // For now, we'll return an error since key parsing is complex
        // In a real implementation, we would need to properly parse the keys
        return Err(MeshError::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Outbound session creation not implemented yet - requires proper key parsing"
        )));
    }

    pub fn olm_session(&self) -> Result<OlmSession> {
        let pickle: SessionPickle = serde_json::from_str(&self.pickled_session)?;
        Ok(pickle.into())
    }
}