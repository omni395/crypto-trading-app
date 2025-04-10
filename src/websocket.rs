use actix::{ActorContext, AsyncContext};
use actix_web::{web, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use serde_json::json;
use std::sync::Arc;

use crate::app_state::AppState;
use crate::models::DrawingLine;

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
                            if let Ok(drawing) = serde_json::from_value::<DrawingLine>(value["data"].clone()) {
                                let db = self.app_state.db.clone();
                                let addr = ctx.address();
                                ctx.spawn(actix::fut::wrap_future(async move {
                                    match db.save_drawing(&drawing).await {
                                        Ok(_) => addr.do_send(ClientMessage(json!({"event_type": "drawing_saved", "status": "success"}).to_string())),
                                        Err(e) => addr.do_send(ClientMessage(json!({"event_type": "drawing_saved", "status": "error", "message": e.to_string()}).to_string())),
                                    }
                                }));
                            }
                        }
                        "load_drawings" => {
                            if let Some(symbol) = value["data"]["symbol"].as_str() {
                                let db = self.app_state.db.clone();
                                let addr = ctx.address();
                                let symbol = symbol.to_string();
                                ctx.spawn(actix::fut::wrap_future(async move {
                                    match db.load_drawings(&symbol).await {
                                        Ok(lines) => addr.do_send(ClientMessage(json!({"event_type": "drawings_loaded", "data": lines}).to_string())),
                                        Err(e) => addr.do_send(ClientMessage(json!({"event_type": "drawings_loaded", "status": "error", "message": e.to_string()}).to_string())),
                                    }
                                }));
                            }
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