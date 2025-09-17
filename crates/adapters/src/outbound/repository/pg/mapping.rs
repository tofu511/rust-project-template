#[cfg(feature = "outbound-repo-pg")]
use domain::{User, UserId};

#[cfg(feature = "outbound-repo-pg")]
pub fn to_domain(row: super::model::UserRow) -> User {
    User { id: UserId(row.id as u64), active: row.active }
}

#[cfg(feature = "outbound-repo-pg")]
pub fn from_domain(u: &User) -> super::model::UserRow {
    super::model::UserRow { id: u.id.0 as i64, active: u.active }
}

