use std::env;

use http_client::http_client::HttpClient;

use crate::models::weather::WeatherData;

#[derive(Clone)]
pub struct WeatherapiClient {
    http_client: HttpClient,
    base_url: String,
    key: String,
}

const WEATHERAPI_URL: &str = "WEATHERAPI_URL";
const WEATHERAPI_KEY: &str = "WEATHERAPI_KEY";

impl WeatherapiClient {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let http_client = HttpClient::new();
        let base_url =
            env::var(WEATHERAPI_URL).map_err(|_| format!("${} is not set", WEATHERAPI_URL))?;
        let key =
            env::var(WEATHERAPI_KEY).map_err(|_| format!("${} is not set", WEATHERAPI_KEY))?;

        Ok(Self {
            http_client,
            base_url,
            key,
        })
    }

    pub async fn get_current(
        &self,
        lat: f32,
        lng: f32,
    ) -> Result<WeatherData, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/v1/current.json?key={}&q={},{}",
            self.base_url, self.key, lat, lng
        );
        let weather_data = self.http_client.get::<WeatherData>(&url).await?;

        Ok(weather_data)
    }
}
