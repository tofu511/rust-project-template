/// HTTP server bootstrap.
///
/// Uses adapters' HTTP routes and starts an Actix Web server.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let addr = std::env::var("HTTP_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".to_string());

    let server = bootstrap::server(&addr)?;
    server.await
}
