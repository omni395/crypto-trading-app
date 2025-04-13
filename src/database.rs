use crate::models::Drawing;
use redis::{AsyncCommands, RedisError, ErrorKind};
use serde_json;

pub async fn save_drawing(mut con: redis::aio::MultiplexedConnection, drawing: &Drawing) -> redis::RedisResult<()> {
    let serialized = serde_json::to_string(drawing)
        .map_err(|e| RedisError::from((ErrorKind::TypeError, "Serialization error", e.to_string())))?;
    let key = format!("drawing:{}:{}:{}", drawing.drawing_type, drawing.symbol, drawing.id);
    con.set::<_, _, ()>(&key, serialized).await?;
    Ok(())
}

pub async fn load_drawings(
    mut con: redis::aio::MultiplexedConnection,
    symbol: &str,
) -> redis::RedisResult<Vec<Drawing>> {
    let keys: Vec<String> = con.keys(format!("drawing:*:{}:*", symbol)).await?;
    let mut drawings = Vec::new();

    for key in keys {
        let serialized: String = con.get(&key).await?;
        let drawing: Drawing = serde_json::from_str(&serialized)
            .map_err(|e| RedisError::from((ErrorKind::TypeError, "Deserialization error", e.to_string())))?;
        drawings.push(drawing);
    }

    Ok(drawings)
}

pub async fn delete_drawing(
    mut con: redis::aio::MultiplexedConnection,
    drawing_type: &str,
    symbol: &str,
    id: &str,
) -> redis::RedisResult<()> {
    let key = format!("drawing:{}:{}:{}", drawing_type, symbol, id);
    con.del::<_, ()>(&key).await?;
    Ok(())
}

pub async fn save_historical_data(
    mut con: redis::aio::MultiplexedConnection,
    symbol: &str,
    interval: &str,
    time: i64,
    open: &str,
    high: &str,
    low: &str,
    close: &str,
    volume: &str,
) -> redis::RedisResult<()> {
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
    con.set::<_, _, ()>(&key, serialized).await?;
    Ok(())
}

pub async fn load_historical_data(
    mut con: redis::aio::MultiplexedConnection,
    symbol: &str,
    interval: &str,
    start_time: i64,
    end_time: i64,
) -> redis::RedisResult<Vec<serde_json::Value>> {
    let keys: Vec<String> = con.keys(format!("historical:{}:{}:*", symbol, interval)).await?;
    let mut historical_data = Vec::new();

    for key in keys {
        let serialized: String = con.get(&key).await?;
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