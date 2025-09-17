use async_trait::async_trait;
use domain::{User, UserId};

use crate::errors::InfraError;

#[async_trait]
pub trait UserRepo: Send + Sync {
    async fn get_by_id(&self, id: &UserId) -> Result<Option<User>, InfraError>;
    async fn save(&self, user: &User) -> Result<(), InfraError>;
}

