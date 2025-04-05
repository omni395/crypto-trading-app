use serde_json::json;
use futures_util::StreamExt;
use actix_web::web;

use crate::websocket::{AppState};
use crate::models::{KlineEvent, DepthEvent};

pub async fn connect_to_binance(state: web::Data<AppState>) {
    tokio::spawn(start_binance_ws(state.clone()));
    tokio::spawn(start_binance_depth_ws(state.clone()));
}

pub async fn start_binance_ws(state: web::Data<AppState>) {
    let ws_url = "wss://stream.binance.com:9443/ws/btcusdt@kline_1m";
    loop {
        match tokio_tungstenite::connect_async(ws_url).await {
            Ok((mut ws, _)) => {
                println!("Подключено к Binance WebSocket: {}", ws_url);
                while let Some(msg) = ws.next().await {
                    match msg {
                        Ok(tokio_tungstenite::tungstenite::Message::Text(text)) => {
                            if let Ok(data) = serde_json::from_str::<KlineEvent>(&text) {
                                let kline_json = json!({
                                    "event_type": "kline",
                                    "event_time": data.event_time,
                                    "symbol": data.symbol,
                                    "kline": {
                                        "start_time": data.k.start_time,
                                        "close_time": data.k.close_time,
                                        "symbol": data.k.symbol,
                                        "interval": data.k.interval,
                                        "open": data.k.open,
                                        "high": data.k.high,
                                        "low": data.k.low,
                                        "close": data.k.close,
                                        "volume": data.k.volume,
                                        "is_closed": data.k.is_closed,
                                    }
                                });

                                let clients = state.clients.lock().await;
                                for client in clients.iter() {
                                    client.do_send(ClientMessage(kline_json.to_string()));
                                }
                            }
                        }
                        Ok(_) => continue,
                        Err(e) => {
                            println!("Ошибка WebSocket (kline): {:?}", e);
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                println!("Не удалось подключиться к Binance WebSocket (kline): {:?}", e);
            }
        }
        println!("Переподключение к Binance WebSocket (kline) через 5 секунд...");
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }
}

pub async fn start_binance_depth_ws(state: web::Data<AppState>) {
    let ws_url = "wss://stream.binance.com:9443/ws/btcusdt@depth20@100ms";
    loop {
        match tokio_tungstenite::connect_async(ws_url).await {
            Ok((mut ws, _)) => {
                println!("Подключено к Binance Depth WebSocket: {}", ws_url);
                while let Some(msg) = ws.next().await {
                    match msg {
                        Ok(tokio_tungstenite::tungstenite::Message::Text(text)) => {
                            if let Ok(data) = serde_json::from_str::<DepthEvent>(&text) {
                                let depth_json = json!({
                                    "event_type": "depth",
                                    "event_time": data.event_time,
                                    "symbol": data.symbol,
                                    "bids": data.bids,
                                    "asks": data.asks,
                                });

                                let clients = state.clients.lock().await;
                                for client in clients.iter() {
                                    client.do_send(ClientMessage(depth_json.to_string()));
                                }
                            }
                        }
                        Ok(_) => continue,
                        Err(e) => {
                            println!("Ошибка WebSocket (depth): {:?}", e);
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                println!("Не удалось подключиться к Binance Depth WebSocket: {:?}", e);
            }
        }
        println!("Переподключение к Binance Depth WebSocket через 5 секунд...");
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }
}

#[derive(actix::Message)]
#[rtype(result = "()")]
pub struct ClientMessage(pub String);