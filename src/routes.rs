use actix_web::web;
use crate::websocket;

pub fn websocket(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/ws").route(web::get().to(websocket::ws_index)));
}