use async_trait::async_trait;
use uuid::Uuid;

use crate::core::{
    device::Device,
    errors::{Result},
    session::Session,
};

#[async_trait]
pub trait MeshRepository: Send + Sync {
    async fn save_device(&self, device: &Device) -> Result<()>;
    async fn load_device(&self, device_id: Uuid) -> Result<Device>;
    async fn get_all_device_ids(&self) -> Result<Vec<Uuid>>;
    async fn save_session(&self, session: &Session) -> Result<()>;
    async fn load_session(&self, local_device_id: Uuid, remote_device_id: Uuid) -> Result<Session>;
    async fn delete_device(&self, device_id: Uuid) -> Result<()>;
}
