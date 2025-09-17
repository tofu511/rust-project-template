use std::time::SystemTime;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct UserId(pub u64);

impl From<u64> for UserId {
    fn from(v: u64) -> Self {
        UserId(v)
    }
}

#[derive(Clone, Debug)]
pub struct User {
    pub id: UserId,
    pub active: bool,
}

impl User {
    pub fn new(id: UserId) -> Self {
        Self { id, active: false }
    }

    pub fn activate(&mut self, now: SystemTime) -> crate::events::DomainEvent {
        self.active = true;
        crate::events::DomainEvent::UserActivated {
            user_id: self.id,
            at: now,
        }
    }
}

