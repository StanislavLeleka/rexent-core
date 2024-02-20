use super::models::Movie;

impl From<Movie> for proto::services::movie::Movie {
    fn from(value: Movie) -> Self {
        proto::services::movie::Movie {
            id: value.id as i32,
            imdb_id: value.imdb_id,
            original_language: value.original_language,
            original_title: value.original_title,
            overview: value.overview,
            poster_path: value.poster_path,
            release_date: value.release_date,
            title: value.title,
            vote_average: value.vote_average,
            genres: value
                .genres
                .iter()
                .map(|g| proto::services::movie::Genre {
                    id: g.id as i32,
                    name: g.name.clone(),
                })
                .collect(),
        }
    }
}
