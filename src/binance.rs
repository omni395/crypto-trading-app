use serde_json::json;
use futures_util::StreamExt;
use actix_web::{web, HttpResponse};
use reqwest;
use serde::Deserialize;
use crate::app_state::AppState;
#[allow(unused_imports)] // Подавляем предупреждение
use crate::websocket::ClientMessage;
use crate::models::{KlineEvent, DepthEvent};
use redis::Commands;

// Остальной код остаётся без изменений
#[derive(Deserialize, Debug)]
pub struct HistoricalRequest {
    pub symbol: String,
    pub interval: String,
    pub start_time: i64,
    pub end_time: i64,
}

pub async fn connect_to_binance(state: web::Data<AppState>) {
    tokio::spawn(start_binance_ws(state.clone()));
    tokio::spawn(start_binance_depth_ws(state.clone()));
}

pub async fn get_historical_data(
    query: web::Query<HistoricalRequest>,
    state: web::Data<AppState>,
) -> HttpResponse {
    let request = HistoricalRequest {
        symbol: query.symbol.clone(),
        interval: query.interval.clone(),
        start_time: query.start_time,
        end_time: query.end_time,
    };

    match fetch_historical_data(state, request).await {
        Ok(historical_data) => HttpResponse::Ok().json(json!({
            "type": "historical",
            "data": historical_data
        })),
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {:?}", e)),
    }
}

pub async fn fetch_historical_data(
    state: web::Data<AppState>,
    request: HistoricalRequest,
) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
    let mut conn = state.redis_client
        .get_connection()
        .expect("Failed to connect to Redis");

    let key = format!("historical:{}:{}", request.start_time, request.end_time);
    let cached_data: Option<String> = conn.get(&key).ok();

    let historical_data = if let Some(data) = cached_data {
        println!("Данные взяты из кэша Redis");
        serde_json::from_str(&data).unwrap_or_default()
    } else {
        let client = reqwest::Client::new();
        let symbol = request.symbol.to_uppercase();
        let url = format!(
            "https://api.binance.com/api/v3/klines?symbol={}&interval={}&startTime={}&endTime={}&limit=1000",
            symbol, request.interval, request.start_time, request.end_time
        );
        println!("Отправлен запрос на исторические данные: {}", url);

        let response = client.get(&url).send().await?;
        let data: Vec<Vec<serde_json::Value>> = response.json().await?;
        let historical_data: Vec<serde_json::Value> = data.into_iter().map(|kline| {
            let time = kline[0].as_i64().unwrap() / 1000; // Убедимся, что время в секундах
            let open = kline[1].as_str().unwrap().parse::<f64>().unwrap_or(0.0);
            let high = kline[2].as_str().unwrap().parse::<f64>().unwrap_or(0.0);
            let low = kline[3].as_str().unwrap().parse::<f64>().unwrap_or(0.0);
            let close = kline[4].as_str().unwrap().parse::<f64>().unwrap_or(0.0);
            let volume = kline[5].as_str().unwrap().parse::<f64>().unwrap_or(0.0);

            json!({
                "event_type": "historical_kline",
                "time": time,
                "open": open,
                "high": high,
                "low": low,
                "close": close,
                "volume": volume,
            })
        }).collect();

        let serialized_data = serde_json::to_string(&historical_data).unwrap();
        conn.set_ex::<_, _, ()>(&key, serialized_data, 3600).ok();

        historical_data
    };

    Ok(historical_data)
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

                                let message = kline_json.to_string();
                                let clients = state.clients.lock().await;
                                for client in clients.iter() {
                                    client.send_message(message.clone());
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

                                let message = depth_json.to_string();
                                let clients = state.clients.lock().await;
                                for client in clients.iter() {
                                    client.send_message(message.clone());
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