use actix_web::{web, App, HttpServer};
use actix_files::Files;
use std::sync::Arc;
use tokio::sync::Mutex;

mod binance;
mod websocket;
mod routes;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init(); // Инициализация логгера

    let app_state = web::Data::new(websocket::AppState {
        clients: Arc::new(Mutex::new(Vec::new())),
    });

    tokio::spawn(binance::connect_to_binance(app_state.clone()));

    println!("Запуск сервера на http://127.0.0.1:3000");

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .service(web::resource("/ws").route(web::get().to(routes::ws_handler)))
            .service(Files::new("/", "./static").index_file("index.html"))
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}