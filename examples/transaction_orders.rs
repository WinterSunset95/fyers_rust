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

    let orders = transaction.get_orders().await?;

    let filename = format!("transaction_orders.json");
    let json_data = to_string_pretty(&orders)?;
    fs::write(&filename, json_data).expect("Unable to write file");
    println!("\n Successfully fetched transaction orders");
    println!("\n Data saved to {}", &filename);
    println!("{:#?}", orders);

    if orders.s == "ok" {
        Ok(())
    } else {
        Err(FyersError::Unknown("Error fetching transaction orders".to_string()))
    }
}
