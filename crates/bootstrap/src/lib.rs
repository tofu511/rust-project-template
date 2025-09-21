use actix_web::{App, HttpServer};

/// Build the Actix Web server used by the HTTP binary.
pub fn server(addr: &str) -> std::io::Result<actix_web::dev::Server> {
    let srv = HttpServer::new(|| {
        App::new()
            .configure(adapters_inbound::http::routes::system::configure)
            .configure(adapters_inbound::http::routes::users::configure)
    })
    .bind(addr)?
    .run();

    Ok(srv)
}
