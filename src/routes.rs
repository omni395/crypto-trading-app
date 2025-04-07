use actix_web::web;
use crate::websocket::ws_index; // Импортируем ws_index напрямую
use crate::binance;

// Переименовываем функцию, чтобы избежать конфликта
pub fn configure_websocket(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/ws").route(web::get().to(ws_index)));
    cfg.service(web::resource("/historical").route(web::get().to(binance::get_historical_data)));
}