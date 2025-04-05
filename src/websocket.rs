use actix::{Actor, ActorContext, AsyncContext, StreamHandler, Handler};
use actix_web_actors::ws;
use serde::{Deserialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use actix::Addr;

use crate::binance::{ClientMessage};

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Subscription {
    symbol: String,
    interval: String,
}

pub struct AppState {
    pub clients: Arc<Mutex<Vec<Addr<WebSocketActor>>>>,
}

pub struct WebSocketActor {
    app_state: Arc<AppState>,
    addr: Option<Addr<WebSocketActor>>,
}

impl WebSocketActor {
    pub fn new(app_state: Arc<AppState>) -> Self {
        let actor = WebSocketActor {
            app_state,
            addr: None,
        };
        actor
    }
}

impl Actor for WebSocketActor {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("WebSocketActor запущен");
        // Сохраняем адрес актора
        self.addr = Some(ctx.address());
        // Добавляем клиента в список
        let addr = ctx.address();
        let clients = self.app_state.clients.clone();
        tokio::spawn(async move {
            clients.lock().await.push(addr);
        });
    }

    fn stopped(&mut self, _: &mut Self::Context) {
        // Удаляем клиента из списка при остановке
        if let Some(addr) = self.addr.take() {
            let clients = self.app_state.clients.clone();
            tokio::spawn(async move {
                let mut clients = clients.lock().await;
                clients.retain(|client| client != &addr);
            });
        }
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketActor {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                // Парсим сообщение от клиента
                if let Ok(subscription) = serde_json::from_str::<Subscription>(&text) {
                    println!("Получена подписка: {:?}", subscription);
                }
            }
            Ok(ws::Message::Close(_)) => {
                println!("WebSocket закрыт клиентом");
                ctx.stop();
            }
            _ => (),
        }
    }
}

impl Handler<ClientMessage> for WebSocketActor {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}