pub mod helpers;
use fyers_rust::models::{market_data, SubscriptionMode};
use fyers_rust::{websocket::GeneralSocket, websocket::MarketDataSocket};
use fyers_rust::error::FyersError;
use serde_json::to_string_pretty;
use std::time::Duration;
use std::{fs, io};

#[tokio::main]
async fn main() -> Result<(), FyersError> {
    let config = helpers::config::load_config();
    let app_id = config.app_id;
    let access_token = config.access_token;

    let mut client = MarketDataSocket::new(app_id, access_token);

    client.connect().await?;

    let symbols = &["NSE:NIFTY50-INDEX","MCX:COPPER25SEPFUT"];

    client.subscribe(symbols, SubscriptionMode::Depth).await?;
    println!("Successfully subscribed to symbols: {:?}\n", symbols);

    println!("\n--- Listening to market data for 30 seconds ---\n");

    let listener_handle = tokio::spawn(async move {
        client.listen(|socket_message| {
            match to_string_pretty(&socket_message) {
                Ok(json_str) => println!("{}", json_str),
                Err(_) => eprintln!("Could not serialize socket message into json for printing."),
            }
        }).await
    });

    tokio::time::sleep(Duration::from_secs(30)).await;

    listener_handle.abort();
    println!("\n--- 30 seconds have elapsed. Test finisned. ---\n");

    Ok(())
}
