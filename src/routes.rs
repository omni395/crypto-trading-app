use actix_web::{get, web, HttpResponse};

use crate::app_state::AppState;
use crate::binance;
use crate::websocket::ws_index;

#[get("/historical")]
async fn historical(
    query: web::Query<binance::HistoricalRequest>,
    state: web::Data<AppState>,
) -> HttpResponse {
    binance::get_historical_data(query, state).await
}

pub fn configure_websocket(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/ws").route(web::get().to(ws_index)))
        .service(historical);
}