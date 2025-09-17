use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct InfraError {
    pub kind: InfraErrorKind,
    pub message: String,
}

#[derive(Debug, Clone)]
pub enum InfraErrorKind {
    NotFound,
    Conflict,
    Unavailable,
    Unknown,
}

impl InfraError {
    pub fn new(kind: InfraErrorKind, message: impl Into<String>) -> Self {
        Self { kind, message: message.into() }
    }
}

impl Display for InfraError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {}", self.kind, self.message)
    }
}

impl std::error::Error for InfraError {}

