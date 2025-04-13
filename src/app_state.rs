use std::sync::Arc;
use tokio::sync::Mutex;
use crate::websocket::WsSession;

pub struct AppState {
    pub clients: Arc<Mutex<Vec<Arc<WsSession>>>>,
    pub redis_pool: redis::aio::MultiplexedConnection,
}

impl AppState {
    pub fn new(redis_pool: redis::aio::MultiplexedConnection) -> Self {
        AppState {
            clients: Arc::new(Mutex::new(Vec::new())),
            redis_pool,
        }
    }

    pub async fn broadcast(&self, message: &str) {
        let clients = self.clients.lock().await;
        for client in clients.iter() {
            client.send_message(message.to_string());
        }
    }
}