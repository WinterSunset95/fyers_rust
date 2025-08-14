use fyers_rust::auth;
use fyers_rust::client::FyersClient;
use fyers_rust::error::FyersError;
use fyers_rust::models::profile;
use std::{env, fs};
use std::io::{self, Write};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), FyersError> {
    dotenvy::dotenv().expect(".env file not found");
    let client_id = env::var("FYERS_APP_ID").expect("FYERS_APP_ID must be set");
    let secret_key = env::var("FYERS_SECRET").expect("FYERS_SECRET must be set");

    let redirect_url = "https://wintersunset95.github.io";
    let state = "sample_state";

    println!("\n Starting FYERS Authentication & Profile fetch \n");

    let auth_url = auth::generate_auth_url(&client_id, redirect_url, state);

    println!("ACTION REQUIRED: Please complete the manual login process");
    println!("1. Copy the following URL and paste it into your browser:");
    println!("\n{}\n", auth_url);
    println!("2. Log in to your FYERS account and authorize the application.");
    println!("3. You will be redirected. Copy that ENTIRE new URL from your browser's address bar.");
    println!("\n4. Paste here and press ENTER: ");
    io::stdout().flush().unwrap();
    let mut redirect_url_input = String::new();
    io::stdin().read_line(&mut redirect_url_input).unwrap();

    let auth_code = redirect_url_input
        .split('&')
        .find(|s| s.contains("auth_code="))
        .and_then(|s| s.split('=').nth(1))
        .expect("Cound not find 'auth_code' in the provided URL")
        .to_string();
    
    println!("\n Auth code extracted. Requesting access token from FYERS...");

    let access_token = auth::generate_access_token(&client_id, &secret_key, &auth_code.trim()).await?;
    println!("\n Access token generated. Fetching user profile...");

    println!("Fetching user profile...");
    let client = FyersClient::new(client_id, access_token.clone());

    match client.get_profile().await {
        Ok(profile) => {
            println!("\n Successfully fetched profile for {}", profile.name);
            println!("{:#?}", profile);
        }
        Err(e) => {
            eprintln!("Error fetching profile: {}", e);
        }
    }

    println!("\n Saving access token to .env file...");
    if let Err(e) = save_token_to_env(&access_token) {
        eprintln!("Error saving access token to .env file: {}", e);
    } else {
        println!("\n Access token saved to .env file.");
    }

    Ok(())

}

fn save_token_to_env(token: &str) -> io::Result<()> {
    let env_path = Path::new(".env");
    let mut new_lines: Vec<String> = Vec::new();
    let mut token_updated = false;

    if env_path.exists() {
        let content = fs::read_to_string(env_path)?;
        for line in content.lines() {
            if line.starts_with("FYERS_ACCESS_TOKEN=") {
                new_lines.push(format!("FYERS_ACCESS_TOKEN={}", token));
                token_updated = true;
            } else {
                new_lines.push(line.to_string());
            }
        }
    }

    if !token_updated {
        new_lines.push(format!("FYERS_ACCESS_TOKEN={}", token));
    }

    fs::write(env_path, new_lines.join("\n"))
}
