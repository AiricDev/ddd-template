use crate::core::device::Device;
use crate::core::errors::Result;
use crate::use_cases::ports::mesh_repository::MeshRepository;
use serde_json::json;
use std::sync::Arc;

pub struct ProvisionNewDevice {
    mesh_repo: Arc<dyn MeshRepository>,
}

impl ProvisionNewDevice {
    pub fn new(mesh_repo: Arc<dyn MeshRepository>) -> Self {
        Self { mesh_repo }
    }

    pub async fn execute(&self, name: String) -> Result<(Device, String)> {
        let device = Device::new(name)?;

        self.mesh_repo.save_device(&device).await?;

        let mut account = device.account()?;
        
        // Generate one-time keys for the invitation
        account.generate_one_time_keys(10);
        
        let identity_keys = account.identity_keys();
        let one_time_keys = account.one_time_keys();

        // This is a simplified invitation for demonstration.
        // A real implementation would involve a more robust mechanism
        // for sharing and using one-time keys.
        let invitation = json!({
            "device_id": device.id.to_string(),
            "identity_key": identity_keys.curve25519,
            "one_time_key": one_time_keys.values().next().unwrap(),
        })
        .to_string();

        Ok((device, invitation))
    }
}
