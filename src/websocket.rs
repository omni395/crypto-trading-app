use actix::{ActorContext, AsyncContext};
use actix_web::{web, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use serde_json::json;
use std::sync::Arc;
use uuid::Uuid;

use crate::app_state::AppState;
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
                            if let Some(data) = value.get("data") {
                                let drawing_type = data.get("drawing_type").and_then(|v| v.as_str()).unwrap_or("");
                                let symbol = data.get("symbol").and_then(|v| v.as_str()).unwrap_or("");
                                let drawing_data = data.get("drawing_data").ok_or_else(|| {
                                    println!("Отсутствует drawing_data");
                                    "Missing drawing_data"
                                });
                                if drawing_data.is_err() {
                                    let addr = ctx.address();
                                    addr.do_send(ClientMessage(json!({"event_type": "drawing_saved", "status": "error", "message": "Missing drawing_data"}).to_string()));
                                    return;
                                }
                                // Проверяем, что drawing_data можно сериализовать в JSON
                                let drawing_data_str = match serde_json::to_string(&drawing_data.unwrap()) {
                                    Ok(s) => s,
                                    Err(e) => {
                                        println!("Ошибка сериализации drawing_data: {:?}", e);
                                        let addr = ctx.address();
                                        addr.do_send(ClientMessage(json!({"event_type": "drawing_saved", "status": "error", "message": "Invalid drawing_data format"}).to_string()));
                                        return;
                                    }
                                };
                                let mut drawing = Drawing {
                                    id: Uuid::new_v4().to_string(),
                                    drawing_type: drawing_type.to_string(),
                                    symbol: symbol.to_string(),
                                    data: drawing_data_str,
                                };
                                let mut redis_con = match self.app_state.redis_client.get_connection() {
                                    Ok(con) => con,
                                    Err(e) => {
                                        println!("Ошибка подключения к Redis: {:?}", e);
                                        let addr = ctx.address();
                                        addr.do_send(ClientMessage(json!({"event_type": "drawing_saved", "status": "error", "message": e.to_string()}).to_string()));
                                        return;
                                    }
                                };
                                let addr = ctx.address();
                                if let Err(e) = database::save_drawing(&mut redis_con, &drawing) {
                                    println!("Ошибка сохранения drawing: {:?}", e);
                                    addr.do_send(ClientMessage(json!({"event_type": "drawing_saved", "status": "error", "message": e.to_string()}).to_string()));
                                    return;
                                }
                                println!("Drawing успешно сохранён: {:?}", drawing);
                                addr.do_send(ClientMessage(json!({"event_type": "drawing_saved", "status": "success", "id": drawing.id}).to_string()));
                                // Рассылаем сообщение всем клиентам
                                let app_state = self.app_state.clone();
                                let message = json!({
                                    "event_type": "drawing_added",
                                    "data": drawing
                                }).to_string();
                                ctx.spawn(actix::fut::wrap_future(async move {
                                    app_state.broadcast(&message).await;
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
                                let drawing_type = data.get("drawing_type").and_then(|v| v.as_str()).unwrap_or("drawing.line");
                                let mut redis_con = match self.app_state.redis_client.get_connection() {
                                    Ok(con) => con,
                                    Err(e) => {
                                        println!("Ошибка подключения к Redis: {:?}", e);
                                        let addr = ctx.address();
                                        addr.do_send(ClientMessage(json!({"event_type": "drawings_loaded", "status": "error", "message": e.to_string()}).to_string()));
                                        return;
                                    }
                                };
                                let addr = ctx.address();
                                match database::load_drawings(&mut redis_con, drawing_type, symbol) {
                                    Ok(lines) => addr.do_send(ClientMessage(json!({"event_type": "drawings_loaded", "data": lines}).to_string())),
                                    Err(e) => addr.do_send(ClientMessage(json!({"event_type": "drawings_loaded", "status": "error", "message": e.to_string()}).to_string())),
                                }
                            }
                        }
                        "delete_drawing" => {
                            if let Some(data) = value.get("data") {
                                let symbol = data.get("symbol").and_then(|v| v.as_str()).unwrap_or("");
                                let drawing_type = data.get("drawing_type").and_then(|v| v.as_str()).unwrap_or("drawing.line");
                                let id = data.get("id").and_then(|v| v.as_str()).unwrap_or("");
                                let mut redis_con = match self.app_state.redis_client.get_connection() {
                                    Ok(con) => con,
                                    Err(e) => {
                                        println!("Ошибка подключения к Redis: {:?}", e);
                                        let addr = ctx.address();
                                        addr.do_send(ClientMessage(json!({"event_type": "drawing_deleted", "status": "error", "message": e.to_string()}).to_string()));
                                        return;
                                    }
                                };
                                let addr = ctx.address();
                                if let Err(e) = database::delete_drawing(&mut redis_con, drawing_type, symbol, id) {
                                    addr.do_send(ClientMessage(json!({"event_type": "drawing_deleted", "status": "error", "message": e.to_string()}).to_string()));
                                    return;
                                }
                                addr.do_send(ClientMessage(json!({"event_type": "drawing_deleted", "status": "success"}).to_string()));
                                // Рассылаем сообщение об удалении всем клиентам
                                let app_state = self.app_state.clone();
                                let message = json!({
                                    "event_type": "drawing_deleted",
                                    "data": {
                                        "drawing_type": drawing_type,
                                        "symbol": symbol,
                                        "id": id
                                    }
                                }).to_string();
                                ctx.spawn(actix::fut::wrap_future(async move {
                                    app_state.broadcast(&message).await;
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