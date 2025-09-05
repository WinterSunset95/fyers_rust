use std::{env, fs};

pub struct Config {
    pub app_id: String,
    pub access_token: String,
}

pub fn load_config() -> Config {
    dotenvy::dotenv().expect(".env file not found");
    let app_id = env::var("FYERS_APP_ID").expect("FYERS_APP_ID must be set in .env");
    let access_token = env::var("FYERS_ACCESS_TOKEN").expect("FYERS_ACCESS_TOKEN must be set in .env");
    Config { app_id, access_token }
}
