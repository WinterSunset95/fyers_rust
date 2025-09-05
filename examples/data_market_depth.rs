use fyers_rust::dataapi::DataApi;
use fyers_rust::error::FyersError;
use serde_json::to_string_pretty;
use std::{env, fs};

#[tokio::main]
async fn main() -> Result<(), FyersError> {
    dotenvy::dotenv().expect(".env file not found");
    let app_id = env::var("FYERS_APP_ID").expect("FYERS_APP_ID must be set in .env");
    let access_token = env::var("FYERS_ACCESS_TOKEN").expect("FYERS_ACCESS_TOKEN must be set in .env");

    println!("\n Initializing DataApi with token from .env file");
    let data = DataApi::new(app_id, access_token);

    println!("\n Fetching quote data ... ");

    let response = data
        .get_market_depth("NSE:SBIN-EQ", "1")
        .await?;

    let filename = format!("data_market_depth_{}.json", "NSE:SBIN-EQ");
    let json_data = to_string_pretty(&response)?;
    fs::write(&filename, json_data).expect("Unable to write file");
    println!("\n Successfully fetched quote data for {}", "NSE:SBIN-EQ");
    println!("\n Data saved to {}", &filename);

    if response.s == "ok" {
        Ok(())
    } else {
        Err(FyersError::Unknown("Error fetching market depth data".to_string()))
    }
}
