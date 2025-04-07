use crate::websocket::WsSession;
use redis::Client;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct AppState {
    pub clients: Arc<Mutex<Vec<Arc<WsSession>>>>,
    pub redis_client: Client,
}

impl AppState {
    pub fn new() -> Self {
        let redis_client = Client::open("redis://redis:6379").expect("Failed to connect to Redis");
        AppState {
            clients: Arc::new(Mutex::new(Vec::new())),
            redis_client,
        }
    }
}