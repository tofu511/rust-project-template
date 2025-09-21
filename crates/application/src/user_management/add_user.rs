//! Add-user use case.

use crate::user_management::user_command::UserCommand;
use adapters_outbound::repository::sqlite::SqliteRepoError;
#[cfg_attr(test, mockall_double::double)]
use adapters_outbound::repository::sqlite::SqliteUserRepository;
use domain::user_management::User;

/// Errors for the add-user use case.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddUserError {
    DuplicateEmail,
    Persist(String),
}

/// Adds a new user by validating input, constructing the domain entity,
/// and persisting it via the repository abstraction.
pub struct AddUserUseCase {
    repo: SqliteUserRepository,
}

impl AddUserUseCase {
    /// Creates a new use case instance with a concrete SQLite repository.
    pub fn new(repo: SqliteUserRepository) -> Self {
        Self { repo }
    }

    /// Executes the command and returns the created user on success.
    pub fn execute(&self, cmd: UserCommand) -> Result<User, AddUserError> {
        match cmd {
            UserCommand::Add {
                email,
                first_name,
                last_name,
            } => {
                let user = User::new(email, first_name, last_name);
                self.repo.create_user(&user).map_err(|e| match e {
                    SqliteRepoError::DuplicateEmail => AddUserError::DuplicateEmail,
                    SqliteRepoError::Sql(err) => AddUserError::Persist(err.to_string()),
                })?;
                Ok(user)
            }
        }
    }
}

// No Default impl: repository must be supplied explicitly.

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_user_with_double_mock_success() {
        let mut repo = SqliteUserRepository::default();
        repo.expect_create_user().returning(|_u| Ok(()));

        let uc = AddUserUseCase::new(repo);
        let out = uc
            .execute(UserCommand::Add {
                email: domain::user_management::Email::parse("carol@example.com").unwrap(),
                first_name: domain::user_management::FirstName::parse("Carol").unwrap(),
                last_name: domain::user_management::LastName::parse("Clark").unwrap(),
            })
            .expect("should persist via double mock");
        assert_eq!(out.email.as_str(), "carol@example.com");
    }

    #[test]
    fn add_user_with_double_mock_duplicate_maps_error() {
        let mut repo = SqliteUserRepository::default();
        repo.expect_create_user()
            .returning(|_u| Err(SqliteRepoError::DuplicateEmail));

        let uc = AddUserUseCase::new(repo);
        let err = uc
            .execute(UserCommand::Add {
                email: domain::user_management::Email::parse("dana@example.com").unwrap(),
                first_name: domain::user_management::FirstName::parse("Dana").unwrap(),
                last_name: domain::user_management::LastName::parse("Doe").unwrap(),
            })
            .expect_err("should map duplicate error");
        assert_eq!(err, AddUserError::DuplicateEmail);
    }

    #[test]
    fn add_user_with_double_mock() {
        use mockall_double::double;

        // Replace the concrete repository type with a mock for this test scope.
        #[double]
        use adapters_outbound::repository::sqlite::SqliteUserRepository;

        let mut repo = SqliteUserRepository::default();
        repo.expect_create_user().returning(|_u| Ok(()));

        let uc = AddUserUseCase::new(repo);
        let out = uc
            .execute(UserCommand::Add {
                email: domain::user_management::Email::parse("carol@example.com").unwrap(),
                first_name: domain::user_management::FirstName::parse("Carol").unwrap(),
                last_name: domain::user_management::LastName::parse("Clark").unwrap(),
            })
            .expect("should persist via double mock");

        assert_eq!(out.email.as_str(), "carol@example.com");
    }
}
