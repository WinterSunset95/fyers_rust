use crate::error::FyersError;
use reqwest::Client;
use serde::de::Error;
use log::{debug, info, error};

#[derive(Debug, Clone)]
pub struct DataApi {
    http_client: Client,
    base_url: String,
    app_id: String,
    access_token: String,
}

impl DataApi {
    pub fn new(base_url: String, app_id: String, access_token: String) -> Self {
        Self {
            http_client: Client::new(),
            base_url,
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
    pub async fn get_historical_data(
        &self,
        symbol: &str,
        resolution: &str,
        date_format: &str,
        range_from: &str,
        range_to: &str,
        cont_flag: &str,
        oi_flag: &str
    ) {
        unimplemented!();
    }
}
