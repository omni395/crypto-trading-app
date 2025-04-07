use redis::Client;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::websocket::WsSession;

pub struct AppState {
    pub redis_client: Client,
    pub clients: Arc<Mutex<Vec<Arc<WsSession>>>>,
}

impl AppState {
    pub fn new(redis_client: Client) -> Self {
        AppState {
            redis_client,
            clients: Arc::new(Mutex::new(Vec::new())),
        }
    }
}