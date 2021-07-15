use std::net::TcpListener;

use actix_web::{App, HttpServer, dev::Server, web};

use crate::routes::{health_check, subscribe};

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    // handles all transport level concerns
    let server = HttpServer::new(|| {
        // handles all application logic -- routing, middleware, request handlers, etc.
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
