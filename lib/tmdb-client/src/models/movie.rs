use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Movie {
    pub adult: bool,
    pub backdrop_path: Option<String>,
    pub genre_ids: Vec<i32>,
    pub id: i32,
    pub original_language: Option<String>,
    pub original_title: Option<String>,
    pub overview: Option<String>,
    pub popularity: f32,
    pub poster_path: Option<String>,
    pub release_date: Option<String>,
    pub title: Option<String>,
    pub video: bool,
    pub vote_average: f32,
    pub vote_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovieDetails {
    pub adult: Option<bool>,
    pub backdrop_path: Option<String>,
    pub belongs_to_collection: Option<Collection>,
    pub budget: Option<i32>,
    pub genres: Option<Vec<Genre>>,
    pub homepage: Option<String>,
    pub id: i32,
    pub imdb_id: Option<String>,
    pub original_language: Option<String>,
    pub original_title: Option<String>,
    pub overview: Option<String>,
    pub popularity: Option<f64>,
    pub poster_path: Option<String>,
    pub production_companies: Option<Vec<ProductionCompany>>,
    pub production_countries: Option<Vec<ProductionCountry>>,
    pub release_date: Option<String>,
    pub revenue: Option<i64>,
    pub runtime: Option<i32>,
    pub spoken_languages: Option<Vec<SpokenLanguage>>,
    pub status: Option<String>,
    pub tagline: Option<String>,
    pub title: Option<String>,
    pub video: Option<bool>,
    pub vote_average: Option<f64>,
    pub vote_count: Option<i32>,
    pub cast: Option<Vec<CastMember>>,
    pub crew: Option<Vec<CrewMember>>,
    pub keywords: Option<Vec<Keyword>>,
}

impl MovieDetails {
    pub fn set_credits(&mut self, credits: MovieCredits) {
        self.cast = Some(credits.cast);
        self.crew = Some(credits.crew);
    }

    pub fn set_keywords(&mut self, keywords: MovieKeywords) {
        self.keywords = Some(keywords.keywords);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Collection {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Genre {
    pub id: Option<i32>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionCompany {
    pub id: Option<i32>,
    pub logo_path: Option<String>,
    pub name: Option<String>,
    pub origin_country: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionCountry {
    pub iso_3166_1: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpokenLanguage {
    pub english_name: Option<String>,
    pub iso_639_1: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CastMember {
    pub adult: Option<bool>,
    pub gender: Option<i32>,
    pub id: i32,
    pub known_for_department: Option<String>,
    pub name: String,
    pub original_name: Option<String>,
    pub popularity: Option<f64>,
    pub profile_path: Option<String>,
    pub cast_id: Option<i32>,
    pub character: Option<String>,
    pub credit_id: Option<String>,
    pub order: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrewMember {
    pub adult: Option<bool>,
    pub gender: Option<i32>,
    pub id: Option<i32>,
    pub known_for_department: Option<String>,
    pub name: String,
    pub original_name: Option<String>,
    pub popularity: Option<f64>,
    pub profile_path: Option<String>,
    pub credit_id: Option<String>,
    pub department: Option<String>,
    pub job: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovieCredits {
    pub id: i32,
    pub cast: Vec<CastMember>,
    pub crew: Vec<CrewMember>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Keyword {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovieKeywords {
    pub id: i32,
    pub keywords: Vec<Keyword>,
}
