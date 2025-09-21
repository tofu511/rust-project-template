//! User management domain module.

pub mod user;
pub use user::{Email, EmailError, FirstName, LastName, NameError, User, UserId, UserStatus};
