use super::errors::{MeshError, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use vodozemac::olm::Account;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Device {
    pub id: Uuid,
    pub name: String,
    pub pickled_account: String,
    pub created_at: DateTime<Utc>,
}

impl Device {
    pub fn new(name: String) -> Result<Self> {
        let account = Account::new();
        let pickled_account = account.pickle().map_err(MeshError::Cryptography)?;
        Ok(Self {
            id: Uuid::new_v4(),
            name,
            pickled_account,
            created_at: Utc::now(),
        })
    }

    pub fn account(&self) -> Result<Account> {
        Account::from_pickle(&self.pickled_account).map_err(MeshError::Cryptography)
    }
}
