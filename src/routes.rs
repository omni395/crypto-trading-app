use actix_web::{get, web, HttpRequest, Responder};
use actix_web_actors::ws;

use crate::websocket::WebSocketActor;
use crate::app_state::AppState;

#[get("/ws")]
pub async fn websocket(
    req: HttpRequest,
    stream: web::Payload,
    app_state: web::Data<AppState>,
) -> impl Responder {
    ws::start(WebSocketActor::new(app_state), &req, stream) // app_state уже имеет тип web::Data<AppState>
}