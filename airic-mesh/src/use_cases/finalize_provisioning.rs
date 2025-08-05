use crate::core::errors::{MeshError, Result};
use crate::core::session::Session;
use crate::core::wire_message::WireMessage;
use crate::use_cases::ports::mesh_repository::MeshRepository;
use crate::use_cases::ports::network_transport::NetworkTransport;
use serde_json::Value;
use std::sync::Arc;
use uuid::Uuid;

pub struct FinalizeProvisioning {
    mesh_repo: Arc<dyn MeshRepository>,
    transport: Arc<dyn NetworkTransport>,
    local_device_id: Uuid,
}

impl FinalizeProvisioning {
    pub fn new(
        mesh_repo: Arc<dyn MeshRepository>,
        transport: Arc<dyn NetworkTransport>,
        local_device_id: Uuid,
    ) -> Self {
        Self {
            mesh_repo,
            transport,
            local_device_id,
        }
    }

    pub async fn execute(&self, invitation: &str) -> Result<()> {
        let invitation: Value = serde_json::from_str(invitation)?;

        let remote_device_id = Uuid::parse_str(
            invitation["device_id"]
                .as_str()
                .ok_or_else(|| MeshError::Serialization(serde_json::Error::custom("missing device_id")))?,
        )
        .map_err(|_| MeshError::Serialization(serde_json::Error::custom("invalid device_id")))?;

        let remote_identity_key = invitation["identity_key"]
            .as_str()
            .ok_or_else(|| MeshError::Serialization(serde_json::Error::custom("missing identity_key")))?;

        let remote_one_time_key = invitation["one_time_key"]
            .as_str()
            .ok_or_else(|| MeshError::Serialization(serde_json::Error::custom("missing one_time_key")))?;

        let local_device = self.mesh_repo.load_device(self.local_device_id).await?;
        let local_account = local_device.account()?;

        let (session, first_message) = Session::new_outbound(
            self.local_device_id,
            remote_device_id,
            &local_account,
            remote_identity_key,
            remote_one_time_key,
        )?;

        self.mesh_repo.save_session(&session).await?;

        let wire_message = WireMessage {
            ciphertext: first_message,
            sender_device_id: self.local_device_id,
        };

        self.transport
            .send(remote_device_id, wire_message)
            .await?;

        Ok(())
    }
}
// Note: serde_json::Error doesn't have a constructor for custom messages,
// so I'm using a placeholder. A more robust implementation might define a more specific error type.
use serde::ser::Error as SerdeError;
