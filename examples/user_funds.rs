use fyers_rust::user::User;
use fyers_rust::error::FyersError;
use serde_json::to_string_pretty;
use std::{env, fs};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect(".env file not found");
    let app_id = env::var("FYERS_APP_ID").expect("FYERS_APP_ID must be set in .env");
    let access_token = env::var("FYERS_ACCESS_TOKEN").expect("FYERS_ACCESS_TOKEN must be set in .env");

    println!("\n Initializing DataApi with token from .env file");
    let user = User::new(app_id, access_token);

    println!("\n Fetching quote data ... ");

    let response = user
        .get_funds()
        .await;

}
