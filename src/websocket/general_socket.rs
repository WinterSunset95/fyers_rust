use crate::error::FyersError;
use crate::models::market_data;
use futures_util::{StreamExt, SinkExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message, MaybeTlsStream, WebSocketStream};
use url::Url;
use prost::Message as ProstMessage;

type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

pub struct GeneralSocket {
    app_id: String,
    access_token: String,
    stream: Option<WsStream>,
}

impl GeneralSocket {
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
}
