use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum DomainError {
    NotFound,
    InvalidState(&'static str),
}

impl Display for DomainError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DomainError::NotFound => write!(f, "not found"),
            DomainError::InvalidState(m) => write!(f, "invalid state: {m}"),
        }
    }
}

impl std::error::Error for DomainError {}

