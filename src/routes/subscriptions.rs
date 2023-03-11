use actix_web::{web, HttpResponse, Responder};

#[derive(serde::Deserialize)]
pub struct SubscriptionInfo {
    name: String,
    email: String,
}

pub async fn subscribe(form: web::Form<SubscriptionInfo>) -> impl Responder {
    HttpResponse::Ok()
}
