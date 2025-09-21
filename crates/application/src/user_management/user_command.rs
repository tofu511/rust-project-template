//! Commands for user management use-cases.

use domain::user_management::{Email, FirstName, LastName};

/// Command envelope for user-related operations.
#[derive(Debug, Clone)]
pub enum UserCommand {
    /// Adds a new user with the provided attributes.
    Add {
        email: Email,
        first_name: FirstName,
        last_name: LastName,
    },
}
