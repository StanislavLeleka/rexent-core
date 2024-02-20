use crate::models::models::{PredictRequest, PredictResponse};
use api_error::error::ApiError;
use http_client::http_client::HttpClient;

pub struct MovieRecoClient {
    base_url: String,
    client: HttpClient,
}

impl MovieRecoClient {
    pub fn new() -> Self {
        let base_url = std::env::var("MOVIE_RECO_URL").expect("$MOVIE_RECO_URL not set");
        Self {
            base_url,
            client: HttpClient::new(),
        }
    }

    pub async fn predict(&self, request: PredictRequest) -> Result<PredictResponse, ApiError> {
        log::info!("Sending predict request: {:#?}", request);

        let url = format!("{}/predict", self.base_url);
        let response = self.client.post(&url, request).await?;

        log::info!("Received predict response: {:#?}", response);

        Ok(response)
    }

    pub async fn retrain(&self) -> Result<(), ApiError> {
        log::info!("Sending retrain request");

        let url = format!("{}/retrain", self.base_url);
        self.client.post(&url, ()).await?;

        log::info!("Received retrain response");

        Ok(())
    }
}
