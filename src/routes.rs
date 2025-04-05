use actix_web::{web, HttpRequest, HttpResponse};
use actix_web_actors::ws;

use crate::websocket::{WebSocketActor, AppState};

pub async fn ws_handler(req: HttpRequest, stream: web::Payload, state: web::Data<AppState>) -> Result<HttpResponse, actix_web::Error> {
    ws::start(WebSocketActor::new(state.into_inner().into()), &req, stream)
}