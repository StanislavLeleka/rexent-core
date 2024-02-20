use api_error::error::ApiError;
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::json;

#[derive(Clone)]
pub struct HttpClient {
    pub client: Client,
}

impl HttpClient {
    pub fn new() -> Self {
        let client = reqwest::Client::new();
        log::info!("New HTTP client instance created.");
        HttpClient { client }
    }

    pub async fn get<T: DeserializeOwned>(&self, url: &String) -> Result<T, ApiError> {
        log::info!("Sending GET request to url: {}", url);
        let response = self.client.get(url).send().await;
        self.process_response(response).await
    }

    pub async fn get_with_headers<T: DeserializeOwned>(
        &self,
        url: &String,
        headers: HeaderMap<HeaderValue>,
    ) -> Result<T, ApiError> {
        log::info!(
            "Sending GET request with headers [{:?}] to url: {}",
            headers,
            url
        );
        let response = self.client.get(url).headers(headers).send().await;
        self.process_response(response).await
    }

    pub async fn post<Req: Serialize, Res: DeserializeOwned>(
        &self,
        url: &String,
        data: Req,
    ) -> Result<Res, ApiError> {
        log::info!("Sending POST request to url: {}", url);
        let response = self.client.post(url).json(&data).send().await;
        self.process_response(response).await
    }

    async fn process_response<T: DeserializeOwned>(
        &self,
        response: Result<reqwest::Response, reqwest::Error>,
    ) -> Result<T, ApiError> {
        match response {
            Ok(response) => {
                log::info!("Received response with status: {}", response.status());
                match response.status() {
                    reqwest::StatusCode::OK => {
                        match response.json::<T>().await {
                            Ok(parsed) => {
                                log::info!("Successfully parsed response.");
                                Ok(parsed)
                            }
                            Err(_) => {
                                log::error!("Failed to parse response. Response didn't match expected shape.");
                                Err(ApiError::InternalServerError(
                                    json!({"error": "Response didn't match expected shape"}),
                                ))
                            }
                        }
                    }
                    reqwest::StatusCode::UNAUTHORIZED => {
                        log::warn!("Unauthorized request.");
                        Err(ApiError::Unauthorized(json!({"error": "Unauthorized"})))
                    }
                    _ => {
                        log::error!("Unexpected status code: {}", response.status());
                        Err(ApiError::InternalServerError(json!({
                            "error": format!("Unexpected status code: {}", response.status())
                        })))
                    }
                }
            }
            Err(_) => {
                log::error!("Failed to send request or receive response.");
                Err(ApiError::InternalServerError(
                    json!({"error": "Internal Server Error"}),
                ))
            }
        }
    }
}
