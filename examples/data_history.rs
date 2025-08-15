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

    println!("\n Fetching historical data...");

    let response = data
        .get_historical_data(
            "NSE:NIFTY50-INDEX",
            "60",
            "1",
            "2025-08-14",
            "2025-08-15",
            "0",
            "0",
        )
        .await?;

    let filename = format!("data_history_{}_{}_{}_{}_{}_{}_{}.json", "NSE:SBIN-EQ", "60", "1", "2022-01-01", "2022-01-01", "0", "0");
    let json_data = to_string_pretty(&response)?;
    fs::write(&filename, json_data).expect("Unable to write file");
    println!("\n Successfully fetched historical data for {}", "NSE:SBIN-EQ");
    println!("\n Data saved to {}", &filename);

    if response.s == "ok" {
        Ok(())
    } else {
        Err(FyersError::Unknown("Error fetching historical data".to_string()))
    }


}
