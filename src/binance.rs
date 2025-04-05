use serde_json::json;
use futures_util::StreamExt;
use actix_web::web;
use reqwest; // Добавляем для HTTP-запросов

use crate::websocket::{AppState};
use crate::models::{KlineEvent, DepthEvent};

pub async fn connect_to_binance(state: web::Data<AppState>) {
    tokio::spawn(fetch_historical_data(state.clone())); // Загружаем исторические данные
    tokio::spawn(start_binance_ws(state.clone()));
    tokio::spawn(start_binance_depth_ws(state.clone()));
}

async fn fetch_historical_data(state: web::Data<AppState>) {
    let client = reqwest::Client::new();
    let symbol = "BTCUSDT";
    let interval = "1m";
    let mut start_time = 1498867200000; // 1 июля 2017 года в миллисекундах
    let end_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as i64;

    while start_time < end_time {
        let url = format!(
            "https://api.binance.com/api/v3/klines?symbol={}&interval={}&startTime={}&endTime={}&limit=1000",
            symbol, interval, start_time, end_time
        );

        match client.get(&url).send().await {
            Ok(response) => {
                if let Ok(data) = response.json::<Vec<Vec<serde_json::Value>>>().await {
                    if data.is_empty() {
                        break; // Больше данных нет
                    }

                    let mut last_kline_time = start_time; // Сохраняем время последней свечи
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

                        last_kline_time = kline[0].as_i64().unwrap(); // Обновляем время последней свечи
                    }

                    // Обновляем start_time на время последней свечи + 1 минута
                    start_time = (last_kline_time + 60 * 1000) as i64;
                } else {
                    println!("Ошибка десериализации исторических данных");
                    break;
                }
            }
            Err(e) => {
                println!("Ошибка загрузки исторических данных: {:?}", e);
                break;
            }
        }

        // Задержка, чтобы не превысить лимиты API
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
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

#[derive(actix::Message)]
#[rtype(result = "()")]
pub struct ClientMessage(pub String);