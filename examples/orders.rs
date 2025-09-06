pub mod helpers;
use fyers_rust::models::SingleOrderRequest;
use fyers_rust::{orders::Order};
use fyers_rust::error::FyersError;
use serde_json::to_string_pretty;
use std::{fs, io};

async fn single_order(orderclass: Order) -> Result<(), FyersError> {
    let order_json = SingleOrderRequest {
        symbol: "NIFTY".to_string(),
        qty: 1,
        order_type: 2,
        side: 1,
        product_type: "INTRADAY".to_string(),
        validity: "DAY".to_string(),
        offline_order: false,

        limit_price: 0.0,
        stop_price: 0.0,
        disclosed_qty: 0,
        stop_loss: 0.0,
        take_profit: 0.0,
        order_tag: Some("tag1".to_string()),
    };

    let single_order = orderclass.single_order(&order_json).await?;

    let filename = format!("transaction_positions.json");
    let json_data = to_string_pretty(&single_order)?;
    fs::write(&filename, json_data).expect("Unable to write file");
    println!("\n Successfully fetched transaction single_order");
    println!("\n Data saved to {}", &filename);
    println!("{:#?}", single_order);

    if single_order.s == "ok" {
        Ok(())
    } else {
        Err(FyersError::Unknown("Error fetching transaction single_order".to_string()))
    }
}

async fn multi_order(orderclass: Order) -> Result<(), FyersError> {
    let order1_json = SingleOrderRequest {
        symbol: "NIFTY".to_string(),
        qty: 1,
        order_type: 2,
        side: 1,
        product_type: "INTRADAY".to_string(),
        validity: "DAY".to_string(),
        offline_order: false,

        limit_price: 0.0,
        stop_price: 0.0,
        disclosed_qty: 0,
        stop_loss: 0.0,
        take_profit: 0.0,
        order_tag: None,
    };
    let order2_json = SingleOrderRequest {
        symbol: "NIFTY".to_string(),
        qty: 1,
        order_type: 2,
        side: 1,
        product_type: "INTRADAY".to_string(),
        validity: "DAY".to_string(),
        offline_order: false,

        limit_price: 0.0,
        stop_price: 0.0,
        disclosed_qty: 0,
        stop_loss: 0.0,
        take_profit: 0.0,
        order_tag: None,
    };

    let multi_order_res = orderclass.multiple_orders(&vec![order1_json, order2_json]).await?;

    let filename = format!("transaction_positions.json");
    let json_data = to_string_pretty(&multi_order_res)?;
    fs::write(&filename, json_data).expect("Unable to write file");
    println!("\n Successfully fetched transaction multi_order_res");
    println!("\n Data saved to {}", &filename);
    println!("{:#?}", multi_order_res);

    if multi_order_res.s == "ok" {
        Ok(())
    } else {
        Err(FyersError::Unknown("Error fetching transaction single_order".to_string()))
    }
}

#[tokio::main]
async fn main() -> Result<(), FyersError> {
    let config = helpers::config::load_config();
    let app_id = config.app_id;
    let access_token = config.access_token;

    let order = Order::new(app_id, access_token);

    println!("\n\nSelect a method to run:\n");
    println!("1. single_order()\n");
    println!("2. multiple_orders()\n");
    println!("-> ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input = input.trim();

    let order = match input {
        "1" => single_order(order).await,
        "2" => multi_order(order).await,
        _ => Err(FyersError::Unknown("Invalid input".to_string())),
    };

    order

}
