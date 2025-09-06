use crate::error::FyersError;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

const FYERS_API_BASE_URL: &str = "https://api-t1.fyers.in/api/v3";

/// The TokenResponse struct
#[derive(Deserialize, Debug)]
struct TokenResponse {
    s: String,
    code: i32,
    message: String,
    access_token: Option<String>
}

/// The TokenRequest struct
#[derive(Serialize, Debug)]
struct TokenRequest<'a> {
    grant_type: &'a str,
    #[serde(rename = "appIdHash")]
    app_id_hash: &'a str,
    code: &'a str,
}

/// # Description
/// Generate the initial authentication URL where user must log in. [Read more](https://myapi.fyers.in/docsv3#tag/Authentication-and-Login-Flow-User-Apps/paths/~1Authentication%20&%20Login%20Flow%20-%20User%20Apps/patch)
///
/// # Arguments
/// * `client_id` - FYERS client id
/// * `redirect_uri` - Redirect URI
/// * `state` - A unique, random string to prevent CSRF attacks
pub fn generate_auth_url(client_id: &str, redirect_uri: &str, state: &str) -> String {
    format!(
        "{}/generate-authcode?client_id={}&redirect_uri={}&response_type=code&state={}",
        FYERS_API_BASE_URL, client_id, redirect_uri, state
    )
}

/// # Description
/// Exchange the temporary `auth_code` from FYERS for a permanent `access_token`. [Read more](https://myapi.fyers.in/docsv3#tag/Authentication-and-Login-Flow-User-Apps/paths/~1Authentication%20&%20Login%20Flow%20-%20User%20Apps/patch)
/// 
/// # Arguments
/// * `client_id` - FYERS client id
/// * `client_secret` - FYERS client secret
/// * `auth_code` - Temporary authorization code
pub async fn generate_access_token(
    client_id: &str,
    client_secret: &str,
    auth_code: &str,
) -> Result<String, FyersError> {
    // The steps for authentication are outlined in the docs here:
    // * https://myapi.fyers.in/docsv3#tag/Authentication-and-Login-Flow-User-Apps/paths/~1Authentication%20&%20Login%20Flow%20-%20User%20Apps/patch

    // 1. Create SHA-256 hash of `client_id:secret_key`
    let to_hash = format!("{}:{}", client_id, client_secret);
    let mut hasher = Sha256::new();
    hasher.update(to_hash.as_bytes());
    let app_id_hash = format!("{:x}", hasher.finalize());

    // 2. Construct the JSON request body
    let request_body = TokenRequest {
        grant_type: "authorization_code",
        app_id_hash: &app_id_hash,
        code: auth_code,
    };

    // 3. Make POST request to the `/validate-authcode` endpoint
    let url = format!("{}/validate-authcode", FYERS_API_BASE_URL);
    let client = Client::new();
    let response = client
        .post(&url)
        .json(&request_body)
        .send()
        .await?;

    // 4. Parse the response and extract `access_token`
    if response.status().is_success() {
        let token_response = response.json::<TokenResponse>().await?;
        if token_response.s == "ok" {
            token_response.access_token.ok_or_else(|| {
                FyersError::AuthError("API returned ok, but no access token was found".to_string())
            })
        } else {
            Err(FyersError::ApiError { 
                s: token_response.s,
                code: token_response.code.into(),
                message: token_response.message
            })
        }
    } else {
        let status = response.status();
        let error_text = response.text().await.unwrap_or_else(|_| "Could not read error body".to_string());
        Err(FyersError::AuthError(format!(
            "Token validation failed with status: {} \n Body: {}",
            status, error_text
        )))
    }
}

