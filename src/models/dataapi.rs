use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use serde_with::{ serde_as, DisplayFromStr };

/// A single candle
#[derive(Debug, Deserialize, Serialize)]
pub struct Candle(
    pub i64,
    pub f64,
    pub f64,
    pub f64,
    pub f64,
    pub i64
);

/// Top level response for a history request
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

/// Convenience methods for Candle
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

/// Quote data for symbol(s)
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

/// Quote data for a single symbol
#[derive(Debug, Deserialize, Serialize)]
pub struct SymbolQuoteResponse {
    pub s: String,
    pub n: String,
    pub v: QuoteData,
}

/// Top level response for a quote request
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

/// A single depth entry
#[derive(Debug, Deserialize, Serialize)]
pub struct DepthEntry {
    pub price: f64,
    pub volume: i64,
    pub ord: i64,
}

/// Market Depth
#[derive(Debug, Deserialize, Serialize)]
pub struct MarketDepthData {
    pub totalbuyqty: i64,
    pub totalsellqty: i64,
    pub bids: Vec<DepthEntry>,
    pub ask: Vec<DepthEntry>,
    pub o: f64,
    pub h: f64,
    pub l: f64,
    pub c: f64,
    pub chp: f64,

    #[serde(rename = "tick_Size")]
    pub tick_size: f64,

    pub ch: f64,
    pub ltq: f64,
    pub ltt: f64,
    pub ltp: f64,
    pub v: i64,
    pub atp: f64,
    pub lower_ckt: f64,
    pub upper_ckt: f64,
    pub expiry: String,
    pub oi: f64,
    pub oiflag: bool,
    pub pdoi: i64,
    pub oipercent: f64,
}

/// Top level response for a market depth request
#[derive(Debug, Deserialize, Serialize)]
pub struct MarketDepthResponse {
    pub s: String,

    #[serde(default)]
    pub d: HashMap<String, MarketDepthData>,

    #[serde(default)]
    pub code: Option<i64>,
    #[serde(default)]
    pub message: Option<String>
}

/// A single Call or Put option contract in the chain
#[derive(Debug, Deserialize, Serialize)]
pub struct OptionData {
    pub ask: f64,
    pub bid: f64,
    #[serde(rename = "fyToken")]
    pub fy_token: String,
    pub ltp: f64,
    pub ltpch: f64,
    pub ltpchp: f64,
    pub oi: i64,
    pub oich: i64,
    pub oichp: f64,
    pub option_type: String, // "CE" or "PE"
    pub prev_oi: i64,
    pub strike_price: f64,
    pub symbol: String,
    pub volume: i64,
}

/// The underlying security (the first element in the option chain response)
#[derive(Debug, Deserialize, Serialize)]
pub struct UnderlyingData {
    pub ask: f64,
    pub bid: f64,
    pub description: String,
    pub ex_symbol: String,
    pub exchange: String,
    pub fp: f64,
    pub fpch: f64,
    pub fpchp: f64,
    #[serde(rename = "fyToken")]
    pub fy_token: String,
    pub ltp: f64,
    pub ltpch: f64,
    pub ltpchp: f64,
    pub option_type: String,
    pub strike_price: f64,
    pub symbol: String,
}

/// An entry in the 'optionsChain' array, which can be either the underlying security or a specific
/// option contract. 'untagged' tells serde to try parsing each variant until one succeeds.
#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ChainEntry {
    Underlying(UnderlyingData),
    Option(OptionData),
}

/// The data for the India VIX index
#[derive(Debug, Deserialize, Serialize)]
pub struct IndiaVixData {
    pub ask: f64,
    pub bid: f64,
    pub description: String,
    pub ex_symbol: String,
    pub exchange: String,
    #[serde(rename = "fyToken")]
    pub fy_token: String,
    pub ltp: f64,
    pub ltpch: f64,
    pub ltpchp: f64,
    pub option_type: String,
    pub strike_price: f64,
    pub symbol: String,
}

/// An expiry date for the option chain
#[serde_as]
#[derive(Debug, Deserialize, Serialize)]
pub struct ExpiryData {
    pub date: String,
    #[serde_as(as = "DisplayFromStr")]
    pub expiry: i64,
}

/// Main data payload of the option chain
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChainData {
    pub call_oi: i64,
    pub put_oi: i64,
    pub expiry_data: Vec<ExpiryData>,
    pub indiavix_data: IndiaVixData,
    pub options_chain: Vec<ChainEntry>,
}

/// The top-level response for an Option Cahin request
#[derive(Debug, Deserialize, Serialize)]
pub struct OptionChainResponse {
    pub s: String,
    #[serde(default)]
    pub data: Option<ChainData>,
    #[serde(default)]
    pub code: Option<i64>,
    #[serde(default)]
    pub message: Option<String>
}
