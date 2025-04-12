use actix::{ActorContext, AsyncContext};
use actix_web::{web, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use serde_json::json;
use std::sync::Arc;

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
                            if let Ok(drawing) = serde_json::from_value::<Drawing>(value["data"].clone()) {
                                let mut redis_con = self.app_state.redis_client.get_connection().unwrap();
                                let addr = ctx.address();
                                if let Err(e) = database::save_drawing(&mut redis_con, &drawing) {
                                    addr.do_send(ClientMessage(json!({"event_type": "drawing_saved", "status": "error", "message": e.to_string()}).to_string()));
                                    return;
                                }
                                addr.do_send(ClientMessage(json!({"event_type": "drawing_saved", "status": "success"}).to_string()));
                            }
                        }
                        "load_drawings" => {
                            if let Some(data) = value.get("data") {
                                let symbol = data.get("symbol").and_then(|v| v.as_str()).unwrap_or("");
                                let drawing_type = data.get("drawing_type").and_then(|v| v.as_str()).unwrap_or("drawing.line");
                                let mut redis_con = self.app_state.redis_client.get_connection().unwrap();
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
                                let price = data.get("price").and_then(|v| v.as_f64()).unwrap_or(0.0);
                                let time = data.get("time").and_then(|v| v.as_i64()).unwrap_or(0);
                                let mut redis_con = self.app_state.redis_client.get_connection().unwrap();
                                let addr = ctx.address();
                                if let Err(e) = database::delete_drawing(&mut redis_con, drawing_type, symbol, price, time) {
                                    addr.do_send(ClientMessage(json!({"event_type": "drawing_deleted", "status": "error", "message": e.to_string()}).to_string()));
                                    return;
                                }
                                addr.do_send(ClientMessage(json!({"event_type": "drawing_deleted", "status": "success"}).to_string()));
                            }
                        }
                        "kline" => {
                            let ws_message: WebSocketMessage = match serde_json::from_str(&text) {
                                Ok(msg) => msg,
                                Err(_) => return,
                            };
                            let mut redis_con = self.app_state.redis_client.get_connection().unwrap();
                            if let Some(kline) = ws_message.kline {
                                if let Err(e) = database::save_historical_data(
                                    &mut redis_con,
                                    &kline.symbol,
                                    &kline.interval,
                                    kline.start_time as i64,
                                    &kline.open,
                                    &kline.high,
                                    &kline.low,
                                    &kline.close,
                                    &kline.volume,
                                ) {
                                    println!("Failed to save historical data: {:?}", e);
                                }
                            }
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