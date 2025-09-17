use std::time::SystemTime;

use crate::types::UserId;

#[derive(Clone, Debug)]
pub enum DomainEvent {
    UserActivated { user_id: UserId, at: SystemTime },
}

