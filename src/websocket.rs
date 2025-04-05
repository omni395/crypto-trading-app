use actix::{Actor, ActorContext, AsyncContext, StreamHandler, Handler};
use actix_web_actors::ws;
use serde::{Deserialize};
use actix::Addr;
use actix_web::web;

use crate::binance::{fetch_historical_data, HistoricalRequest};
use crate::app_state::AppState;

#[derive(Deserialize, Debug)]
struct Subscription {
    #[allow(dead_code)]
    symbol: String,
    #[allow(dead_code)]
    interval: String,
}

pub struct WebSocketActor {
    app_state: web::Data<AppState>,
    addr: Option<Addr<WebSocketActor>>,
}

impl WebSocketActor {
    pub fn new(app_state: web::Data<AppState>) -> Self {
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
                } else if let Ok(request) = serde_json::from_str::<HistoricalRequest>(&text) {
                    println!("Получен запрос на исторические данные: {:?}", request);
                    let state = self.app_state.clone();
                    tokio::spawn(fetch_historical_data(state, request));
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

#[derive(actix::Message)]
#[rtype(result = "()")]
pub struct ClientMessage(pub String);