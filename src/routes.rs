use actix_web::web;
use crate::websocket;
use crate::binance;

pub fn websocket(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/ws").route(web::get().to(websocket::ws_index)));
    cfg.service(web::resource("/historical").route(web::get().to(binance::get_historical_data)));
}