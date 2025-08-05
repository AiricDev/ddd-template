use super::errors::{MeshError, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use vodozemac::olm::{Account, InboundCreationResult, Session as OlmSession};

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
        let pickled_session = olm_session.pickle().map_err(MeshError::Cryptography)?;

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
        let InboundCreationResult {
            session: olm_session,
            plaintext,
        } = account
            .create_inbound_session_from(one_time_key_message)
            .map_err(MeshError::Cryptography)?;

        let session = Session::new(local_device_id, remote_device_id, olm_session)?;
        let plaintext = String::from_utf8_lossy(&plaintext).to_string();

        Ok((session, plaintext))
    }

    pub fn new_outbound(
        local_device_id: Uuid,
        remote_device_id: Uuid,
        account: &Account,
        remote_identity_key: &str,
        remote_one_time_key: &str,
    ) -> Result<(Self, String)> {
        let (olm_session, first_message) = account
            .create_outbound_session(remote_identity_key, remote_one_time_key)
            .map_err(MeshError::Cryptography)?;

        let session = Session::new(local_device_id, remote_device_id, olm_session)?;

        Ok((session, first_message))
    }

    pub fn olm_session(&self) -> Result<OlmSession> {
        OlmSession::from_pickle(&self.pickled_session).map_err(MeshError::Cryptography)
    }
}
