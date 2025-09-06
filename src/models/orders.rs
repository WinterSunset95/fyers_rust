use serde::{Serialize, Deserialize};

/// Request structure for a single order
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SingleOrderRequest {
    pub symbol: String,
    pub qty: i64,
    #[serde(rename = "type")]
    pub order_type: i64,

        //1 => Limit Order
        //2 => Market Order
        //3 => Stop Order (SL-M)
        //4 => Stoplimit Order (SL-L)

    pub side: i64,

        //1 => Buy
        //-1 => Sell

    pub product_type: String,

        //CNC => For equity only
        //INTRADAY => Applicable for all segments.
        //MARGIN => Applicable only for derivatives
        //CO => Cover Order
        //BO => Bracket Order
        //MTF => Approved Symbols Only

    pub validity: String,
    pub offline_order: bool,

    pub limit_price: f64,
    pub stop_price: f64,
    pub disclosed_qty: i64,
    pub stop_loss: f64,
    pub take_profit: f64,
    pub order_tag: Option<String>,
}

/// Response structure for a single order
#[derive(Debug, Deserialize, Serialize)]
pub struct SingleOrderResponse {
    pub s: String,
    pub code: i64,
    pub message: String,
    pub id: String,
}

/// A single 'data' field for the MultipleOrdersResponse
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub status_code: i64,
    pub body: SingleOrderResponse,
    pub status_description: String,
}

/// Response structure for multiple orders
#[derive(Debug, Deserialize, Serialize)]
pub struct MultipleOrdersResponse {
    pub s: String,
    pub code: i64,
    pub data: Vec<Data>,
    pub message: String,
}
