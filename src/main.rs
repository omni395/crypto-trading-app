use actix_web::{App, HttpServer};
use log::info;

mod app_state;
mod binance;
mod models;
mod routes;
mod websocket;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let app_state = actix_web::web::Data::new(app_state::AppState::new());
    let app_state_clone = app_state.clone();

    let server = HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .configure(routes::websocket)
            .wrap(actix_web::middleware::Logger::default())
    })
    .workers(8)
    .bind("0.0.0.0:3000")?;

    info!("Starting server at http://127.0.0.1:3000");
    tokio::spawn(binance::connect_to_binance(app_state_clone));
    server.run().await
}