use std::env;

pub struct UrlBuilder {
    base_url: String,
}

impl UrlBuilder {
    pub fn new() -> Self {
        let base_url = env::var("THEMOVIEDB_API_URL").expect("$THEMOVIEDB_API_URL is not set");
        Self { base_url }
    }

    pub fn now_playing(&self, region: String, language: String, page: i8) -> String {
        format!(
            "{}/3/movie/now_playing?language={}&page={}&region={}",
            self.base_url,
            language,
            page.to_string(),
            region
        )
    }

    pub fn movie_details(&self, movie_id: i32) -> String {
        format!("{}/3/movie/{}", self.base_url, movie_id)
    }

    pub fn movie_credits(&self, movie_id: i32) -> String {
        format!("{}/3/movie/{}/credits", self.base_url, movie_id)
    }

    pub fn movie_keywords(&self, movie_id: i32) -> String {
        format!("{}/3/movie/{}/keywords", self.base_url, movie_id)
    }
}
