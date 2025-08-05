use crate::{
    core::{
        device::Device,
        errors::{MeshError, Result},
        session::Session,
    },
    use_cases::ports::mesh_repository::MeshRepository,
};
use async_trait::async_trait;
use std::{
    fs,
    path::{Path, PathBuf},
    sync::Mutex,
};
use uuid::Uuid;

pub struct FileKeystore {
    base_path: PathBuf,
    // Using a simple Mutex for now to prevent race conditions when reading/writing files.
    // A more robust solution might use file-level locking.
    _lock: Mutex<()>,
}

impl FileKeystore {
    pub fn new(base_path: impl AsRef<Path>) -> Result<Self> {
        let path = base_path.as_ref().to_path_buf();
        fs::create_dir_all(&path.join("devices"))?;
        fs::create_dir_all(&path.join("sessions"))?;
        Ok(Self {
            base_path: path,
            _lock: Mutex::new(()),
        })
    }

    fn device_path(&self, device_id: Uuid) -> PathBuf {
        self.base_path
            .join("devices")
            .join(format!("{}.json", device_id))
    }

    fn session_path(&self, local_id: Uuid, remote_id: Uuid) -> PathBuf {
        // Create a consistent name for the session file regardless of which device is local/remote
        let (first, second) = if local_id.as_bytes() < remote_id.as_bytes() {
            (local_id, remote_id)
        } else {
            (remote_id, local_id)
        };
        self.base_path
            .join("sessions")
            .join(format!("{}_{}.json", first, second))
    }
}

#[async_trait]
impl MeshRepository for FileKeystore {
    async fn save_device(&self, device: &Device) -> Result<()> {
        let path = self.device_path(device.id);
        let json = serde_json::to_string_pretty(device)?;
        fs::write(path, json)?;
        Ok(())
    }

    async fn load_device(&self, device_id: Uuid) -> Result<Device> {
        let path = self.device_path(device_id);
        if !path.exists() {
            return Err(MeshError::DeviceNotFound(device_id.to_string()));
        }
        let json = fs::read_to_string(path)?;
        let device: Device = serde_json::from_str(&json)?;
        Ok(device)
    }

    async fn get_all_device_ids(&self) -> Result<Vec<Uuid>> {
        let mut ids = Vec::new();
        let dir = self.base_path.join("devices");
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                if let Some(stem) = path.file_stem() {
                    if let Some(stem_str) = stem.to_str() {
                        if let Ok(id) = Uuid::parse_str(stem_str) {
                            ids.push(id);
                        }
                    }
                }
            }
        }
        Ok(ids)
    }

    async fn save_session(&self, session: &Session) -> Result<()> {
        let path = self.session_path(session.local_device_id, session.remote_device_id);
        let json = serde_json::to_string_pretty(session)?;
        fs::write(path, json)?;
        Ok(())
    }

    async fn load_session(&self, local_device_id: Uuid, remote_device_id: Uuid) -> Result<Session> {
        let path = self.session_path(local_device_id, remote_device_id);
        if !path.exists() {
            return Err(MeshError::SessionNotFound(
                local_device_id.to_string(),
                remote_device_id.to_string(),
            ));
        }
        let json = fs::read_to_string(path)?;
        let session: Session = serde_json::from_str(&json)?;
        Ok(session)
    }

    async fn delete_device(&self, device_id: Uuid) -> Result<()> {
        let device_path = self.device_path(device_id);
        if device_path.exists() {
            fs::remove_file(device_path)?;
        }

        // Also delete all sessions associated with this device
        let sessions_dir = self.base_path.join("sessions");
        for entry in fs::read_dir(sessions_dir)? {
            let entry = entry?;
            let path = entry.path();
            if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                if file_name.contains(&device_id.to_string()) {
                    fs::remove_file(path)?;
                }
            }
        }
        Ok(())
    }
}
