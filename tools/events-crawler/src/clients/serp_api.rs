use crate::models::showtime::Showtime;
use serpapi_search_rust::serp_api_search::SerpApiSearch;
use std::collections::HashMap;

pub struct SerpApiClient {
    params: HashMap<String, String>,
}

impl SerpApiClient {
    pub fn new() -> Self {
        Self {
            params: HashMap::<String, String>::new(),
        }
    }

    pub fn set_param(&mut self, key: &str, val: String) -> &mut Self {
        self.params.insert(key.to_string(), val);
        self
    }

    pub fn set_engine(&mut self, engine: String) -> &mut Self {
        self.params.insert("engine".to_string(), engine);
        self
    }

    pub async fn showtimes(
        &self,
        api_key: String,
    ) -> Result<Vec<Showtime>, Box<dyn std::error::Error>> {
        log::info!("Initialize the search engine");
        let search = SerpApiSearch::google(self.params.clone(), api_key);

        log::info!("Waiting...");
        let results = search.json().await?;
        let showtimes = results["showtimes"].as_array();

        match showtimes {
            Some(s) => {
                log::info!("Results received");
                log::info!(" - number of showtimes results: {}", s.len());

                let showtimes = s.iter().map(|v| v.into()).collect::<Vec<Showtime>>();
                Ok(showtimes)
            }
            None => Err(format!(
                "No showtimes found for query: {}",
                self.params.get("q").unwrap_or(&"".to_string())
            )
            .into()),
        }
    }
}
