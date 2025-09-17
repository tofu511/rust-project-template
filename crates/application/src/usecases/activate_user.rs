use std::time::SystemTime;

use domain::{events::DomainEvent, User, UserId};
use observability::{context::Context, telemetry};
use ports::{publisher::EventPublisher, repository::UserRepo, service::ProfileService};

use crate::errors::ApplicationError;

#[allow(clippy::too_many_arguments)]
pub async fn activate_user<R, P, S>(
    _ctx: &Context,
    now: SystemTime,
    id: UserId,
    repo: &R,
    publisher: &P,
    profile_svc: &S,
) -> Result<User, ApplicationError>
where
    R: UserRepo + ?Sized,
    P: EventPublisher + ?Sized,
    S: ProfileService + ?Sized,
{
    let _span = telemetry::usecase_span("activate_user");

    let maybe = repo.get_by_id(&id).await?;
    let mut user = match maybe {
        Some(u) => u,
        None => return Err(ApplicationError::NotFound),
    };

    let evt: DomainEvent = user.activate(now);

    repo.save(&user).await?;
    publisher.publish(&evt).await?;
    profile_svc.enrich_user(&id).await?;

    Ok(user)
}

