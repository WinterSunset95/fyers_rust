use fyers_rust::dataapi::DataApi;
use fyers_rust::error::FyersError;
use log::info;
use std::env;

#[tokio::main]
async fn main() -> Result<(), FyersError> {
    dotenvy::dotenv().expect(".env file not found");

    let app_id = env::var("FYERS_APP_ID").expect("FYERS_APP_ID must be set in .env");
    let access_token = env::var("FYERS_ACCESS_TOKEN").expect("FYERS_ACCESS_TOKEN must be set in .env");

    info!("\n Initializing DataApi with token from .env file");

    let data = DataApi::new(app_id, access_token);

    info!("\n Fetching historical data...");

    let response = data
        .get_historical_data(
            "NSE:SBIN-EQ",
            "60",
            "1",
            "2022-01-01",
            "2022-01-01",
            "0",
            "0",
        )
        .await?;

    if response.s == "ok" {
        Ok(())
    } else {
        Err(FyersError::Unknown("Error fetching historical data".to_string()))
    }


}
