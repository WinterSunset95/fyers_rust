use crate::error::FyersError;
use crate::models::{ SingleOrderResponse, MultipleOrdersResponse, SingleOrderRequest };
use reqwest::Client;

const FYERS_API_BASE_URL: &str = "https://api-t1.fyers.in/api/v3";

/// The Order Class. Implements the [Order Placement](https://myapi.fyers.in/docsv3#tag/Order-Placement) section of the official Fyers API
#[derive(Debug, Clone)]
pub struct Order {
    http_client: Client,
    app_id: String,
    access_token: String,
}

impl Order {
    /// # Description
    /// Create a new instance of the Order class.
    ///
    /// # Arguments
    /// * `app_id` - The app id of the user.
    /// * `access_token` - The access token of the user.
    pub fn new(app_id: String, access_token: String) -> Self {
        Self {
            http_client: Client::new(),
            app_id,
            access_token
        }
    }

    /// # Description
    /// Place a single order to any exchange. [Read more](https://myapi.fyers.in/docsv3#tag/Order-Placement)
    ///
    /// # Arguments
    /// * `order` - The order to place, as a SingleOrderRequest
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

    /// # Description
    /// Place multiple orders to any exchanges. [Read more](https://myapi.fyers.in/docsv3#tag/Order-Placement)
    ///
    /// # Arguments
    /// * `orders` - The orders to place, as an array of SingleOrderRequest
    pub async fn multiple_orders(&self, orders: &Vec<SingleOrderRequest>) -> Result<MultipleOrdersResponse, FyersError> {
        unimplemented!()
    }
}
