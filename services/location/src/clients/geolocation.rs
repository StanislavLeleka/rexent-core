use super::geocode::{Geocode, GeocodeResult};
use api_error::error::ApiError;
use http_client::http_client::HttpClient;
use std::env;

#[derive(Clone)]
pub struct Geolocation {
    http_client: HttpClient,
}

impl Geolocation {
    pub fn new(http_client: HttpClient) -> Self {
        Self { http_client }
    }

    pub async fn get_geocode(&self, lat: f32, lng: f32) -> Result<Geocode, ApiError> {
        let geocode_result = self.get_geocode_result(lat, lng).await?;
        let geocode = geocode_result
            .results
            .first()
            .expect("Geodata result is empty.");
        Ok(geocode.clone())
    }

    async fn get_geocode_result(&self, lat: f32, lng: f32) -> Result<GeocodeResult, ApiError> {
        let api_url =
            env::var("GOOGLE_API_GEOCODE_URL").expect("$GOOGLE_API_GEOCODE_URL is not set.");
        let api_key = env::var("GOOGLE_API_KEY").expect("$GOOGLE_API_KEY is not set.");
        let request_url = format!(
            "{}?latlng={},{}&key={}&result_type=locality",
            api_url,
            lat.to_string(),
            lng.to_string(),
            api_key
        );
        let geocode_result = self.http_client.get::<GeocodeResult>(&request_url).await?;
        Ok(geocode_result)
    }
}
