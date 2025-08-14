use crate::error::FyersError;
use crate::models::profile::Profile;
use reqwest::Client;

#[derive(Debug, Clone)]
pub struct FyersClient {
    http_client: Client,
    app_id_with_version: String,
    access_token: String,
}

impl FyersClient {
    pub fn new(app_id: String, access_token: String) -> Self {
        let app_id_with_version = format!("{}:{}", app_id, "v3");
        Self {
            http_client: Client::new(),
            app_id_with_version,
            access_token
        }
    }

    pub async fn get_profile(&self) -> Result<Profile, FyersError> {
        // TODO: Implement the actual HTTP GET request
        // 1. Build the URL
        // 2. Use `self.http_client.get(...)`
        // 3. Add the required `Authorization` header using `self.access_token` and `self.app_id_with_version`
        // 4. Send the request with '.await?'
        // 5. Deserialize the JSON response with `.json::<Profile>().await?`
        unimplemented!("get profile is not implemented yet");
    }
}
