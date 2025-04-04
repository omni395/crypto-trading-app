use actix_web::{web, HttpResponse, Responder};
use actix_web::get;
use crate::websocket;

#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Привет от Rust!")
}

pub async fn ws_handler(req: actix_web::HttpRequest, stream: web::Payload, state: web::Data<websocket::AppState>) -> actix_web::Result<HttpResponse> {
    websocket::ws_handler(req, stream, state).await
}