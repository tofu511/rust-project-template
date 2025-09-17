use async_trait::async_trait;
use domain::events::DomainEvent;

use crate::errors::InfraError;

#[async_trait]
pub trait EventPublisher: Send + Sync {
    async fn publish(&self, event: &DomainEvent) -> Result<(), InfraError>;
}

