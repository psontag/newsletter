use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpResponse, HttpServer, Responder};

#[derive(serde::Deserialize)]
struct SubscriptionInfo {
    name: String,
    email: String,
}

async fn health() -> impl Responder {
    HttpResponse::Ok()
}
async fn subscribe(form: web::Form<SubscriptionInfo>) -> impl Responder {
    HttpResponse::Ok()
}
pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health", web::get().to(health))
            .route("/subscriptions", web::post().to(subscribe))
    })
    .listen(listener)?
    .run();
    Ok(server)
}
