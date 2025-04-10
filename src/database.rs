use redis::AsyncCommands;
use serde_json;

use crate::models::{DrawingLine, Kline};

pub struct Database {
    redis_client: redis::Client,
}

impl Database {
    pub fn new(redis_client: redis::Client) -> Self {
        Database { redis_client }
    }

    pub async fn store_kline(&self, symbol: &str, interval: &str, kline: &Kline) -> Result<(), redis::RedisError> {
        let mut conn = self.redis_client.get_multiplexed_async_connection().await?;
        let key = format!("kline:{}:{}", symbol, interval);
        let value = serde_json::to_string(kline).map_err(|e| redis::RedisError::from(std::io::Error::new(std::io::ErrorKind::Other, e)))?;
        let _: () = conn.zadd(key, value, kline.start_time).await?;
        Ok(())
    }

    #[allow(dead_code)]
    pub async fn get_historical_data(
        &self,
        symbol: &str,
        interval: &str,
        start_time: i64,
        end_time: i64,
    ) -> Result<Vec<Kline>, redis::RedisError> {
        let mut conn = self.redis_client.get_multiplexed_async_connection().await?;
        let key = format!("kline:{}:{}", symbol, interval);
        let data: Vec<String> = conn.zrangebyscore(key, start_time, end_time).await?;
        let klines: Vec<Kline> = data
            .iter()
            .filter_map(|item| serde_json::from_str(item).ok())
            .collect();
        Ok(klines)
    }

    pub async fn cache_historical_data(
        &self,
        key: &str,
        data: &Vec<serde_json::Value>,
    ) -> Result<(), redis::RedisError> {
        let mut conn = self.redis_client.get_multiplexed_async_connection().await?;
        let serialized_data = serde_json::to_string(data).map_err(|e| redis::RedisError::from(std::io::Error::new(std::io::ErrorKind::Other, e)))?;
        let _: () = conn.set_ex(key, serialized_data, 3600).await?;
        Ok(())
    }

    pub async fn get_cached_historical_data(&self, key: &str) -> Result<Option<Vec<serde_json::Value>>, redis::RedisError> {
        let mut conn = self.redis_client.get_multiplexed_async_connection().await?;
        let cached_data: Option<String> = conn.get(key).await?;
        Ok(cached_data.map(|data| serde_json::from_str(&data).unwrap_or_default()))
    }

    pub async fn save_drawing(&self, drawing: &DrawingLine) -> Result<(), redis::RedisError> {
        let mut conn = self.redis_client.get_multiplexed_async_connection().await?;
        let key = format!("drawings:{}", drawing.symbol);
        let line_json = serde_json::to_string(drawing).map_err(|e| redis::RedisError::from(std::io::Error::new(std::io::ErrorKind::Other, e)))?;
        let _: () = conn.lpush(key, line_json).await?;
        Ok(())
    }

    pub async fn load_drawings(&self, symbol: &str) -> Result<Vec<DrawingLine>, redis::RedisError> {
        let mut conn = self.redis_client.get_multiplexed_async_connection().await?;
        let key = format!("drawings:{}", symbol);
        let lines: Vec<String> = conn.lrange(key, 0, -1).await?;
        let parsed_lines: Vec<DrawingLine> = lines
            .iter()
            .filter_map(|l| serde_json::from_str(l).ok())
            .collect();
        Ok(parsed_lines)
    }
}