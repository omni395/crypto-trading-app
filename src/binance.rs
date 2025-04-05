use serde_json::json;
use futures_util::StreamExt;
use actix_web::web;
use reqwest;
use serde::Deserialize;

use crate::app_state::AppState;
use crate::websocket::ClientMessage;
use crate::models::{KlineEvent, DepthEvent};

#[derive(Deserialize, Debug)] // Добавляем Debug
pub struct HistoricalRequest {
    pub symbol: String,
    pub interval: String,
    pub start_time: i64, // В миллисекундах
    pub end_time: i64,   // В миллисекундах
}

pub async fn connect_to_binance(state: web::Data<AppState>) {
    tokio::spawn(start_binance_ws(state.clone()));
    tokio::spawn(start_binance_depth_ws(state.clone()));
}

pub async fn fetch_historical_data(state: web::Data<AppState>, request: HistoricalRequest) {
    let client = reqwest::Client::new();
    let url = format!(
        "https://api.binance.com/api/v3/klines?symbol={}&interval={}&startTime={}&endTime={}&limit=1000",
        request.symbol, request.interval, request.start_time, request.end_time
    );

    match client.get(&url).send().await {
        Ok(response) => {
            if let Ok(data) = response.json::<Vec<Vec<serde_json::Value>>>().await {
                for kline in data {
                    let candlestick_data = json!({
                        "event_type": "historical_kline",
                        "time": kline[0].as_i64().unwrap() / 1000, // Конвертируем в секунды
                        "open": kline[1].as_str().unwrap().parse::<f64>().unwrap_or(0.0),
                        "high": kline[2].as_str().unwrap().parse::<f64>().unwrap_or(0.0),
                        "low": kline[3].as_str().unwrap().parse::<f64>().unwrap_or(0.0),
                        "close": kline[4].as_str().unwrap().parse::<f64>().unwrap_or(0.0),
                        "volume": kline[5].as_str().unwrap().parse::<f64>().unwrap_or(0.0),
                    });

                    let clients = state.clients.lock().await;
                    for client in clients.iter() {
                        client.do_send(ClientMessage(candlestick_data.to_string()));
                    }
                }
            } else {
                println!("Ошибка десериализации исторических данных");
            }
        }
        Err(e) => {
            println!("Ошибка загрузки исторических данных: {:?}", e);
        }
    }
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