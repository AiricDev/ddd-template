use super::errors::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use vodozemac::olm::{Account, AccountPickle};

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
        let pickled_account = serde_json::to_string(&account.pickle())?;
        Ok(Self {
            id: Uuid::new_v4(),
            name,
            pickled_account,
            created_at: Utc::now(),
        })
    }

    pub fn account(&self) -> Result<Account> {
        let pickle: AccountPickle = serde_json::from_str(&self.pickled_account)?;
        Ok(pickle.into())
    }
}
