use crate::error::FyersError;
use crate::models::{ SingleOrderResponse, MultipleOrdersResponse, SingleOrderRequest };
use reqwest::Client;

const FYERS_API_BASE_URL: &str = "https://api-t1.fyers.in/api/v3";

#[derive(Debug, Clone)]
pub struct Order {
    http_client: Client,
    app_id: String,
    access_token: String,
}

impl Order {
    pub fn new(app_id: String, access_token: String) -> Self {
        Self {
            http_client: Client::new(),
            app_id,
            access_token
        }
    }

    /// Place a single order to any exchange
    ///
    /// # Arguments
    /// * `order` - The order to place, as a SingleOrderRequest
    /// [API Docs](https://myapi.fyers.in/docsv3#tag/Order-Placement)
    pub async fn single_order(&self, order: &SingleOrderRequest) -> Result<SingleOrderResponse, FyersError> {
        let url = format!("{}/orders/sync", FYERS_API_BASE_URL);
        let auth_header_value = format!("{}:{}", self.app_id, self.access_token);
        let response = self
            .http_client
            .post(&url)
            .header("Authorization", auth_header_value)
            .json(order)
            .send()
            .await?;

        // First we check if API returned a non-success status code
        if !response.status().is_success() {
            return Err(FyersError::Network(response.error_for_status().unwrap_err()));
        }

        // Next we print the raw response, for debugging purposes
        let response_text = response.text().await?;
        println!("Raw response from /orders/sync:\n---\n{}\n---", response_text);

        // Parse it into json
        let order_response: SingleOrderResponse = serde_json::from_str(&response_text)?;

        if order_response.s == "ok" {
            Ok(order_response)
        } else {
            Err(FyersError::ApiError {
                s: order_response.s,
                code: order_response.code,
                message: order_response.message,
            })
        }
    }

    /// Place multiple orders to any exchanges
    ///
    /// # Arguments
    /// * `orders` - The orders to place, as an array of SingleOrderRequest
    /// [API Docs](https://myapi.fyers.in/docsv3#tag/Order-Placement)
    pub async fn multiple_orders(&self, orders: &Vec<SingleOrderRequest>) -> Result<MultipleOrdersResponse, FyersError> {
        unimplemented!()
    }

}
