use crate::error::FyersError;
use crate::models::{OrdersResponse};
use reqwest::Client;

const FYERS_API_BASE_URL: &str = "https://api-t1.fyers.in/api/v3";

#[derive(Debug, Clone)]
pub struct Transaction {
    http_client: Client,
    app_id: String,
    access_token: String,
}

impl Transaction {
    pub fn new(app_id: String, access_token: String) -> Self {
        Self {
            http_client: Client::new(),
            app_id,
            access_token
        }
    }


    /// Fetch all the orders placed by the user across all platforms and exchanges in the current
    /// trading day.
    /// [API Docs](https://myapi.fyers.in/docsv3#tag/Transaction-Info)
    pub async fn get_orders(&self) -> Result<OrdersResponse, FyersError> {
        let url = format!("{}/orders", FYERS_API_BASE_URL);
        let auth_header_value = format!("{}:{}", self.app_id, self.access_token);

        let response = self
            .http_client
            .get(&url)
            .header("Authorization", auth_header_value)
            .send()
            .await?;
        // First check if API returned a non-success status code
        if !response.status().is_success() {
            return Err(FyersError::Network(response.error_for_status().unwrap_err()));
        }

        let response_text = response.text().await?;
        println!("Raw response from /orders:\n---\n{}\n---", response_text);

        let orders_response: OrdersResponse = serde_json::from_str(&response_text)?;
        if orders_response.s == "ok" {
            Ok(orders_response)
        } else {
            Err(FyersError::ApiError {
                s: orders_response.s,
                code: orders_response.code,
                message: orders_response.message
            })
        }
    }


}

