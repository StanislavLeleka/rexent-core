use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Movie {
    pub(crate) adult: bool,
    pub(crate) belongs_to_collection: Option<Collection>,
    pub(crate) budget: Option<u64>,
    pub(crate) genres: Vec<Genre>,
    pub(crate) homepage: Option<String>,
    pub(crate) id: u32,
    pub(crate) imdb_id: String,
    pub(crate) original_language: String,
    pub(crate) original_title: String,
    pub(crate) overview: String,
    pub(crate) poster_path: String,
    pub(crate) production_companies: Vec<ProductionCompany>,
    pub(crate) production_countries: Vec<ProductionCountry>,
    pub(crate) release_date: String,
    pub(crate) revenue: u64,
    pub(crate) runtime: u32,
    pub(crate) spoken_languages: Vec<SpokenLanguage>,
    pub(crate) tagline: Option<String>,
    pub(crate) title: String,
    pub(crate) vote_average: f32,
    pub(crate) vote_count: u32,
    pub(crate) keywords: Vec<Keyword>,
    pub(crate) cast: Vec<Cast>,
    pub(crate) crew: Vec<Crew>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Collection {
    pub(crate) id: u32,
    pub(crate) name: String,
    pub(crate) poster_path: Option<String>,
    pub(crate) backdrop_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Genre {
    pub(crate) id: u32,
    pub(crate) name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct ProductionCompany {
    pub(crate) name: String,
    pub(crate) id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductionCountry {
    pub(crate) iso_3166_1: String,
    pub(crate) name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct SpokenLanguage {
    pub(crate) iso_639_1: String,
    pub(crate) name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Keyword {
    pub(crate) id: u32,
    pub(crate) name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Cast {
    pub(crate) cast_id: u32,
    pub(crate) character: String,
    pub(crate) credit_id: String,
    pub(crate) gender: u16,
    pub(crate) id: u32,
    pub(crate) name: String,
    pub(crate) order: u32,
    pub(crate) profile_path: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Crew {
    pub(crate) credit_id: String,
    pub(crate) department: String,
    pub(crate) gender: u16,
    pub(crate) id: u32,
    pub(crate) job: String,
    pub(crate) name: String,
    pub(crate) profile_path: Option<String>,
}
