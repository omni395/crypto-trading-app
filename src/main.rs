use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use actix_web::get;
use actix_web_actors::ws;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::{stream::StreamExt, sink::SinkExt};
use serde::{Deserialize, Serialize};
use serde_json;
use std::sync::Arc;
use tokio::sync::Mutex;
use actix::{Addr, AsyncContext, Actor, StreamHandler, Handler, Message as ActixMessage};

// Структура для свечных данных от Binance
#[derive(Debug, Deserialize, Serialize)]
struct KlineEvent {
    #[serde(rename = "e")]
    event_type: String,
    #[serde(rename = "E")]
    event_time: i64,
    #[serde(rename = "s")]
    symbol: String,
    #[serde(rename = "k")]
    kline: KlineData,
}

#[derive(Debug, Deserialize, Serialize)]
struct KlineData {
    #[serde(rename = "t")]
    start_time: i64,
    #[serde(rename = "T")]
    close_time: i64,
    #[serde(rename = "s")]
    symbol: String,
    #[serde(rename = "i")]
    interval: String,
    #[serde(rename = "o")]
    open: String,
    #[serde(rename = "h")]
    high: String,
    #[serde(rename = "l")]
    low: String,
    #[serde(rename = "c")]
    close: String,
    #[serde(rename = "v")]
    volume: String,
    #[serde(rename = "x")]
    is_closed: bool,
}

// Структура для данных стакана от Binance
#[derive(Debug, Deserialize, Serialize)]
struct DepthEvent {
    #[serde(rename = "e")]
    event_type: String,
    #[serde(rename = "E")]
    event_time: i64,
    #[serde(rename = "s")]
    symbol: String,
    #[serde(rename = "b")]
    bids: Vec<(String, String)>, // (цена, объем)
    #[serde(rename = "a")]
    asks: Vec<(String, String)>, // (цена, объем)
}

// Собственное сообщение для отправки данных клиентам
#[derive(ActixMessage)]
#[rtype(result = "()")]
struct ClientMessage(String);

// Структура для управления WebSocket-клиентами
struct AppState {
    clients: Arc<Mutex<Vec<Addr<EchoWs>>>>,
}

// Обработчик для корневого маршрута
#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Привет от сервера на Rust!")
}

// Обработчик WebSocket
async fn ws_handler(req: actix_web::HttpRequest, stream: web::Payload, state: web::Data<AppState>) -> actix_web::Result<HttpResponse> {
    ws::start(EchoWs { state: state.clone(), addr: None }, &req, stream)
}

// Актор для WebSocket
struct EchoWs {
    state: web::Data<AppState>,
    addr: Option<Addr<EchoWs>>,
}

impl Actor for EchoWs {
    type Context = ws::WebsocketContext<Self>;

    // Вызывается при старте актора
    fn started(&mut self, ctx: &mut Self::Context) {
        // Сохраняем адрес актора
        self.addr = Some(ctx.address());
        // Добавляем адрес в список клиентов
        let addr = ctx.address();
        let state = self.state.clone();
        actix::spawn(async move {
            let mut clients = state.clients.lock().await;
            clients.push(addr);
        });
    }

    // Вызывается при остановке актора
    fn stopped(&mut self, _: &mut Self::Context) {
        // Удаляем адрес из списка клиентов
        if let Some(addr) = &self.addr {
            let state = self.state.clone();
            let addr = addr.clone();
            actix::spawn(async move {
                let mut clients = state.clients.lock().await;
                clients.retain(|client| !std::ptr::eq(client, &addr));
            });
        }
    }
}

// Обработка входящих сообщений WebSocket
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for EchoWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => ctx.text(text), // Возвращаем сообщение обратно
            _ => (), // Игнорируем другие типы сообщений
        }
    }
}

// Реализация Handler для нашего сообщения
impl Handler<ClientMessage> for EchoWs {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0); // Отправляем сообщение клиенту
    }
}

// Функция для подключения к Binance WebSocket
async fn connect_to_binance(state: web::Data<AppState>) {
    let url = "wss://stream.binance.com:9443/ws/btcusdt@kline_1m/btcusdt@depth20";
    match connect_async(url).await {
        Ok((mut ws_stream, _)) => {
            println!("Подключено к Binance WebSocket");

            while let Some(msg) = ws_stream.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        // Проверяем, что это свечные данные
                        if text.contains("\"e\":\"kline\"") {
                            match serde_json::from_str::<KlineEvent>(&text) {
                                Ok(kline_event) => {
                                    println!("Получены свечные данные: {:?}", kline_event);
                                    // Отправляем данные всем клиентам
                                    let mut clients = state.clients.lock().await;
                                    let kline_json = serde_json::to_string(&kline_event).unwrap();
                                    for client in clients.iter_mut() {
                                        client.do_send(ClientMessage(kline_json.clone()));
                                    }
                                }
                                Err(e) => println!("Ошибка парсинга свечных данных: {:?}", e),
                            }
                        }
                        // Проверяем, что это данные стакана
                        else if text.contains("\"e\":\"depthUpdate\"") {
                            match serde_json::from_str::<DepthEvent>(&text) {
                                Ok(depth_event) => {
                                    println!("Получены данные стакана: {:?}", depth_event);
                                    // Отправляем данные всем клиентам
                                    let mut clients = state.clients.lock().await;
                                    let depth_json = serde_json::to_string(&depth_event).unwrap();
                                    for client in clients.iter_mut() {
                                        client.do_send(ClientMessage(depth_json.clone()));
                                    }
                                }
                                Err(e) => println!("Ошибка парсинга данных стакана: {:?}", e),
                            }
                        }
                    }
                    Ok(Message::Ping(ping)) => {
                        // Отвечаем на пинг
                        let _ = ws_stream.send(Message::Pong(ping)).await;
                    }
                    Err(e) => println!("Ошибка WebSocket: {:?}", e),
                    _ => (),
                }
            }
        }
        Err(e) => println!("Ошибка подключения к Binance: {:?}", e),
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init(); // Инициализация логгера

    // Создаем состояние приложения
    let app_state = web::Data::new(AppState {
        clients: Arc::new(Mutex::new(Vec::new())),
    });

    // Запускаем задачу для подключения к Binance
    tokio::spawn(connect_to_binance(app_state.clone()));

    println!("Запуск сервера на http://127.0.0.1:3000");

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone()) // Передаем состояние в приложение
            .service(index)
            .route("/ws", web::get().to(ws_handler)) // Маршрут для WebSocket
    })
    .bind("127.0.0.1:3000")?
    .run()
    .await
}