use thiserror::Error;

#[derive(Error, Debug)]
pub enum FyersError {
    // Error originating from the network layer
    #[error("Network request failed: {0}")]
    Network(#[from] reqwest::Error),

    // Error when the server's JSON response cannot be parsed into a rust struct
    #[error("Failed to parse server response: {0}")]
    Parse(#[from] serde_json::Error),

    // A specific error returned by the Fyers api itself
    #[error("FYERS API error: [Code: {code}] {message}")]
    ApiError { s: String, code: i64, message: String },

    // An error related to the auth flow
    #[error("Auth error: {0}")]
    AuthError(String),

    // Websocket errors
    #[error("Websocket error: {0}")]
    WebsocketError(#[from] tokio_tungstenite::tungstenite::Error),

    // Unknown errors
    #[error("Unknown error: {0}")]
    Unknown(String),
}
