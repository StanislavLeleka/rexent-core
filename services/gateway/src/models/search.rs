use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize)]
pub struct SearchQuery {
    pub q: String,
    pub page: i64,
    pub page_size: i64,
}

#[derive(Debug, Serialize)]
pub struct SearchResult {
    pub movies: Vec<Movie>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Movie {
    pub(crate) id: i32,
    pub(crate) imdb_id: String,
    pub(crate) original_language: String,
    pub(crate) original_title: String,
    pub(crate) overview: String,
    pub(crate) poster_path: String,
    pub(crate) release_date: String,
    pub(crate) title: String,
    pub(crate) vote_average: f32,
    pub(crate) genres: Vec<Genre>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Genre {
    pub(crate) id: i32,
    pub(crate) name: String,
}

impl From<SearchQuery> for proto::services::search::SearchMovieRequest {
    fn from(value: SearchQuery) -> Self {
        proto::services::search::SearchMovieRequest {
            query: value.q,
            page: Some(value.page),
            page_size: Some(value.page_size),
        }
    }
}

impl From<proto::services::search::SearchResultResponse> for SearchResult {
    fn from(value: proto::services::search::SearchResultResponse) -> Self {
        SearchResult {
            movies: value
                .movies
                .iter()
                .map(|movie| Movie {
                    id: movie.id,
                    imdb_id: movie.imdb_id.clone(),
                    original_language: movie.original_language.clone(),
                    original_title: movie.original_title.clone(),
                    overview: movie.overview.clone(),
                    poster_path: movie.poster_path.clone(),
                    release_date: movie.release_date.clone(),
                    title: movie.title.clone(),
                    vote_average: movie.vote_average,
                    genres: movie
                        .genres
                        .iter()
                        .map(|genre| Genre {
                            id: genre.id,
                            name: genre.name.clone(),
                        })
                        .collect(),
                })
                .collect(),
        }
    }
}
