use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::{stream::StreamExt, sink::SinkExt};
use serde_json;
use crate::models::{KlineEvent, DepthEvent};
use actix_web::web;
use crate::websocket::AppState;

pub async fn connect_to_binance(state: web::Data<AppState>) {
    let url = "wss://stream.binance.com:9443/ws/btcusdt@kline_1m/btcusdt@depth20";
    match connect_async(url).await {
        Ok((mut ws_stream, _)) => {
            println!("Подключено к Binance WebSocket");

            while let Some(msg) = ws_stream.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        if text.contains("\"e\":\"kline\"") {
                            match serde_json::from_str::<KlineEvent>(&text) {
                                Ok(kline_event) => {
                                    println!("Получены свечные данные: {:?}", kline_event);
                                    let mut clients = state.clients.lock().await;
                                    let kline_json = serde_json::to_string(&kline_event).unwrap();
                                    for client in clients.iter_mut() {
                                        client.do_send(crate::websocket::ClientMessage(kline_json.clone()));
                                    }
                                }
                                Err(e) => println!("Ошибка парсинга свечных данных: {:?}", e),
                            }
                        } else if text.contains("\"e\":\"depthUpdate\"") {
                            match serde_json::from_str::<DepthEvent>(&text) {
                                Ok(depth_event) => {
                                    println!("Получены данные стакана: {:?}", depth_event);
                                    let mut clients = state.clients.lock().await;
                                    let depth_json = serde_json::to_string(&depth_event).unwrap();
                                    for client in clients.iter_mut() {
                                        client.do_send(crate::websocket::ClientMessage(depth_json.clone()));
                                    }
                                }
                                Err(e) => println!("Ошибка парсинга данных стакана: {:?}", e),
                            }
                        }
                    }
                    Ok(Message::Ping(ping)) => {
                        let _ = ws_stream.send(Message::Pong(ping)).await;
                    }
                    Err(e) => println!("Ошибка WebSocket: {:?}", e),
                    _ => (),
                }
            }
        }
        Err(e) => println!("Ошибка подключения к Binance: {:?}", e),
    }
}