use crate::{error::FyersError, models::dataapi::HistoryResponse};
use reqwest::Client;
use serde::de::Error;
use log::{debug, info, error};

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
            access_token
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
    /// [API Docs](https://myapi.fyers.in/docsv3#tag/Data-Api/paths/~1DataApi/get)
    pub async fn get_historical_data(
        &self,
        symbol: &str,
        resolution: &str,
        date_format: &str,
        range_from: &str,
        range_to: &str,
        cont_flag: &str,
        oi_flag: &str
    ) -> Result<HistoryResponse, FyersError> {
        let url = format!("{}/history?symbol={}&resolution={}&date_format={}&range_from={}&range_to={}&cont_flag={}&oi_Flag={}",
            DATA_API_BASE_URL, symbol, resolution, date_format, range_from, range_to, cont_flag, oi_flag
            );
        let auth_header_value = format!("{}:{}", self.app_id, self.access_token);

        let response = self
            .http_client
            .get(&url)
            .header("Authorization", auth_header_value)
            .send()
            .await?;

        // First check if API returned a non-success status code
        if !response.status().is_success() {
            return Err(FyersError::Network(response.error_for_status().unwrap_err()));
        }

        let response_text = response.text().await?;

        debug!("Raw response from /history:\n---\n{}\n---", response_text);

        let history_response: HistoryResponse = serde_json::from_str(&response_text).unwrap();

        if history_response.s == "ok" {
            Ok(history_response)
        } else {
            Err(FyersError::ApiError {
                code: 1,
                message: "An unknown error occured".to_string()
            })
        }
    }
}
