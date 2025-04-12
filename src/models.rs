use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct KlineEvent {
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
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
    pub drawing_type: String,  // Тип объекта (например, "drawing.line", "drawing.brush")
    pub symbol: String,
    pub price: f64,
    pub time: i64,
    pub color: String,
    pub line_width: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WebSocketMessage {
    pub event_type: String,
    pub kline: Option<Kline>,
}