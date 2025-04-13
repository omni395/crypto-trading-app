use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct KlineEvent {
    #[serde(rename = "E")]
    #[allow(dead_code)]
    pub event_time: u64,
    #[serde(rename = "s")]
    #[allow(dead_code)]
    pub symbol: String,
    pub k: Kline,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Kline {
    #[serde(rename = "t")]
    pub start_time: u64,
    #[serde(rename = "T")]
    pub close_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    #[serde(rename = "i")]
    pub interval: String,
    #[serde(rename = "o")]
    pub open: String,
    #[serde(rename = "h")]
    pub high: String,
    #[serde(rename = "l")]
    pub low: String,
    #[serde(rename = "c")]
    pub close: String,
    #[serde(rename = "v")]
    pub volume: String,
    #[serde(rename = "x")]
    pub is_closed: bool,
}

#[derive(Deserialize)]
pub struct DepthEvent {
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    pub bids: Vec<Vec<String>>,
    pub asks: Vec<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Drawing {
    pub id: String,           // Уникальный идентификатор
    pub drawing_type: String, // Тип объекта ("drawing.line", "drawing.ray", "drawing.trendline")
    pub symbol: String,       // Символ криптовалюты
    pub data: String,         // Данные в формате JSON
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebSocketMessage {
    pub event_type: String,
    pub kline: Option<Kline>,
}