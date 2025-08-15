use crate::error::FyersError;
use reqwest::Client;
use serde::de::Error;
use log::{debug, info, error};

#[derive(Debug, Clone)]
pub struct DataApi {
    http_client: Client,
    base_url: String,
    app_id: String,
    access_token: String,
}

impl DataApi {
    pub fn new(base_url: String, app_id: String, access_token: String) -> Self {
        Self {
            http_client: Client::new(),
            base_url,
            app_id,
            access_token
        }
    }
}
