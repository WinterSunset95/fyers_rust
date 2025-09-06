use crate::error::FyersError;
use crate::models::user::{Profile, ProfileResponse};
use crate::models::{FundsResponse, HoldingsResponse};
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
}

