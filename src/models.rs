use serde::Deserialize;

#[derive(Deserialize)]
pub struct KlineEvent {
    #[serde(rename = "E")]
    pub event_time: u64,
    #[serde(rename = "s")]
    pub symbol: String,
    pub k: Kline,
}

#[derive(Deserialize)]
pub struct Kline {
    #[serde(rename = "t")]
    pub start_time: u64,
    #[serde(rename = "T")]
    #[allow(dead_code)]
    pub close_time: u64,
    #[serde(rename = "s")]
    #[allow(dead_code)]
    pub symbol: String,
    #[serde(rename = "i")]
    #[allow(dead_code)]
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
    #[allow(dead_code)]
    pub volume: String,
    #[serde(rename = "x")]
    #[allow(dead_code)]
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