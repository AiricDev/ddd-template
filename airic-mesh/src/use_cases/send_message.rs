use crate::{
    core::{
        application_message::ApplicationMessage,
        errors::Result,
        wire_message::WireMessage,
    },
    use_cases::ports::{
        mesh_repository::MeshRepository,
        network_transport::NetworkTransport,
    },
};
use std::sync::Arc;
use uuid::Uuid;

pub struct SendMessage {
    mesh_repo: Arc<dyn MeshRepository>,
    transport: Arc<dyn NetworkTransport>,
    local_device_id: Uuid,
}

impl SendMessage {
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

    pub async fn execute(
        &self,
        recipient_device_id: Uuid,
        message: ApplicationMessage,
    ) -> Result<()> {
        let mut session = self
            .mesh_repo
            .load_session(self.local_device_id, recipient_device_id)
            .await?;
        let mut olm_session = session.olm_session()?;

        let plaintext = serde_json::to_string(&message)?;
        let encrypted = olm_session.encrypt(&plaintext);

        session.pickled_session = serde_json::to_string(&olm_session.pickle())?;
        self.mesh_repo.save_session(&session).await?;

        let wire_message = WireMessage {
            ciphertext: encrypted,
            sender_device_id: self.local_device_id,
        };

        self.transport
            .send(recipient_device_id, wire_message)
            .await
    }
}
