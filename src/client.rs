use crate::error::FyersError;
use crate::models::profile::{Profile, ProfileResponse};
use reqwest::Client;
use serde::de::Error;

const FYERS_API_BASE_URL: &str = "https://api-t1.fyers.in/api/v3";

#[derive(Debug, Clone)]
pub struct FyersClient {
    http_client: Client,
    app_id: String,
    access_token: String,
}

impl FyersClient {
    pub fn new(app_id: String, access_token: String) -> Self {
        Self {
            http_client: Client::new(),
            app_id,
            access_token
        }
    }

    /// Fetch user's profile information.
    /// [API Docs](https://myapi.fyers.in/docsv3#tag/User/paths/~1User/post)
    pub async fn get_profile(&self) -> Result<Profile, FyersError> {
        let url = format!("{}/profile", FYERS_API_BASE_URL);
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

        println!("DEBUG: Raw response from /profile:\n---\n{}\n---", response_text);

        // Parse the successful response. The actual profile data is nested.
        let profile_response: ProfileResponse = serde_json::from_str(&response_text)?;

        if profile_response.s == "ok" {
            Ok(profile_response.data)
        } else {
            Err(FyersError::ApiError {
                code: profile_response.code,
                message: profile_response.message
            })
        }
    }

    /// Fetch market data for given symbols.
    /// [API Docs](https://myapi.fyers.in/docsv3#tag/Data-Api/paths/~1DataApi/post)
    /// 
    /// # Arguments
    /// * `symbols` - Comma separated list of symbols (e.g. "NSE:SBIN-EQ,NSE:RELIANCE-EQ")
    /// * `data_type` - Type of data to fetch ("symbolData" or "marketDepth")
    pub async fn get_market_data(&self, symbols: &str, data_type: &str) -> Result<serde_json::Value, FyersError> {
        let url = format!("{}/market-data", FYERS_API_BASE_URL);
        let auth_header_value = format!("{}:{}", self.app_id, self.access_token);

        let request_body = serde_json::json!({
            "symbols": symbols,
            "dataType": data_type
        });

        let response = self
            .http_client
            .post(&url)
            .header("Authorization", auth_header_value)
            .json(&request_body)
            .send()
            .await?;

        // First check if API returned a non-success status code
        if !response.status().is_success() {
            return Err(FyersError::Network(response.error_for_status().unwrap_err()));
        }

        let response_json: serde_json::Value = response.json().await?;

        // Check if API returned an error in the response body
        if let Some(s) = response_json.get("s").and_then(|v| v.as_str()) {
            if s != "ok" {
                return Err(FyersError::ApiError {
                    code: response_json.get("code").and_then(|v| v.as_i64()).unwrap_or(0),
                    message: response_json.get("message").and_then(|v| v.as_str()).unwrap_or("Unknown error").to_string()
                });
            }
        }

        Ok(response_json)
    }
}
