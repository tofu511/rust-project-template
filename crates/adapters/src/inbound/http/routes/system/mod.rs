use actix_web::{HttpResponse, Responder, web};

/// Registers system routes (e.g., health checks).
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health));
}

async fn health() -> impl Responder {
    HttpResponse::Ok()
        .content_type("application/json")
        .body("{\"status\":\"ok\"}")
}

#[cfg(test)]
mod tests {
    use super::configure;
    use actix_web::{http::StatusCode, test, App};

    #[actix_web::test]
    async fn get_health_returns_ok() {
        let app = test::init_service(App::new().configure(configure)).await;
        let req = test::TestRequest::get().uri("/health").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
