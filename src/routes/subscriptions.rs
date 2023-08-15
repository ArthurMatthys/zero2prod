use actix_web::{web, HttpResponse, Responder};

#[derive(serde::Deserialize)]
pub struct UserSubscriber {
    name: String,
    email: String,
}

pub async fn subscribe(form: web::Form<UserSubscriber>) -> impl Responder {
    HttpResponse::Ok().finish()
}
