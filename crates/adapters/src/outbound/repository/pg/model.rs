#[cfg(feature = "outbound-repo-pg")]
#[derive(Debug, Clone, sqlx::FromRow)]
pub struct UserRow {
    pub id: i64,
    pub active: bool,
}

