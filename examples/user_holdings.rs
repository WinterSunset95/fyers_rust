mod helpers;
use fyers_rust::user::User;
use fyers_rust::error::FyersError;
use serde_json::to_string_pretty;
use std::{env, fs};

#[tokio::main]
async fn main() -> Result<(), FyersError> {
    let config = helpers::config::load_config();
    let app_id = config.app_id;
    let access_token = config.access_token;

    let user = User::new(app_id, access_token);
    println!("\n Fetching user holdings ... ");

    let response = user
        .get_holdings()
        .await?;

    // Save to file
    let filename = format!("user_holdings.json");
    let json_data = to_string_pretty(&response)?;
    fs::write(&filename, json_data).expect("Unable to write file");
    println!("\n Successfully fetched user holdings");
    println!("\n Data saved to {}", &filename);
    println!("{:#?}", response);

    if response.s == "ok" {
        Ok(())
    } else {
        Err(FyersError::Unknown("Error fetching user holdings".to_string()))
    }

}
