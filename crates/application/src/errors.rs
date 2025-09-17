use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ApplicationError {
    NotFound,
    Infra(String),
}

impl Display for ApplicationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ApplicationError::NotFound => write!(f, "not found"),
            ApplicationError::Infra(m) => write!(f, "infra error: {m}"),
        }
    }
}

impl std::error::Error for ApplicationError {}

impl From<ports::errors::InfraError> for ApplicationError {
    fn from(e: ports::errors::InfraError) -> Self {
        ApplicationError::Infra(e.to_string())
    }
}

