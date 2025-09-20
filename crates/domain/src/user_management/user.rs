//! User-related domain types.

use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Unique identifier for a [`User`].
pub type UserId = Uuid;

/// Status of a user account.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UserStatus {
    /// Active and usable account.
    Active,
    /// Disabled or deactivated account.
    Inactive,
    /// Suspended due to policy or security reasons.
    Suspended,
}

/// Email value object with basic validation.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Email(String);

impl Email {
    /// Parses and validates an email.
    pub fn parse<S: Into<String>>(s: S) -> Result<Self, EmailError> {
        let s = s.into();
        let trimmed = s.trim();
        if trimmed.is_empty() {
            return Err(EmailError::Empty);
        }
        if !trimmed.contains('@') {
            return Err(EmailError::InvalidFormat);
        }
        Ok(Self(trimmed.to_string()))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Errors for [`Email`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmailError {
    Empty,
    InvalidFormat,
}

/// First name value object (non-empty, trimmed).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct FirstName(String);

impl FirstName {
    pub fn parse<S: Into<String>>(s: S) -> Result<Self, NameError> {
        let s = s.into();
        let trimmed = s.trim();
        if trimmed.is_empty() {
            return Err(NameError::Empty);
        }
        Ok(Self(trimmed.to_string()))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Last name value object (non-empty, trimmed).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct LastName(String);

impl LastName {
    pub fn parse<S: Into<String>>(s: S) -> Result<Self, NameError> {
        let s = s.into();
        let trimmed = s.trim();
        if trimmed.is_empty() {
            return Err(NameError::Empty);
        }
        Ok(Self(trimmed.to_string()))
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Errors for name value objects.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NameError {
    Empty,
}

/// User aggregate root.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    /// Unique identifier.
    pub user_id: UserId,
    /// Email address.
    pub email: Email,
    /// First name (given name).
    pub first_name: FirstName,
    /// Last name (family name).
    pub last_name: LastName,
    /// Current status.
    pub status: UserStatus,
    /// Creation timestamp.
    pub created_at: DateTime<Utc>,
    /// Last update timestamp.
    pub updated_at: DateTime<Utc>,
}

impl User {
    /// Creates a new user with a freshly generated [`UserId`].
    #[must_use]
    pub fn new(email: Email, first_name: FirstName, last_name: LastName) -> Self {
        let now = Utc::now();
        Self {
            user_id: Uuid::new_v4(),
            email,
            first_name,
            last_name,
            status: UserStatus::Active,
            created_at: now,
            updated_at: now,
        }
    }

    /// Updates `updated_at` to the current time.
    pub fn touch(&mut self) {
        self.updated_at = Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_user_has_id_and_timestamps() {
        let u = User::new(
            Email::parse("alice@example.com").unwrap(),
            FirstName::parse("Alice").unwrap(),
            LastName::parse("Anderson").unwrap(),
        );
        assert_ne!(u.user_id, Uuid::nil());
        assert_eq!(u.status, UserStatus::Active);
        assert!(u.updated_at >= u.created_at);
    }
}
