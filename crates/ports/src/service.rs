use async_trait::async_trait;
use domain::UserId;

use crate::errors::InfraError;

#[async_trait]
pub trait ProfileService: Send + Sync {
    async fn enrich_user(&self, id: &UserId) -> Result<(), InfraError>;
}

