use crate::error::FyersError;
use crate::models::{OrdersResponse, PositionsResponse};
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
    ///
    /// # Arguments
    /// * `id` - Optional order ID to filter the results.
    /// * `order_tag` - Optional order tag to filter the results.
    /// [API Docs](https://myapi.fyers.in/docsv3#tag/Transaction-Info)
    pub async fn get_orders(&self, id: Option<&str>, order_tag: Option<&str>) -> Result<OrdersResponse, FyersError> {
        let mut url = format!("{}/orders", FYERS_API_BASE_URL);
        let mut query_params = vec![];
        if let Some(id) = id {
            query_params.push(format!("id={}", id));
        }
        if let Some(order_tag) = order_tag {
            query_params.push(format!("order_tag={}", order_tag));
        }
        if !query_params.is_empty() {
            url = format!("{}?{}", url, query_params.join("&"));
        }

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

    /// Fetch the current open and closed positions for the current trading day. Not that the
    /// previous day's closed positions will not be shown here.
    ///
    /// [API Docs](https://myapi.fyers.in/docsv3#tag/Transaction-Info/paths/~1positions/get)
    pub async fn get_positions(&self) -> Result<PositionsResponse, FyersError> {
        let url = format!("{}/positions", FYERS_API_BASE_URL);
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
        println!("Raw response from /positions:\n---\n{}\n---", response_text);

        let positions_response: PositionsResponse = serde_json::from_str(&response_text)?;
        if positions_response.s == "ok" {
            Ok(positions_response)
        } else {
            Err(FyersError::ApiError {
                s: positions_response.s,
                code: positions_response.code,
                message: positions_response.message
            })
        }
    }


}

