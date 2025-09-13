pub mod helpers;
use fyers_rust::models::market_data;
use fyers_rust::{websocket::GeneralSocket, websocket::MarketDataSocket};
use fyers_rust::error::FyersError;
use serde_json::to_string_pretty;
use std::{fs, io};

#[tokio::main]
async fn main() -> Result<(), FyersError> {
    let config = helpers::config::load_config();
    let app_id = config.app_id;
    let access_token = config.access_token;

    let mut client = MarketDataSocket::new(app_id, access_token);

    match client.connect().await {
        Ok(_) => {
            println!("\nSuccess");
        }
        Err(e) => {
            eprintln!("\nFailure: {}", e);
        }
    }

    Ok(())

}
