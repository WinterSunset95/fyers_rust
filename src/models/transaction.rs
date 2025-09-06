use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use serde_with::{ serde_as, DisplayFromStr };

//////////////
/// Orders ///
//////////////

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
pub struct OrdersResponse {
    pub s: String,
    pub code: i64,
    pub message: String,
    pub orders: Vec<Order>,
}
//////////////
