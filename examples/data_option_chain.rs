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

    println!("\n Fetching option chain data ... ");

    let response = data
        .get_option_chain("NSE:SBIN-EQ", Some("2"), None)
        .await?;

    let filename = format!("data_option_chain_{}.json", "NSE:NIFTY22SEP17500CE");
    let json_data = to_string_pretty(&response)?;
    fs::write(&filename, json_data).expect("Unable to write file");
    println!("\n Successfully fetched option chain data for {}", "NSE:NIFTY22SEP17500CE");
    println!("\n Data saved to {}", &filename);

    if response.s == "ok" {
        Ok(())
    } else {
        Err(FyersError::Unknown("Error fetching option chain data".to_string()))
    }
}
