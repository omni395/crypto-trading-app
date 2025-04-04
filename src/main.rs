use actix_web::{web, App, HttpServer};
use log::info;

mod app_state;
mod binance;
mod models;
mod routes;
mod websocket;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    unsafe {
        std::env::set_var("RUST_LOG", "info");
    }
    env_logger::init();

    let app_state = web::Data::new(app_state::AppState {
        clients: std::sync::Arc::new(tokio::sync::Mutex::new(Vec::new())),
    });

    // Запускаем соединение с Binance
    binance::connect_to_binance(app_state.clone()).await;

    info!("Starting server at http://127.0.0.1:3000");
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(routes::websocket)
            .service(actix_files::Files::new("/", "./static").index_file("index.html"))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}