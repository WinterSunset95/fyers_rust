use serde::Deserialize;

// Top level response for the /profile endpoint
#[derive(Debug, Deserialize)]
pub struct ProfileResponse {
    pub s: String,
    pub code: i64,
    pub message: String,
    pub data: Profile,
}

// This represents the user profile data returned by the FYERS API
// Documentation: https://myapi.fyers.in/docsv3#tag/User/paths/~1User/post
#[derive(Debug, Deserialize)]
pub struct Profile {
    #[serde(rename = "fy_id")]
    pub fy_id: String,
    pub name: String,
    pub display_name: String,
    pub image: String,
    pub email_id: String,
    pub pan: String,
    pub pin_change_date: String,
    pub pwd_change_date: String,
    pub mobile_number: String,
    pub totp: bool,
    pub pwd_to_expire: i64,
    pub ddpi_enabled: bool,
    pub mtf_enabled: bool,
}
