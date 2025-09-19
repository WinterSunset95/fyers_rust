use serde::{Deserialize, Serialize};

// Structs for parsing the dynamic wss socket
#[derive(Deserialize)]
pub struct TbtwsData {
    pub socket_url: String,
}

#[derive(Deserialize)]
pub struct TbtwsResponse {
    pub data: TbtwsData,
}

// Structs for building the subscription JSON Message
#[derive(Clone, Copy, Debug)]
pub enum SubscriptionMode {
    Ping = 0,
    Quote = 1,
    ExtendedQuote = 2,
    DailyQuote = 3,
    MarketLevel = 4,
    Ohlcv = 5,
    Depth = 6,
    All = 7,
    Response = 8,
}

#[derive(Serialize)]
pub struct SubscriptionData<'a> {
    pub subs: i32,
    pub symbols: &'a [&'a str],
    pub mode: i32,
    pub channel: i32,
}

#[derive(Serialize)]
pub struct SubscriptionRequest<'a> {
    #[serde(rename = "type")]
    pub request_type: i32,
    pub data: SubscriptionData<'a>,
}

