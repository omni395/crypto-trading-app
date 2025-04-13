use actix::{ActorContext, AsyncContext};
use actix_web::{web, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

use crate::app_state::AppState;
use crate::binance;
use crate::database;
use crate::models::{Drawing, WebSocketMessage};

pub async fn ws_index(
    req: HttpRequest,
    stream: web::Payload,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, actix_web::Error> {
    ws::start(WsSession::new(app_state), &req, stream)
}

pub struct WsSession {
    pub app_state: web::Data<AppState>,
    pub id: usize,
    addr: Option<actix::Addr<Self>>,
}

impl WsSession {
    pub fn new(app_state: web::Data<AppState>) -> Self {
        WsSession {
            app_state,
            id: rand::random(),
            addr: None,
        }
    }

    pub fn send_message(&self, message: String) {
        if let Some(addr) = &self.addr {
            addr.do_send(ClientMessage(message));
        }
    }
}

#[derive(Debug)]
pub struct ClientMessage(pub String);

impl actix::Message for ClientMessage {
    type Result = ();
}

impl actix::Actor for WsSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.addr = Some(ctx.address());
        let session = Arc::new(WsSession {
            app_state: self.app_state.clone(),
            id: self.id,
            addr: Some(ctx.address()),
        });
        let app_state = self.app_state.clone();
        ctx.spawn(actix::fut::wrap_future(async move {
            let mut clients = app_state.clients.lock().await;
            clients.push(session.clone());
            println!("Клиент добавлен, ID: {}, общее количество клиентов: {}", session.id, clients.len());
        }));
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        let app_state = self.app_state.clone();
        let id = self.id;
        ctx.spawn(actix::fut::wrap_future(async move {
            let mut clients = app_state.clients.lock().await;
            clients.retain(|client| client.id != id);
            println!("Клиент удалён, ID: {}, общее количество клиентов: {}", id, clients.len());
        }));
    }
}

impl actix::Handler<ClientMessage> for WsSession {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, ctx: &mut Self::Context) -> Self::Result {
        ctx.text(msg.0);
    }
}

impl actix::StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                if let Ok(value) = serde_json::from_str::<serde_json::Value>(&text) {
                    let event_type = value["event_type"].as_str().unwrap_or("");
                    match event_type {
                        "save_drawing" => {
                            println!("Получено сообщение save_drawing: {}", text);
                            if let Ok(mut drawing) = serde_json::from_value::<Drawing>(value["data"].clone()) {
                                // Генерируем новый UUID на сервере
                                drawing.id = Uuid::new_v4().to_string();
                                let app_state = self.app_state.clone();
                                let addr = ctx.address();
                                let drawing_clone = drawing.clone();
                                ctx.spawn(actix::fut::wrap_future(async move {
                                    let mut redis_con = app_state.redis_pool.clone();
                                    match database::save_drawing(redis_con, &drawing_clone).await {
                                        Ok(_) => {
                                            println!("Drawing успешно сохранён: {:?}", drawing_clone);
                                            addr.do_send(ClientMessage(json!({"event_type": "drawing_saved", "status": "success", "id": drawing_clone.id}).to_string()));
                                            // Рассылаем сообщение всем клиентам
                                            let message = json!({
                                                "event_type": "drawing_added",
                                                "data": drawing_clone
                                            }).to_string();
                                            app_state.broadcast(&message).await;
                                        }
                                        Err(e) => {
                                            println!("Ошибка сохранения drawing: {:?}", e);
                                            addr.do_send(ClientMessage(json!({"event_type": "drawing_saved", "status": "error", "message": e.to_string()}).to_string()));
                                        }
                                    }
                                }));
                            } else {
                                println!("Ошибка десериализации drawing: {:?}", value["data"]);
                                let addr = ctx.address();
                                addr.do_send(ClientMessage(json!({"event_type": "drawing_saved", "status": "error", "message": "Invalid drawing data"}).to_string()));
                            }
                        }
                        "load_drawings" => {
                            if let Some(data) = value.get("data") {
                                let symbol = data.get("symbol").and_then(|v| v.as_str()).unwrap_or("");
                                let app_state = self.app_state.clone();
                                let addr = ctx.address();
                                ctx.spawn(actix::fut::wrap_future(async move {
                                    let mut redis_con = app_state.redis_pool.clone();
                                    match database::load_drawings(redis_con, symbol).await {
                                        Ok(drawings) => {
                                            addr.do_send(ClientMessage(json!({"event_type": "drawings_loaded", "data": drawings}).to_string()));
                                        }
                                        Err(e) => {
                                            println!("Ошибка загрузки drawings: {:?}", e);
                                            addr.do_send(ClientMessage(json!({"event_type": "drawings_loaded", "status": "error", "message": e.to_string()}).to_string()));
                                        }
                                    }
                                }));
                            }
                        }
                        "delete_drawing" => {
                            if let Some(data) = value.get("data") {
                                let id = data.get("id").and_then(|v| v.as_str()).unwrap_or("");
                                let symbol = data.get("symbol").and_then(|v| v.as_str()).unwrap_or("");
                                let drawing_type = data.get("drawing_type").and_then(|v| v.as_str()).unwrap_or("");
                                if Uuid::parse_str(id).is_err() {
                                    let addr = ctx.address();
                                    addr.do_send(ClientMessage(json!({"event_type": "drawing_deleted", "status": "error", "message": "Invalid UUID"}).to_string()));
                                    return;
                                }
                                let app_state = self.app_state.clone();
                                let addr = ctx.address();
                                ctx.spawn(actix::fut::wrap_future(async move {
                                    let mut redis_con = app_state.redis_pool.clone();
                                    match database::delete_drawing(redis_con, drawing_type, symbol, id).await {
                                        Ok(_) => {
                                            addr.do_send(ClientMessage(json!({"event_type": "drawing_deleted", "status": "success"}).to_string()));
                                            // Рассылаем сообщение всем клиентам
                                            let message = json!({
                                                "event_type": "drawing_deleted",
                                                "data": {
                                                    "id": id,
                                                    "drawing_type": drawing_type,
                                                    "symbol": symbol
                                                }
                                            }).to_string();
                                            app_state.broadcast(&message).await;
                                        }
                                        Err(e) => {
                                            println!("Ошибка удаления drawing: {:?}", e);
                                            addr.do_send(ClientMessage(json!({"event_type": "drawing_deleted", "status": "error", "message": e.to_string()}).to_string()));
                                        }
                                    }
                                }));
                            }
                        }
                        "load_historical_data" => {
                            if let Some(data) = value.get("data") {
                                let symbol = data.get("symbol").and_then(|v| v.as_str()).unwrap_or("BTCUSDT");
                                let interval = data.get("interval").and_then(|v| v.as_str()).unwrap_or("1m");
                                let start_time = data.get("start_time").and_then(|v| v.as_i64()).unwrap_or(0);
                                let end_time = data.get("end_time").and_then(|v| v.as_i64()).unwrap_or(0);
                                let is_initial_load = data.get("is_initial_load").and_then(|v| v.as_bool()).unwrap_or(false);
                                let request = binance::HistoricalRequest {
                                    symbol: symbol.to_string(),
                                    interval: interval.to_string(),
                                    start_time,
                                    end_time,
                                };
                                let app_state = self.app_state.clone();
                                let addr = ctx.address();
                                ctx.spawn(actix::fut::wrap_future(async move {
                                    match binance::fetch_historical_data(app_state, request).await {
                                        Ok(historical_data) => {
                                            addr.do_send(ClientMessage(json!({
                                                "event_type": "historical_data",
                                                "data": historical_data,
                                                "is_initial_load": is_initial_load
                                            }).to_string()));
                                        }
                                        Err(e) => {
                                            println!("Ошибка загрузки исторических данных: {:?}", e);
                                            addr.do_send(ClientMessage(json!({
                                                "event_type": "historical_data",
                                                "status": "error",
                                                "message": e.to_string()
                                            }).to_string()));
                                        }
                                    }
                                }));
                            }
                        }
                        "kline" => {
                            let ws_message: WebSocketMessage = match serde_json::from_str(&text) {
                                Ok(msg) => msg,
                                Err(_) => return,
                            };
                            let app_state = self.app_state.clone();
                            ctx.spawn(actix::fut::wrap_future(async move {
                                let clients = app_state.clients.lock().await;
                                for client in clients.iter() {
                                    client.send_message(text.to_string());
                                }
                            }));
                        }
                        _ => println!("Неизвестный тип события: {}", event_type),
                    }
                }
            }
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Close(_)) => ctx.stop(),
            _ => (),
        }
    }
}