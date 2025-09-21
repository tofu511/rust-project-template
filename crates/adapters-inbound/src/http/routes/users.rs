use actix_web::{HttpResponse, Responder, web};
use adapters_outbound::repository::sqlite::SqliteUserRepository;
use application::user_management::{
    add_user::{AddUserError, AddUserUseCase},
    user_command::UserCommand,
};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Serialize)]
pub struct CreateUserResponse {
    pub user_id: String,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/users").route("/", web::post().to(create_user)));
}

async fn create_user(payload: web::Json<CreateUserRequest>) -> impl Responder {
    // Build concrete repository and inject into use-case
    let db_path = std::env::var("SQLITE_DB_PATH").unwrap_or_else(|_| "app.db".to_string());
    let repo = if db_path == ":memory:" {
        SqliteUserRepository::open_in_memory()
            .map_err(|e| HttpResponse::InternalServerError().body(e.to_string()))
            .unwrap()
    } else {
        SqliteUserRepository::new(&db_path)
            .map_err(|e| HttpResponse::InternalServerError().body(e.to_string()))
            .unwrap()
    };
    let uc = AddUserUseCase::new(repo);

    // Validate input via domain value objects
    let email = match domain::user_management::Email::parse(payload.email.clone()) {
        Ok(v) => v,
        Err(_) => return HttpResponse::BadRequest().body("invalid email"),
    };
    let first = match domain::user_management::FirstName::parse(payload.first_name.clone()) {
        Ok(v) => v,
        Err(_) => return HttpResponse::BadRequest().body("invalid first_name"),
    };
    let last = match domain::user_management::LastName::parse(payload.last_name.clone()) {
        Ok(v) => v,
        Err(_) => return HttpResponse::BadRequest().body("invalid last_name"),
    };

    match uc.execute(UserCommand::Add {
        email,
        first_name: first,
        last_name: last,
    }) {
        Ok(user) => {
            let resp = CreateUserResponse {
                user_id: user.user_id.to_string(),
                email: user.email.as_str().to_string(),
                first_name: user.first_name.as_str().to_string(),
                last_name: user.last_name.as_str().to_string(),
                status: match user.status {
                    domain::user_management::UserStatus::Active => "active".to_string(),
                    domain::user_management::UserStatus::Inactive => "inactive".to_string(),
                    domain::user_management::UserStatus::Suspended => "suspended".to_string(),
                },
                created_at: user.created_at.to_rfc3339(),
                updated_at: user.updated_at.to_rfc3339(),
            };
            HttpResponse::Created().json(resp)
        }
        Err(AddUserError::DuplicateEmail) => HttpResponse::Conflict().body("duplicate email"),
        Err(AddUserError::Persist(e)) => HttpResponse::InternalServerError().body(e),
    }
}

#[cfg(test)]
mod tests {}
