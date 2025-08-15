use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Candle {
    pub t: i64,
    pub o: f64,
    pub h: f64,
    pub l: f64,
    pub c: f64,
    pub v: i64
}

#[derive(Debug, Deserialize)]
pub struct HistoryResponse {
    pub s: String,
    #[serde(rename = "Candles")]
    pub candles: Vec<Candle>
}
