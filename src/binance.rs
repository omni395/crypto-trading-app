use serde_json::json;
use actix_web::web;
use reqwest;
use serde::Deserialize;
use std::cmp::min;
use tokio_tungstenite::tungstenite;
use redis::AsyncCommands;
use futures_util::TryStreamExt;

use crate::app_state::AppState;
use crate::database;
use crate::models::{KlineEvent, DepthEvent};

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

pub async fn fetch_historical_data(
    state: web::Data<AppState>,
    request: HistoricalRequest,
) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error + Send + Sync>> {
    let key = format!("historical:{}:{}:{}", request.symbol, request.start_time, request.end_time);
    let mut redis_con = state.redis_pool.clone();
    let cached_data: Option<String> = redis_con.get(&key).await?;

    if let Some(cached_data) = cached_data {
        let data: Vec<serde_json::Value> = serde_json::from_str(&cached_data)?;
        log::info!("Данные взяты из кэша Redis для ключа {} ({} записей)", key, data.len());
        return Ok(data);
    }

    if request.start_time < 0 || request.end_time < 0 {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Start time and end time must be positive",
        )));
    }

    if request.start_time >= request.end_time {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Start time must be less than end time",
        )));
    }

    let client = reqwest::Client::new();
    let symbol = request.symbol.to_uppercase();
    let mut all_historical_data = Vec::new();
    let mut current_start = request.start_time;
    const LIMIT: i64 = 1000;
    const INTERVAL_SECONDS: i64 = 60 * 60 * 24;

    while current_start < request.end_time {
        let current_end = min(current_start + LIMIT * INTERVAL_SECONDS * 1000, request.end_time);
        let url = format!(
            "https://api.binance.com/api/v3/klines?symbol={}&interval={}&startTime={}&endTime={}&limit={}",
            symbol, request.interval, current_start, current_end, LIMIT
        );
        log::info!(
            "Отправлен запрос на Binance API: {} (интервал: {} сек)",
            url,
            (current_end - current_start) / 1000
        );

        let response = client.get(&url).send().await?;
        let data: Vec<Vec<serde_json::Value>> = response.json().await?;
        let historical_data: Vec<serde_json::Value> = data
            .into_iter()
            .map(|kline| {
                let time = kline[0].as_i64().unwrap() / 1000;
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
            })
            .collect();

        log::info!(
            "Получено {} записей исторических данных для {} (время: {} - {})",
            historical_data.len(),
            symbol,
            current_start,
            current_end
        );

        let redis_con = state.redis_pool.clone();
        for kline_data in &historical_data {
            database::save_historical_data(
                redis_con.clone(),
                &symbol,
                &request.interval,
                kline_data["time"].as_i64().unwrap(),
                &kline_data["open"].to_string(),
                &kline_data["high"].to_string(),
                &kline_data["low"].to_string(),
                &kline_data["close"].to_string(),
                &kline_data["volume"].to_string(),
            )
            .await?;
        }

        all_historical_data.extend(historical_data);
        current_start = current_end;
    }

    all_historical_data.sort_by(|a, b| {
        let time_a = a["time"].as_i64().unwrap_or(0);
        let time_b = b["time"].as_i64().unwrap_or(0);
        time_a.cmp(&time_b)
    });

    let serialized_data = serde_json::to_string(&all_historical_data)?;
    let mut redis_con = state.redis_pool.clone();
    redis_con.set_ex::<_, _, ()>(&key, serialized_data, 3600).await?;

    Ok(all_historical_data)
}

pub async fn start_binance_ws(state: web::Data<AppState>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let ws_url = "wss://stream.binance.com:9443/ws/btcusdt@kline_1m";
    loop {
        match tokio_tungstenite::connect_async(ws_url).await {
            Ok((mut ws, _)) => {
                log::info!("Подключено к Binance WebSocket: {}", ws_url);
                while let Some(msg) = ws.try_next().await? {
                    match msg {
                        tungstenite::Message::Text(text) => {
                            if let Ok(data) = serde_json::from_str::<KlineEvent>(&text) {
                                let kline = data.k;

                                let open = kline.open.parse::<f64>().unwrap_or(0.0);
                                let high = kline.high.parse::<f64>().unwrap_or(0.0);
                                let low = kline.low.parse::<f64>().unwrap_or(0.0);
                                let close = kline.close.parse::<f64>().unwrap_or(0.0);
                                let volume = kline.volume.parse::<f64>().unwrap_or(0.0);

                                let redis_con = state.redis_pool.clone();
                                if let Err(e) = database::save_historical_data(
                                    redis_con.clone(),
                                    &kline.symbol,
                                    &kline.interval,
                                    kline.start_time as i64,
                                    &open.to_string(),
                                    &high.to_string(),
                                    &low.to_string(),
                                    &close.to_string(),
                                    &volume.to_string(),
                                )
                                .await
                                {
                                    log::error!("Не удалось сохранить kline: {:?}", e);
                                    continue;
                                }

                                let kline_json = json!({
                                    "event_type": "kline",
                                    "time": kline.start_time / 1000,
                                    "open": open,
                                    "high": high,
                                    "low": low,
                                    "close": close,
                                    "volume": volume,
                                });

                                let message = kline_json.to_string();
                                state.broadcast(&message).await;
                            }
                        }
                        _ => continue,
                    }
                }
            }
            Err(e) => {
                log::error!("Не удалось подключиться к Binance WebSocket (kline): {:?}", e);
            }
        }
        log::info!("Переподключение к Binance WebSocket (kline) через 5 секунд...");
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }
}

pub async fn start_binance_depth_ws(state: web::Data<AppState>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let ws_url = "wss://stream.binance.com:9443/ws/btcusdt@depth20@100ms";
    loop {
        match tokio_tungstenite::connect_async(ws_url).await {
            Ok((mut ws, _)) => {
                log::info!("Подключено к Binance Depth WebSocket: {}", ws_url);
                while let Some(msg) = ws.try_next().await? {
                    match msg {
                        tungstenite::Message::Text(text) => {
                            if let Ok(data) = serde_json::from_str::<DepthEvent>(&text) {
                                let depth_json = json!({
                                    "event_type": "depth",
                                    "event_time": data.event_time,
                                    "symbol": data.symbol,
                                    "bids": data.bids,
                                    "asks": data.asks,
                                });

                                let message = depth_json.to_string();
                                state.broadcast(&message).await;
                            }
                        }
                        _ => continue,
                    }
                }
            }
            Err(e) => {
                log::error!("Не удалось подключиться к Binance Depth WebSocket: {:?}", e);
            }
        }
        log::info!("Переподключение к Binance Depth WebSocket через 5 секунд...");
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }
}