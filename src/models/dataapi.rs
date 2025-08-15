use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Candle(
    pub i64,
    pub f64,
    pub f64,
    pub f64,
    pub f64,
    pub i64
);

#[derive(Debug, Deserialize, Serialize)]
pub struct HistoryResponse {
    pub s: String,
    #[serde(default)]
    pub candles: Vec<Candle>,
    #[serde(default)]
    pub code: Option<i64>,
    #[serde(default)]
    pub message: Option<String>
}

// Convenience methods for Candle
impl Candle {
    pub fn timestamp(&self) -> i64 {
        self.0
    }
    pub fn open(&self) -> f64 {
        self.1
    }
    pub fn high(&self) -> f64 {
        self.2
    }
    pub fn low(&self) -> f64 {
        self.3
    }
    pub fn close(&self) -> f64 {
        self.4
    }
    pub fn volume(&self) -> i64 {
        self.5
    }
}
