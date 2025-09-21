#![cfg(test)]

use actix_web::{http::StatusCode, test, App};
use adapters_inbound::http::routes::users;
use adapters_outbound::repository::sqlite::SqliteUserRepository;
use domain::user_management::{Email, FirstName, LastName, User};

fn prepare_fresh_db(path: &str) {
    let _ = std::fs::remove_file(path);
    let repo = SqliteUserRepository::new(path).expect("open db");
    repo.init_schema().expect("init schema");
}

#[ignore]
#[actix_web::test]
async fn post_users_creates_user_returns_created() {
    let db_path = "app.db";
    prepare_fresh_db(db_path);

    let app = test::init_service(App::new().configure(users::configure)).await;

    let body = serde_json::json!({
        "email": "alice@example.com",
        "first_name": "Alice",
        "last_name": "Anderson"
    });

    let req = test::TestRequest::post().uri("/users/").set_json(&body).to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::CREATED);

    // Optionally, verify response JSON shape
    let bytes = test::read_body(resp).await;
    let v: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(v["email"], "alice@example.com");
}

#[ignore]
#[actix_web::test]
async fn post_users_duplicate_returns_conflict() {
    let db_path = "app.db";
    prepare_fresh_db(db_path);

    // Seed an existing user directly via repository
    let repo = SqliteUserRepository::new(db_path).unwrap();
    let user = User::new(
        Email::parse("bob@example.com").unwrap(),
        FirstName::parse("Bob").unwrap(),
        LastName::parse("Builder").unwrap(),
    );
    repo.create_user(&user).unwrap();

    let app = test::init_service(App::new().configure(users::configure)).await;
    let body = serde_json::json!({
        "email": "bob@example.com",
        "first_name": "Bob",
        "last_name": "Builder"
    });
    let req = test::TestRequest::post().uri("/users/").set_json(&body).to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::CONFLICT);
}
