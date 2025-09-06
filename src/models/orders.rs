use serde::{Serialize, Deserialize};

/// Request structure for a single order
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderRequest {
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

    pub limit_price: Option<f64>,
    pub stop_price: Option<f64>,
    pub disclosed_qty: Option<i64>,
    pub validity: String,
    pub offline_order: bool,
    pub stop_loss: Option<f64>,
    pub take_profit: Option<f64>,
    pub order_tag: Option<String>,
}

/// Response structure for a single order
#[derive(Debug, Deserialize, Serialize)]
pub struct OrderResponse {
    pub s: String,
    pub code: i64,
    pub message: String,
    pub id: String,
}
