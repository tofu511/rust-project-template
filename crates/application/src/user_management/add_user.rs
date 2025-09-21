//! Add-user use case.

use crate::user_management::user_command::UserCommand;
#[cfg_attr(test, mockall_double::double)]
use adapters::outbound::repository::sqlite::SqliteUserRepository;
use domain::user_management::User;

/// Errors for the add-user use case.
#[derive(Debug, Clone)]
pub struct AddUserError(pub String);

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
                self.repo
                    .create_user(&user)
                    .map_err(|e| AddUserError(e.to_string()))?;
                Ok(user)
            }
        }
    }
}

// No Default impl: repository must be supplied explicitly.

#[cfg(test)]
mod tests {
    use super::*;
    // Real DB test can be added in integration tests if desired.

    #[test]
    fn add_user_with_double_mock() {
        use mockall_double::double;

        // Replace the concrete repository type with a mock for this test scope.
        #[double]
        use adapters::outbound::repository::sqlite::SqliteUserRepository;

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
