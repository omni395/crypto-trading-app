use actix::Addr;
use tokio::sync::Mutex;
use std::sync::Arc;
use crate::websocket::WebSocketActor;

#[derive(Clone)]
pub struct AppState {
    pub clients: Arc<Mutex<Vec<Addr<WebSocketActor>>>>,
}