use crate::error::FyersError;
use crate::models::user::{Profile, ProfileResponse};
use crate::models::FundsResponse;
use reqwest::Client;

const FYERS_API_BASE_URL: &str = "https://api-t1.fyers.in/api/v3";

#[derive(Debug, Clone)]
pub struct User {
    http_client: Client,
    app_id: String,
    access_token: String,
}

impl User {
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

        println!("Raw response from /profile:\n---\n{}\n---", response_text);

        // Parse the successful response. The actual profile data is nested.
        let profile_response: ProfileResponse = serde_json::from_str(&response_text)?;

        if profile_response.s == "ok" {
            Ok(profile_response.data)
        } else {
            Err(FyersError::ApiError {
                s: profile_response.s,
                code: profile_response.code,
                message: profile_response.message
            })
        }
    }

    /// Get the balance available for the user for capital as well as the commodity market
    /// [API Docs](https://myapi.fyers.in/docsv3#tag/User/paths/~1User/put)
    pub async fn get_funds(&self) -> Result<FundsResponse, FyersError> {
        let url = format!("{}/funds", FYERS_API_BASE_URL);
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
        println!("Raw response from /funds:\n---\n{}\n---", response_text);
        let funds_response: FundsResponse = serde_json::from_str(&response_text).unwrap();
        if funds_response.s == "ok" {
            Ok(funds_response)
        } else {
            Err(FyersError::ApiError {
                s: funds_response.s,
                code: funds_response.code,
                message: funds_response.message
            })
        }
    }
}
