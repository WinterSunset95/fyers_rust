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

    let trades = transaction.get_trades(None).await?;

    let filename = format!("transaction_positions.json");
    let json_data = to_string_pretty(&trades)?;
    fs::write(&filename, json_data).expect("Unable to write file");
    println!("\n Successfully fetched transaction trades");
    println!("\n Data saved to {}", &filename);
    println!("{:#?}", trades);

    if trades.s == "ok" {
        Ok(())
    } else {
        Err(FyersError::Unknown("Error fetching transaction trades".to_string()))
    }
}
