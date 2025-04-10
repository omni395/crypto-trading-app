use std::sync::Arc;

use tokio::sync::Mutex;

use crate::database::Database;
use crate::websocket::WsSession;

pub struct AppState {
    pub db: Arc<Database>,
    pub clients: Arc<Mutex<Vec<Arc<WsSession>>>>,
}

impl AppState {
    pub fn new(redis_client: redis::Client) -> Self {
        AppState {
            db: Arc::new(Database::new(redis_client)),
            clients: Arc::new(Mutex::new(Vec::new())),
        }
    }
}