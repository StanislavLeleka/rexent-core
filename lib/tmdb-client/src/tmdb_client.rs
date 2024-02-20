use crate::{
    config::Config,
    models::{
        movie::{MovieCredits, MovieDetails, MovieKeywords},
        now_playing::NowPlaying,
    },
    url_builder::UrlBuilder,
};
use api_error::error::ApiError;
use http_client::http_client::HttpClient;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::de::DeserializeOwned;
use std::env;

pub struct TheMovieDbClient {
    url_builder: UrlBuilder,
    http_client: HttpClient,
    auth_headers: HeaderMap,
}

impl TheMovieDbClient {
    pub fn new() -> Result<Self, env::VarError> {
        let config = Config::from_env()?;
        let auth_headers = Self::get_auth_header(config.access_token);

        Ok(Self {
            url_builder: UrlBuilder::new(),
            http_client: HttpClient::new(),
            auth_headers,
        })
    }

    pub async fn get_now_playing(
        &self,
        region: String,
        language: String,
        page: i8,
    ) -> Result<NowPlaying, Box<dyn std::error::Error>> {
        let url = self.url_builder.now_playing(region, language, page);
        self.get(&url).await.map_err(|e| Box::new(e) as _)
    }

    pub async fn get_movie_details(
        &self,
        movie_id: i32,
    ) -> Result<MovieDetails, Box<dyn std::error::Error>> {
        let url = self.url_builder.movie_details(movie_id);
        self.get(&url).await.map_err(|e| Box::new(e) as _)
    }

    pub async fn get_movie_credits(
        &self,
        movie_id: i32,
    ) -> Result<MovieCredits, Box<dyn std::error::Error>> {
        let url = self.url_builder.movie_credits(movie_id);
        self.get(&url).await.map_err(|e| Box::new(e) as _)
    }

    pub async fn get_movie_keywords(
        &self,
        movie_id: i32,
    ) -> Result<MovieKeywords, Box<dyn std::error::Error>> {
        let url = self.url_builder.movie_keywords(movie_id);
        let keywords = self.get::<MovieKeywords>(&url).await?;
        Ok(keywords)
    }

    async fn get<T: DeserializeOwned>(&self, url: &String) -> Result<T, ApiError> {
        match self
            .http_client
            .get_with_headers::<T>(url, self.auth_headers.clone())
            .await
        {
            Ok(result) => Ok(result),
            Err(error) => {
                log::error!("Error requesting url [{}]: {:?}", url, error);
                Err(error)
            }
        }
    }

    fn get_auth_header(access_token: String) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(
            reqwest::header::AUTHORIZATION,
            HeaderValue::from_str(&*format!("Bearer {}", access_token)).expect("Invalid token"),
        );
        headers
    }
}
