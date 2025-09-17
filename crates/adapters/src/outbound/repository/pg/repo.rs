#[cfg(feature = "outbound-repo-pg")]
use async_trait::async_trait;
#[cfg(feature = "outbound-repo-pg")]
use domain::{User, UserId};
#[cfg(feature = "outbound-repo-pg")]
use ports::{errors::InfraError, repository::UserRepo};

#[cfg(feature = "outbound-repo-pg")]
pub struct PgUserRepo;

#[cfg(feature = "outbound-repo-pg")]
#[async_trait]
impl UserRepo for PgUserRepo {
    async fn get_by_id(&self, _id: &UserId) -> Result<Option<User>, InfraError> {
        Ok(None)
    }
    async fn save(&self, _user: &User) -> Result<(), InfraError> {
        Ok(())
    }
}

