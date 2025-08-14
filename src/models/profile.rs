use serde::Deserialize;

// This represents the user profile data returned by the FYERS API
#[derive(Debug, Deserialize)]
pub struct Profile {
    #[serde(rename = "fy_id")]
    pub fy_id: String,
    pub name: String,
    pub email: String,
    #[serde(rename = "mobile")]
    pub phone: String,

    // Here I'll put other stuff from the api documentation
}
