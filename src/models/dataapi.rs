use serde::{Serialize, Deserialize};
use serde_with::{ serde_as, DisplayFromStr };

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

#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
pub struct QuoteData {
    pub ch: f64,
    pub chp: f64,
    pub lp: f64,
    pub spread: f64,
    pub ask: f64,
    pub bid: f64,
    pub open_price: f64,
    pub high_price: f64,
    pub low_price: f64,
    pub prev_close_price: f64,
    pub atp: f64,
    pub volume: i64,
    pub short_name: String,
    pub exchange: String,
    pub description: String,
    pub original_name: String,
    pub symbol: String,
    #[serde(rename = "fyToken")]
    pub fy_token: String,
    #[serde_as(as = "DisplayFromStr")]
    pub tt: i64,
    #[serde(default)]
    pub code: Option<i64>,
    #[serde(default)]
    pub message: Option<String>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SymbolQuoteResponse {
    pub s: String,
    pub n: String,
    pub v: QuoteData,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QuoteResponse {
    pub s: String,
    #[serde(default)]
    pub d: Vec<SymbolQuoteResponse>,
    #[serde(default)]
    pub code: Option<i64>,
    #[serde(default)]
    pub message: Option<String>
}
