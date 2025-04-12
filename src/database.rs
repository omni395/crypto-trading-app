use crate::models::Drawing;
use redis::{Commands, Connection, RedisError, ErrorKind};
use serde_json;

#[allow(deprecated)]
pub fn save_drawing(con: &mut Connection, drawing: &Drawing) -> redis::RedisResult<()> {
    let serialized = serde_json::to_string(drawing)
        .map_err(|e| RedisError::from((ErrorKind::TypeError, "Serialization error", e.to_string())))?;
    let key = format!("{}:{}:{}", drawing.drawing_type, drawing.symbol, drawing.time);
    con.set(&key, serialized)?;
    Ok(())
}

pub fn load_drawings(con: &mut Connection, drawing_type: &str, symbol: &str) -> redis::RedisResult<Vec<Drawing>> {
    let keys: Vec<String> = con.keys(format!("{}:{}:*", drawing_type, symbol))?;
    let mut drawings = Vec::new();

    for key in keys {
        let serialized: String = con.get(&key)?;
        let drawing: Drawing = serde_json::from_str(&serialized)
            .map_err(|e| RedisError::from((ErrorKind::TypeError, "Deserialization error", e.to_string())))?;
        drawings.push(drawing);
    }

    Ok(drawings)
}

#[allow(deprecated)]
pub fn delete_drawing(con: &mut Connection, drawing_type: &str, symbol: &str, price: f64, time: i64) -> redis::RedisResult<()> {
    let key = format!("{}:{}:{}", drawing_type, symbol, time);
    let exists: bool = con.exists(&key)?;
    if exists {
        let serialized: String = con.get(&key)?;
        let drawing: Drawing = serde_json::from_str(&serialized)
            .map_err(|e| RedisError::from((ErrorKind::TypeError, "Deserialization error", e.to_string())))?;
        if (drawing.price - price).abs() < f64::EPSILON {
            con.del(&key)?;
        }
    }
    Ok(())
}

#[allow(deprecated)]
pub fn save_historical_data(con: &mut Connection, symbol: &str, interval: &str, time: i64, open: &str, high: &str, low: &str, close: &str, volume: &str) -> redis::RedisResult<()> {
    let key = format!("historical:{}:{}:{}", symbol, interval, time);
    let value = serde_json::json!({
        "symbol": symbol,
        "interval": interval,
        "time": time,
        "open": open,
        "high": high,
        "low": low,
        "close": close,
        "volume": volume,
    });
    let serialized = serde_json::to_string(&value)
        .map_err(|e| RedisError::from((ErrorKind::TypeError, "Serialization error", e.to_string())))?;
    con.set(&key, serialized)?;
    Ok(())
}

pub fn load_historical_data(con: &mut Connection, symbol: &str, interval: &str, start_time: i64, end_time: i64) -> redis::RedisResult<Vec<serde_json::Value>> {
    let keys: Vec<String> = con.keys(format!("historical:{}:{}:*", symbol, interval))?;
    let mut historical_data = Vec::new();

    for key in keys {
        let serialized: String = con.get(&key)?;
        let data: serde_json::Value = serde_json::from_str(&serialized)
            .map_err(|e| RedisError::from((ErrorKind::TypeError, "Deserialization error", e.to_string())))?;
        let time = data["time"].as_i64().unwrap_or(0);
        if time >= start_time && time <= end_time {
            historical_data.push(data);
        }
    }

    historical_data.sort_by(|a, b| a["time"].as_i64().cmp(&b["time"].as_i64()));
    Ok(historical_data)
}