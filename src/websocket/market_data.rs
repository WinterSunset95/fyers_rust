use crate::error::FyersError;
use crate::models::market_data::{fyers_v1};
use crate::models::websocket::{SubscriptionData, SubscriptionRequest, SubscriptionMode};
use serde::Serialize;
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
use reqwest::Client;

type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

const MARKET_DATA_URL: &str = "wss://socket.fyers.in/hsm/v1-5/prod";
const TBTWS_URL_REST_ENDPOINT: &str = "https://api-t1.fyers.in/indus/home/tbtws";

pub struct MarketDataSocket {
    app_id: String,
    access_token: String,
    http_client: Client,
    stream: Option<WsStream>,
}

impl MarketDataSocket {
    // TODO: Add a autoreconnect(i64) method to determine how many times the socket will try to
    // reconnect before giving up and comitting sepukku.
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
            http_client: Client::new(),
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
        let url = url::Url::parse(MARKET_DATA_URL).expect("Failed to parse market data url");
        let host = url
            .host_str()
            .expect("URL has no host")
            .to_string();

        let request = Request::builder()
            .uri(MARKET_DATA_URL)
            .header("Host", host)
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

    /// # Description
    /// Subscribe to a list of symbols
    ///
    /// # Argumebts
    /// * `symbols` - A slice of symbol strings to subscribe to eg. &["NSE:NIFTY50-INDEX","MCX:COPPER25SEPFUT"]
    pub async fn subscribe(&mut self, symbols: &[&str], mode: SubscriptionMode) -> Result<(), FyersError> {
        let request = SubscriptionRequest {
            request_type: 1,
            data: SubscriptionData {
                subs: 1,
                symbols,
                mode: mode as i32, // TODO: Determine what this magic number means (reverse engineer)
                channel: 1 // TODO: Determine what this magic number means (reverse engineer)
            },
        };
        self.send_request(&request).await
    }


    /// # Description
    /// Unsubscribe from a list of symbols
    ///
    /// # Arguments
    /// * `symbols` - A slice of symbol strings to Unsubscribe from eg. &["NSE:NIFTY50-INDEX","MCX:COPPER25SEPFUT"]
    pub async fn unsubscribe(&mut self, symbols: &[&str]) -> Result<(), FyersError> {
        let request = SubscriptionRequest {
            request_type: 1,
            data: SubscriptionData {
                subs: -1,
                symbols,
                mode: 1,
                channel: 1
            },
        };
        self.send_request(&request).await
    }

    // Private helper function that handles subscription logic
    pub async fn send_request<T: Serialize>(&mut self, request: &T) -> Result<(), FyersError> {
        let stream = self.stream.as_mut().ok_or_else(|| {
            FyersError::WebSocket("Cannot send request on a disconnected client".to_string())
        })?;

        let json_payload = serde_json::to_string_pretty(request)?;
        println!("Sending subscription message: {}", json_payload);
        stream.send(Message::Text(json_payload)).await?;
        Ok(())
    }

    /// # Description
    /// Enter the main loop to listen for incoming market data messages
    ///
    /// # Arguments
    /// * `handler` - A callback function that will be called with each successfully parsed
    /// message.
    pub async fn listen<F>(&mut self, mut handler: F) -> Result<(), FyersError>
        where 
            F: FnMut(fyers_v1::SocketMessage),
    {

        let stream = self
            .stream
            .as_mut()
            .ok_or_else(|| {
                FyersError::WebSocket("Cannot listen to a disconencted client.".to_string())
            })?;

        println!("Starting to listen to market data\n");

        while let Some(message_result) = stream.next().await {
            match message_result {
                Ok(message) => {
                    match message {
                        Message::Binary(bin_data) => {
                            println!("New binary data!\n");
                            match fyers_v1::SocketMessage::decode(&bin_data[..]) {
                                Ok(socket_message) => handler(socket_message),
                                Err(e) => eprintln!("Failed to decode Protobuf message: {}", e),
                            }
                        }
                        Message::Ping(ping_data) => {
                            println!("Recieved ping!! send Pong back!!");
                            stream.send(Message::Pong(ping_data)).await?;
                        }
                        Message::Text(text_data) => {
                            println!("New text data!\n");
                            if text_data.to_lowercase() != "pong" {
                                println!("Recieved unexpected text message: {}", text_data);
                            }
                        }
                        Message::Close(close_frame) => {
                            println!("Recieved close frame: {:?}", close_frame);
                            break;
                        }
                        _ => {}
                    }
                }
                Err(e) => {
                    eprintln!("Websocket protocol error!!: {}", e);
                }
            }
        }

        println!("Market data loop terminated.");
        self.stream = None;
        Ok(())
    }


}
