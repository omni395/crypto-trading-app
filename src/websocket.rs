use actix::AsyncContext;
use actix_web::{web, HttpResponse};
use actix_web_actors::ws;
use std::sync::Arc;
use tokio::sync::Mutex;
use actix::{Addr, Actor, StreamHandler, Handler, Message};

pub struct AppState {
    pub clients: Arc<Mutex<Vec<Addr<EchoWs>>>>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientMessage(pub String);

pub struct EchoWs {
    pub state: web::Data<AppState>,
    pub addr: Option<Addr<EchoWs>>,
}

impl Actor for EchoWs {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.addr = Some(ctx.address());
        let addr = ctx.address();
        let state = self.state.clone();
        actix::spawn(async move {
            let mut clients = state.clients.lock().await;
            clients.push(addr);
        });
    }

    fn stopped(&mut self, _: &mut Self::Context) {
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

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for EchoWs {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => ctx.text(text),
            _ => (),
        }
    }
}

impl Handler<ClientMessage> for EchoWs {
    type Result = ();

    fn handle(&mut self, msg: ClientMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

pub async fn ws_handler(req: actix_web::HttpRequest, stream: web::Payload, state: web::Data<AppState>) -> actix_web::Result<HttpResponse> {
    ws::start(EchoWs { state: state.clone(), addr: None }, &req, stream)
}