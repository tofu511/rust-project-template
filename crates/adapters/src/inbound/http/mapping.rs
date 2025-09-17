#[cfg(feature = "inbound-http")]
use domain::UserId;

#[cfg(feature = "inbound-http")]
#[allow(dead_code)]
pub fn to_user_id(req: super::dto::ActivateUserRequest) -> UserId {
    UserId(req.id)
}

