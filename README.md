
# FYERS API Rust SDK

[![Crates.io](https://img.shields.io/crates/v/fyers_rust)](https://crates.io/crates/fyers_rust)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI Status](https://github.com/WinterSunset95/fyers_rust/actions/workflows/ci.yml/badge.svg)](https://github.com/WinterSunset95/fyers_rust/actions)

A type-safe Rust implementation of the FYERS API for trading and market data.

## Features

- **Complete API Coverage**:
  - ✅ User Profile & Account APIs
  - 📈 Historical Market Data
  - 🔌 WebSocket Streaming
  - 💰 Order Management (Coming Soon)

- **Ergonomic Design**:

  ```rust
  let candles = fyers.market()
      .get_history("NSE:SBIN-EQ", "15", "2023-01-01", "2023-01-31")
      .await?;


 • Robust Error Handling:

   match fyers.user().get_profile().await {
       Ok(profile) => /*handle success */,
       Err(FyersError::ApiError { code, message }) => /* handle error*/,
   }

Installation

Add to your Cargo.toml:

[dependencies]
fyers_rust = { git = "<https://github.com/WinterSunset95/fyers_rust>" }

Quick Start

1. Authentication

use fyers_rust::Fyers;

let fyers = Fyers::new("APP_ID", "ACCESS_TOKEN");

2. Fetch Profile

let profile = fyers.user().get_profile().await?;
println!("Welcome {}", profile.name);

3. Get Historical Data

let candles = fyers.market().get_history(
    "NSE:NIFTY50-INDEX",
    "60",          // 60-minute candles
    "2023-01-01",  // Start date
    "2023-01-05"   // End date
).await?;

4. WebSocket Streaming

fyers.websocket()
    .subscribe(&["NSE:SBIN-EQ", "NSE:RELIANCE-EQ"])
    .on_tick(|tick| println!("{:?}", tick))
    .connect()
    .await?;

Examples

See the examples/ directory:

# Run with debug logging

RUST_LOG=debug cargo run --example auth_and_profile

# Save historical data to file

cargo run --example data_history

Documentation

API Docs

Configuration

Environment Variables

# .env file

FYERS_APP_ID="your-app-id"
FYERS_SECRET="your-secret-key"
FYERS_ACCESS_TOKEN="your-access-token"

Logging

// Initialize default logger (info level)
fyers_rust::init_logger();

// Or configure manually:
env_logger::Builder::new()
    .filter_level(LevelFilter::Debug)
    .init();

Project Structure

src/
├── api/
│   ├── mod.rs      # Main exports
│   ├── user.rs     # Profile, auth status
│   ├── market.rs   # Historical data
│   └── orders.rs   # Trade operations (WIP)
├── websocket.rs    # Real-time data
└── client.rs       # Shared HTTP client

Development

# Run tests

cargo test -- --nocapture

# Build docs

cargo doc --open

# Fix warnings

cargo fix --allow-staged

Roadmap

 • [ ] Order management API
 • [ ] Advanced charting utilities
 • [ ] Backtesting framework
 • [ ] Async streaming interface

License

MIT © WinterSunset95
