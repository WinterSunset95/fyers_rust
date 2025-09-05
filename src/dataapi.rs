use crate::error::FyersError;
use crate::models::{ HistoryResponse, MarketDepthResponse, QuoteResponse, OptionChainResponse };
use reqwest::Client;

#[derive(Debug, Clone)]
pub struct DataApi {
    http_client: Client,
    app_id: String,
    access_token: String,
}

const DATA_API_BASE_URL: &str = "https://api-t1.fyers.in/data";

impl DataApi {
    pub fn new(app_id: String, access_token: String) -> Self {
        Self {
            http_client: Client::new(),
            app_id,
            access_token,
        }
    }

    /// Get Historical data (up to date) for a given symbol. Record is presented in the form of
    /// candle data.
    ///
    /// # Arguments
    /// * `symbol` - Symbol for which data is to be fetched (e.g. "NSE:SBIN-EQ")
    /// * `resolution` - Candle resolution. (e.g. "5S", "15S", "1", "5")
    /// * `date_format` - Date format. 0 to enter the epoch value. Eg: 670073472, 1 to enter the
    /// date in yyyy-MM-dd format. (e.g. "2022-01-01")
    /// * `range_from` - Indicating the start date of records. (e.g. "2022-01-01" or 670073472)
    /// * `range_to` - Indicating the end date of records. (e.g. "2022-01-01" or 670073472)
    /// * `cont_flag` - Indicating if records are to be fetched in continuous mode. (e.g. 0 or 1)
    /// * `oi_flag` - Indicating if open interest data is to be fetched. (e.g. 0 or 1)
    /// [API Docs](https://myapi.fyers.in/docsv3#tag/Data-Api/paths/~1DataApi/post)
    pub async fn get_historical_data(
        &self,
        symbol: &str,
        resolution: &str,
        date_format: &str,
        range_from: &str,
        range_to: &str,
        cont_flag: &str,
        oi_flag: &str,
    ) -> Result<HistoryResponse, FyersError> {
        let url = format!("{}/history?symbol={}&resolution={}&date_format={}&range_from={}&range_to={}&cont_flag={}&oi_Flag={}",
            DATA_API_BASE_URL, symbol, resolution, date_format, range_from, range_to, cont_flag, oi_flag
            );
        let auth_header_value = format!("{}:{}", self.app_id, self.access_token);
        let curl_command = format!(
            "curl -H \"Authorization: {}\" \"{}\"",
            auth_header_value, url
        );

        println!("Execute curl command:\n---\n{}\n---", curl_command);

        let response = self
            .http_client
            .get(&url)
            .header("Authorization", auth_header_value)
            .send()
            .await?;

        // First check if API returned a non-success status code
        if !response.status().is_success() {
            return Err(FyersError::Network(
                response.error_for_status().unwrap_err(),
            ));
        }

        let response_text = response.text().await?;

        println!("Raw response from /history:\n---\n{}\n---", response_text);

        let history_response: HistoryResponse = match serde_json::from_str(&response_text) {
            Ok(resp) => resp,
            Err(e) => {
                eprintln!("Error parsing response from /history: {}", e);
                return Err(FyersError::Parse(e));
            }
        };

        if history_response.s != "ok" {
            return Err(FyersError::ApiError {
                s: history_response.s,
                code: history_response.code.unwrap_or(0),
                message: history_response
                    .message
                    .unwrap_or("Unknown error".to_string()),
            });
        }

        Ok(history_response)
    }

    /// Full market quotes for one or more symbols provided by the user.
    ///
    /// # Arguments
    /// * `symbols` - Symbols for which data is to be fetched (e.g. "NSE:SBIN-EQ", "NSE:RELIANCE-EQ,NSE:SBIN-EQ")
    /// [API Docs](https://myapi.fyers.in/docsv3#tag/Data-Api/paths/~1DataApi/get)
    pub async fn get_market_quotes(&self, symbols: &str) -> Result<QuoteResponse, FyersError> {
        let url = format!("{}/quotes?symbols={}", DATA_API_BASE_URL, symbols);
        let auth_header_value = format!("{}:{}", self.app_id, self.access_token);
        let curl_command = format!(
            "curl -H \"Authorization: {}\" \"{}\"",
            auth_header_value, url
        );

        println!("Execute curl command:\n---\n{}\n---", curl_command);

        let response = self
            .http_client
            .get(&url)
            .header("Authorization", auth_header_value)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(FyersError::Network(
                response.error_for_status().unwrap_err(),
            ));
        }

        let response_text = response.text().await?;
        println!("Raw response from /quotes:\n---\n{}\n---", response_text);

        let quote_response: QuoteResponse = match serde_json::from_str(&response_text) {
            Ok(resp) => resp,
            Err(e) => {
                eprintln!("Error parsing response from /quotes: {}", e);
                return Err(FyersError::Parse(e));
            }
        };

        if quote_response.s != "ok" {
            return Err(FyersError::ApiError {
                s: quote_response.s,
                code: 0,
                message: "Error fetching quotes".to_string(),
            });
        }

        Ok(quote_response)
    }

    /// Market Depth for one symbol provided by the user.
    ///
    /// # Arguments
    /// * `symbol` - Symbol for which data is to be fetched (e.g. "NSE:SBIN-EQ")
    /// * `ohlcv_flag` = Set the ohlcv_flag to 1 to get open, high, low, closing and volume
    /// quantity
    ///
    /// [API Docs](https://myapi.fyers.in/docsv3#tag/Data-Api/paths/~1DataApi/put)
    pub async fn get_market_depth(&self, symbol: &str, ohlcv_flag: &str) -> Result<MarketDepthResponse, FyersError> {
        let url = format!("{}/depth?symbol={}&ohlcv_flag={}", DATA_API_BASE_URL, symbol, ohlcv_flag);
        let auth_header_value = format!("{}:{}", self.app_id, self.access_token);
        let curl_command = format!(
            "curl -H \"Authorization: {}\" \"{}\"",
            auth_header_value, url
        );

        println!("Execute curl command:\n---\n{}\n---", curl_command);

        let response = self
            .http_client
            .get(&url)
            .header("Authorization", auth_header_value)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(FyersError::Network(
                response.error_for_status().unwrap_err(),
            ));
        }

        let response_text = response.text().await?;
        println!("Raw response from /depth:\n---\n{}\n---", response_text);

        let market_depth_response: MarketDepthResponse = match serde_json::from_str(&response_text) {
            Ok(resp) => resp,
            Err(e) => {
                eprintln!("Error parsing response from /depth: {}", e);
                return Err(FyersError::Parse(e));
            }
        };

        if market_depth_response.s != "ok" {
            return Err(FyersError::ApiError {
                s: market_depth_response.s,
                code: 0,
                message: "Error fetching market depth".to_string(),
            });
        }

        Ok(market_depth_response)
    }

    /// Option Chain for a given symbol.
    ///
    /// # Arguments
    /// * `symbol` - Symbol for which data is to be fetched (Mandatory)
    /// * `strikecount` - Options strike count for symbol(MAX = 50)
    /// * `timestamp` - Options chain data at timestamp
    ///
    /// [API Docs](https://myapi.fyers.in/docsv3#tag/Data-Api/paths/~1DataApi/delete)
    pub async fn get_option_chain(&self, symbol: &str, strikecount: Option<&str>, timestamp: Option<&str>) -> Result<OptionChainResponse, FyersError> {
        let mut url = format!("{}/options-chain-v3?symbol={}", DATA_API_BASE_URL, symbol);
        if let Some(sc) = strikecount {
            url.push_str(&format!("&strikecount={}", sc));
        }
        if let Some(ts) = timestamp {
            url.push_str(&format!("&timestamp={}", ts));
        }
        let auth_header_value = format!("{}:{}", self.app_id, self.access_token);
        let curl_command = format!(
            "curl -H \"Authorization: {}\" \"{}\"",
            auth_header_value, url
        );

        println!("Execute curl command:\n---\n{}\n---", curl_command);

        let response = self
            .http_client
            .get(&url)
            .header("Authorization", auth_header_value)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(FyersError::Network(
                response.error_for_status().unwrap_err(),
            ));
        }

        let response_text = response.text().await?;
        println!("Raw response from /option_chain:\n---\n{}\n---", response_text);

        let option_chain_response: OptionChainResponse = match serde_json::from_str(&response_text) {
            Ok(resp) => resp,
            Err(e) => {
                eprintln!("Error parsing response from /option_chain: {}", e);
                return Err(FyersError::Parse(e));
            }
        };

        if option_chain_response.s != "ok" {
            return Err(FyersError::ApiError {
                s: option_chain_response.s,
                code: 0,
                message: "Error fetching option chain".to_string(),
            });
        }

        Ok(option_chain_response)
    }
}
