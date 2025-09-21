use domain::user_management::{User, UserStatus};
use rusqlite::{Connection, Error as SqlError, ErrorCode, Result as SqlResult, params};
use thiserror::Error;

/// SQLite-backed repository for persisting users.
pub struct SqliteUserRepository {
    conn: Connection,
}

impl SqliteUserRepository {
    /// Opens (or creates) a SQLite database at `path`.
    pub fn new(path: impl AsRef<std::path::Path>) -> SqlResult<Self> {
        let conn = Connection::open(path)?;
        Ok(Self { conn })
    }

    /// Opens an in-memory SQLite database (useful for tests).
    pub fn open_in_memory() -> SqlResult<Self> {
        let conn = Connection::open_in_memory()?;
        Ok(Self { conn })
    }
}

#[cfg_attr(any(test, feature = "mocking"), mockall::automock)]
impl SqliteUserRepository {
    /// Initializes schema objects required for user persistence.
    pub fn init_schema(&self) -> SqlResult<()> {
        self.conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS users (
                user_id     TEXT PRIMARY KEY,
                email       TEXT NOT NULL UNIQUE,
                first_name  TEXT NOT NULL,
                last_name   TEXT NOT NULL,
                status      TEXT NOT NULL,
                created_at  TEXT NOT NULL,
                updated_at  TEXT NOT NULL
            );
            "#,
        )
    }

    /// Persists a newly created user.
    pub fn create_user(&self, user: &User) -> Result<(), SqliteRepoError> {
        // Ensure timestamps are sensible (defensive in case callers modify values).
        let created_at = user.created_at.to_rfc3339();
        let updated_at = user.updated_at.to_rfc3339();
        self.conn
            .execute(
                r#"
            INSERT INTO users (
                user_id, email, first_name, last_name, status, created_at, updated_at
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
            "#,
                params![
                    user.user_id.as_bytes(),
                    user.email.as_str(),
                    user.first_name.as_str(),
                    user.last_name.as_str(),
                    status_to_str(user.status),
                    created_at,
                    updated_at,
                ],
            )
            .map(|_| ())
            .map_err(Into::into)
    }

    /// Fetches a user count (utility for tests/verification).
    pub fn count_users(&self) -> SqlResult<i64> {
        self.conn
            .query_row("SELECT COUNT(1) FROM users", [], |row| row.get(0))
    }
}

/// Adapter-specific repository error.
#[derive(Debug, Error)]
pub enum SqliteRepoError {
    /// Email is duplicated (violates UNIQUE constraint).
    #[error("duplicate email")]
    DuplicateEmail,
    /// Other underlying SQLite error.
    #[error(transparent)]
    Sql(SqlError),
}

impl From<SqlError> for SqliteRepoError {
    fn from(e: SqlError) -> Self {
        match &e {
            SqlError::SqliteFailure(err, _)
                if err.code == ErrorCode::ConstraintViolation && err.extended_code == 2067 =>
            {
                // 2067 == SQLITE_CONSTRAINT_UNIQUE
                SqliteRepoError::DuplicateEmail
            }
            _ => SqliteRepoError::Sql(e),
        }
    }
}

fn status_to_str(s: UserStatus) -> &'static str {
    match s {
        UserStatus::Active => "active",
        UserStatus::Inactive => "inactive",
        UserStatus::Suspended => "suspended",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::user_management::{Email, FirstName, LastName};
    // No need for `mock!` macro: `#[automock]` on impl generates MockSqliteUserRepository.

    #[test]
    fn create_and_count_user_in_memory() {
        let repo = SqliteUserRepository {
            conn: Connection::open_in_memory().unwrap(),
        };
        repo.init_schema().unwrap();

        let user = User::new(
            Email::parse("alice@example.com").unwrap(),
            FirstName::parse("Alice").unwrap(),
            LastName::parse("Anderson").unwrap(),
        );
        repo.create_user(&user).unwrap();
        let count = repo.count_users().unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn mock_repository_happy_path() {
        let mut repo = MockSqliteUserRepository::new();
        repo.expect_init_schema().returning(|| Ok(()));
        repo.expect_create_user().returning(|_user| Ok(()));
        repo.expect_count_users().returning(|| Ok(1));

        // Exercise mocked methods
        repo.init_schema().unwrap();
        let user = User::new(
            Email::parse("eve@example.com").unwrap(),
            FirstName::parse("Eve").unwrap(),
            LastName::parse("Evans").unwrap(),
        );
        repo.create_user(&user).unwrap();
        let count = repo.count_users().unwrap();
        assert_eq!(count, 1);
    }
}
