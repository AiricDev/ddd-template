use crate::core::errors::Result;
use crate::use_cases::ports::mesh_repository::MeshRepository;
use std::sync::Arc;
use uuid::Uuid;

pub struct RevokeDevice {
    mesh_repo: Arc<dyn MeshRepository>,
}

impl RevokeDevice {
    pub fn new(mesh_repo: Arc<dyn MeshRepository>) -> Self {
        Self { mesh_repo }
    }

    pub async fn execute(&self, device_id_to_revoke: Uuid) -> Result<()> {
        // This is a simplified implementation. A full implementation would need
        // to securely notify other devices in the mesh to also discard their
        // sessions with the revoked device.
        self.mesh_repo.delete_device(device_id_to_revoke).await
    }
}
