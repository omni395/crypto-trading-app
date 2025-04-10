use serde_json::json;
use futures_util::StreamExt;
use actix_web::{web, HttpResponse};
use reqwest;
use serde::Deserialize;
use std::cmp::min; // Добавляем импорт для функции min

use crate::app_state::AppState;
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

    match fetch_historical_data(&state, request).await {
        Ok(historical_data) => {
            log::info!(
                "Успешно возвращены исторические данные для {}: {} записей",
                query.symbol,
                historical_data.len()
            );
            HttpResponse::Ok().json(json!({
                "type": "historical",
                "data": historical_data
            }))
        }
        Err(e) => {
            log::error!("Ошибка при загрузке исторических данных: {:?}", e);
            HttpResponse::InternalServerError().body(format!("Error: {:?}", e))
        }
    }
}

pub async fn fetch_historical_data(
    state: &web::Data<AppState>,
    request: HistoricalRequest,
) -> Result<Vec<serde_json::Value>, Box<dyn std::error::Error>> {
    let key = format!("historical:{}:{}", request.start_time, request.end_time);
    if let Ok(Some(cached_data)) = state.db.get_cached_historical_data(&key).await {
        log::info!("Данные взяты из кэша Redis для ключа {} ({} записей)", key, cached_data.len());
        return Ok(cached_data);
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
    const LIMIT: i64 = 1000; // Ограничение Binance API
    const INTERVAL_SECONDS: i64 = 60; // 1 минута в миллисекундах

    // Разбиваем интервал на куски по 1000 свечей
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

        all_historical_data.extend(historical_data);
        current_start = current_end;
    }

    // Сортируем данные по времени (на всякий случай)
    all_historical_data.sort_by(|a, b| {
        let time_a = a["time"].as_i64().unwrap_or(0);
        let time_b = b["time"].as_i64().unwrap_or(0);
        time_a.cmp(&time_b)
    });

    state
        .db
        .cache_historical_data(&key, &all_historical_data)
        .await
        .unwrap_or_else(|e| log::error!("Не удалось сохранить данные в кэш: {:?}", e));

    Ok(all_historical_data)
}

pub async fn start_binance_ws(state: web::Data<AppState>) {
    let ws_url = "wss://stream.binance.com:9443/ws/btcusdt@kline_1m";
    loop {
        match tokio_tungstenite::connect_async(ws_url).await {
            Ok((mut ws, _)) => {
                log::info!("Подключено к Binance WebSocket: {}", ws_url);
                while let Some(msg) = ws.next().await {
                    match msg {
                        Ok(tokio_tungstenite::tungstenite::Message::Text(text)) => {
                            if let Ok(data) = serde_json::from_str::<KlineEvent>(&text) {
                                let kline = crate::models::Kline {
                                    start_time: data.k.start_time,
                                    close_time: data.k.close_time,
                                    symbol: data.k.symbol.clone(),
                                    interval: data.k.interval.clone(),
                                    open: data.k.open,
                                    high: data.k.high,
                                    low: data.k.low,
                                    close: data.k.close,
                                    volume: data.k.volume,
                                    is_closed: data.k.is_closed,
                                };

                                state
                                    .db
                                    .store_kline(&data.symbol, &data.k.interval, &kline)
                                    .await
                                    .unwrap_or_else(|e| log::error!("Не удалось сохранить kline: {:?}", e));

                                let kline_json = json!({
                                    "event_type": "kline",
                                    "event_time": data.event_time,
                                    "symbol": data.symbol,
                                    "kline": {
                                        "start_time": kline.start_time,
                                        "close_time": kline.close_time,
                                        "symbol": kline.symbol,
                                        "interval": kline.interval,
                                        "open": kline.open,
                                        "high": kline.high,
                                        "low": kline.low,
                                        "close": kline.close,
                                        "volume": kline.volume,
                                        "is_closed": kline.is_closed,
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
                            log::error!("Ошибка WebSocket (kline): {:?}", e);
                            break;
                        }
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

pub async fn start_binance_depth_ws(state: web::Data<AppState>) {
    let ws_url = "wss://stream.binance.com:9443/ws/btcusdt@depth20@100ms";
    loop {
        match tokio_tungstenite::connect_async(ws_url).await {
            Ok((mut ws, _)) => {
                log::info!("Подключено к Binance Depth WebSocket: {}", ws_url);
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
                            log::error!("Ошибка WebSocket (depth): {:?}", e);
                            break;
                        }
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