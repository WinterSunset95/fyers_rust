pub mod helpers;
use fyers_rust::{transaction::Transaction};
use fyers_rust::error::FyersError;
use serde_json::to_string_pretty;
use std::{fs};

#[tokio::main]
async fn main() -> Result<(), FyersError> {
    let config = helpers::config::load_config();
    let app_id = config.app_id;
    let access_token = config.access_token;

    let transaction = Transaction::new(app_id, access_token);

    let positions = transaction.get_positions().await?;

    let filename = format!("transaction_positions.json");
    let json_data = to_string_pretty(&positions)?;
    fs::write(&filename, json_data).expect("Unable to write file");
    println!("\n Successfully fetched transaction positions");
    println!("\n Data saved to {}", &filename);
    println!("{:#?}", positions);

    if positions.s == "ok" {
        Ok(())
    } else {
        Err(FyersError::Unknown("Error fetching transaction positions".to_string()))
    }
}
