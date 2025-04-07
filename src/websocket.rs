use actix_web::{web, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use actix::AsyncContext;
use crate::app_state::AppState;
use crate::binance::HistoricalRequest;
use std::sync::Arc;

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
                let message: serde_json::Value = serde_json::from_str(&text).unwrap_or_default();
                if message["type"] == "historical" {
                    let request = HistoricalRequest {
                        symbol: "BTCUSDT".to_string(),
                        interval: "1m".to_string(),
                        start_time: message["startTime"].as_i64().unwrap_or(0),
                        end_time: message["endTime"].as_i64().unwrap_or(0),
                    };
                    let app_state = self.app_state.clone();
                    ctx.spawn(actix::fut::wrap_future(async move {
                        // Добавим небольшую задержку, чтобы клиент успел добавиться в список
                        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                        crate::binance::fetch_historical_data(app_state, request).await;
                    }));
                }
            }
            _ => {}
        }
    }
}