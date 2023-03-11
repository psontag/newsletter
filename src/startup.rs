use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpServer};
use sqlx::{PgPool};

use crate::routes;

pub fn run(listener: TcpListener, pool: PgPool) -> Result<Server, std::io::Error> {
    // Wrap connection in a smart pointer so it is cloneable and can be moved with the
    // closure.
    let pool = web::Data::new(pool);
    let server = HttpServer::new(move || {
        App::new()
            .route("/health", web::get().to(routes::health))
            .route("/subscriptions", web::post().to(routes::subscribe))
            .app_data(pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
