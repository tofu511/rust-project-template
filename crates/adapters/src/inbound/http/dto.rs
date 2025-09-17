#[cfg(feature = "inbound-http")]
#[derive(Debug, Clone)]
pub struct ActivateUserRequest {
    pub id: u64,
}

