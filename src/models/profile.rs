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
// [API Docs](https://myapi.fyers.in/docsv3#tag/User/paths/~1User/post)
#[derive(Debug, Deserialize)]
pub struct Profile {
    pub name: String,
    pub display_name: Option<String>,
    pub fy_id: String,
    pub image: Option<String>,
    pub email_id: String,
    #[serde(rename = "PAN")]
    pub pan: Option<String>,
    pub pin_change_date: Option<String>,
    pub pwd_change_date: Option<String>,
    pub mobile_number: Option<String>,
    pub totp: bool,
    pub pwd_to_expire: i64,
    pub ddpi_enabled: bool,
    pub mtf_enabled: bool,
}
