use crate::error::FyersError;
use crate::models::market_data;
use futures_util::{StreamExt, SinkExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{
        self, handshake::client::Request, protocol::Message
    },
    MaybeTlsStream,
    WebSocketStream
};
use prost::Message as ProstMessage;

type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

const MARKET_DATA_URL: &str = "wss://socket.fyers.in/hsm/v1-5/prod";

pub struct MarketDataSocket {
    app_id: String,
    access_token: String,
    stream: Option<WsStream>,
}

impl MarketDataSocket {
    /// # Description
    /// Connect to a given Websocket URL with the provided authentication
    ///
    /// # Arguments
    /// * `app_id` - The app id of the user.
    /// * `access_token` - The access token of the user.
    pub fn new(app_id: String, access_token: String) -> Self {
        Self {
            app_id,
            access_token,
            stream: None,
        }
    }

    /// # Description
    /// Connect to the Market Data Websocket endpoint
    pub async fn connect(&mut self) -> Result<(), FyersError> {
        if self.stream.is_some() {
            return Err(FyersError::WebSocket("Client is already connected.".to_string()));
        }

        let auth_token = format!("{}:{}", self.app_id, self.access_token);

        let request = Request::builder()
            .uri(MARKET_DATA_URL)
            .header("Authorization", auth_token)
            .header("Sec-WebSocket-Version", "13")
            .header("Sec-WebSocket-Key", tungstenite::handshake::client::generate_key())
            .header("Connection", "Upgrade")
            .header("Upgrade", "websocket")
            .body(())
            .expect("Failed to build WebSocket request");

        let (stream, response) = connect_async(request).await?;

        if !response.status().is_informational() {
            return Err(FyersError::WebSocket(format!("Websocket handshake failed with status: {}", response.status())));
        }

        println!("Successfully connected to the market data websocket");
        self.stream = Some(stream);
        Ok(())
    }
}
