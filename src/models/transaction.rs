use serde::{Serialize, Deserialize};

////////////
// Orders //
////////////

/// A single order entry
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub id: String,
    pub exch_ord_id: String,
    pub symbol: String,
    pub qty: i64,
    pub remaining_quantity: i64,
    pub filled_qty: i64,
    pub status: i64, // 1: cancelled, 2: traded/filled, 3: (not used currently), 4: transit, 5:
                     // rejected, 6: pending, 7: expired
    pub sl_no: i64,
    pub message: String,
    pub segment: i64,
    pub limit_price: f64,
    pub stop_price: f64,
    pub product_type: String,
    #[serde(rename = "type")]
    pub in_type: i64, // 1: limit order, 2: market order, 3: stop order(sl-m), 4: stoplimit
                   // order(sl-l)
    pub side: i64, // 1: buy, -1: sell
    pub disclosed_qty: i64,
    pub order_validity: String,
    pub order_date_time: String,
    pub parent_id: String,
    pub traded_price: f64,
    pub source: String,
    pub fytoken: String,
    pub offline_order: bool,
    pub pan: String,
    pub client_id: String,
    pub exchange: i64,
    pub instrument: i64,
    pub disclose_qty: i64,
    pub order_tag: String,
}

/// Top level response for the /orders endpoint
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrdersResponse {
    pub s: String,
    pub code: i64,
    pub message: String,
    pub order_book: Vec<Order>,
}

///////////////
// Positions //
///////////////

/// A Net position entry for the netPositions array
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NetPosition {
    pub symbol: String,
    pub id: String,
    pub buy_avg: f64,
    pub buy_qty: i64,
    pub sell_avg: f64,
    pub sell_qty: i64,
    pub net_avg: f64,
    pub net_qty: i64,
    pub side: i64,
    pub qty: i64,
    pub product_type: String,
    #[serde(rename = "realized_profit")]
    pub realized_profit: f64,
    pub pl: f64,
    pub cross_currency: String,
    pub rbi_ref_rate: f64,
    #[serde(rename = "qtyMulti_com")]
    pub qty_multi_com: f64,
    pub segment: i64,
    pub exchange: i64,
    pub sl_no: i64,
    pub ltp: f64,
    pub fy_token: String,
    pub cf_buy_qty: i64,
    pub cf_sell_qty: i64,
    pub day_buy_qty: i64,
    pub day_sell_qty: i64,
}

/// Overall positions
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct Overall {
    pub count_total: i64,
    pub count_open: i64,
    pub pl_total: f64,
    pub pl_realized: f64,
    pub pl_unrealized: f64,
}

/// The top level response for the /positions endpoint
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionsResponse {
    pub s: String,
    pub code: i64,
    pub message: String,
    pub net_positions: Vec<NetPosition>,
    pub overall: Overall,
}

////////////
// Trades //
////////////

/// A single trade item for the tradeBook array
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    pub symbol: String,
    pub row: i64,
    pub order_date_time: String,
    pub order_number: String,
    pub trade_number: String,
    pub trade_price: f64,
    pub trade_value: f64,
    pub traded_qty: i64,
    pub side: i64,

    //    1 => Buy
    //    -1 => Sell
    //    View Details

    pub product_type: String,
    pub exchange_order_no: String,
    pub segment: i64,
    pub exchange: i64,
    pub fy_token: String,
    pub order_tag: String,
    //Note: 1: will be concatenated at the start of tag provided by user.
    //2: will be concatenated at the start of tag generated internally by Fyers.
    //Default value if tag not provided when order is placed is 1:Untagged
}

/// The top level response for the /tradebook endpoint
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TradesResponse {
    pub s: String,
    pub code: i64,
    pub message: String,
    pub trade_book: Vec<Trade>,
}
