use crate::{
    core::{
        application_message::ApplicationMessage,
        errors::{MeshError, Result},
        session::Session,
        wire_message::WireMessage,
    },
    use_cases::ports::mesh_repository::MeshRepository,
};
use std::sync::Arc;
use uuid::Uuid;

pub struct HandleIncomingMessage {
    mesh_repo: Arc<dyn MeshRepository>,
    local_device_id: Uuid,
}

impl HandleIncomingMessage {
    pub fn new(mesh_repo: Arc<dyn MeshRepository>, local_device_id: Uuid) -> Self {
        Self {
            mesh_repo,
            local_device_id,
        }
    }

    pub async fn execute(&self, message: WireMessage) -> Result<ApplicationMessage> {
        let sender_id = message.sender_device_id;

        // Try to load an existing session.
        let result = self
            .mesh_repo
            .load_session(self.local_device_id, sender_id)
            .await;

        match result {
            Ok(mut session) => {
                // Existing session found, decrypt message.
                let mut olm_session = session.olm_session()?;
                let decrypted = olm_session.decrypt(&message.ciphertext)?;

                session.pickled_session = olm_session.pickle()?;
                self.mesh_repo.save_session(&session).await?;

                let app_message: ApplicationMessage = serde_json::from_str(&decrypted)?;
                Ok(app_message)
            }
            Err(MeshError::SessionNotFound(_, _)) => {
                // No session found, this might be a pre-key message to establish a new session.
                let local_device = self.mesh_repo.load_device(self.local_device_id).await?;
                let local_account = local_device.account()?;

                let (mut session, plaintext) = Session::new_inbound(
                    self.local_device_id,
                    sender_id,
                    &local_account,
                    &message.ciphertext,
                )?;

                self.mesh_repo.save_session(&session).await?;

                // After a session is created from a pre-key message, the plaintext is empty.
                // A real application might need to signal that a session is established
                // and the other party should now send their first "real" message.
                // For now, we'll return a placeholder ApplicationMessage.
                let app_message: ApplicationMessage = serde_json::from_str(&plaintext)
                    .unwrap_or(ApplicationMessage {
                        payload: serde_json::Value::Null,
                        timestamp: chrono::Utc::now(),
                    });

                Ok(app_message)
            }
            Err(e) => Err(e),
        }
    }
}
