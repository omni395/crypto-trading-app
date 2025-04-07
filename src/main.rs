use actix_web::{web, App, HttpServer};
use actix_cors::Cors;

// Объявляем модули
mod app_state;
mod binance;
mod routes;
mod websocket; // Добавляем модуль websocket
mod models; // Добавляем модуль models

// Импортируем нужные элементы из модулей
use app_state::AppState;
use binance::connect_to_binance;
use routes::configure_websocket; // Обновляем имя функции

#[actix_web::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    let redis_client = redis::Client::open("redis://redis:6379/")?;
    let app_state = web::Data::new(AppState::new(redis_client));

    let app_state_clone = app_state.clone();
    tokio::spawn(async move {
        connect_to_binance(app_state_clone).await;
    });

    log::info!("Starting server at http://127.0.0.1:3000");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://127.0.0.1:8080")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![actix_web::http::header::CONTENT_TYPE])
            .max_age(3600);

        App::new()
            .app_data(app_state.clone())
            .wrap(cors)
            .configure(configure_websocket) // Используем переименованную функцию
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await?;

    Ok(())
}