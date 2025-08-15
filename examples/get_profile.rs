use fyers_rust::client::FyersClient;
use fyers_rust::error::FyersError;
use std::env;

const FYERS_API_BASE_URL: &str = "https://api-t1.fyers.in/api/v3";

#[tokio::main]
async fn main() -> Result<(), FyersError> {
    dotenvy::dotenv().expect(".env file not found");

    let app_id = env::var("FYERS_APP_ID").expect("FYERS_APP_ID must be set in .env");
    let access_token = env::var("FYERS_ACCESS_TOKEN").expect("FYERS_ACCESS_TOKEN must be set in .env");

    println!("\n Initializing client with token from .env file");

    let client = FyersClient::new(FYERS_API_BASE_URL.to_string(), app_id, access_token);

    println!("\n Fetching user profile...");

    match client.get_profile().await {
        Ok(profile) => {
            println!("\n Successfully fetched profile for {}", profile.name);
            println!("{:#?}", profile);
        }
        Err(e) => {
            eprintln!("Error fetching profile: {}", e);
        }
    }

    Ok(())
}
