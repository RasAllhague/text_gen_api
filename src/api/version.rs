use actix_web::{get, Responder, HttpResponse};

#[get("/version")]
pub async fn get_version() -> impl Responder {
    HttpResponse::Ok().body(env!("CARGO_PKG_VERSION"))
}