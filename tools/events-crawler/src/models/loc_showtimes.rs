use super::{location::Location, showtime::Showtime};
use serde::{Deserialize, Serialize};
use tmdb_client::models::movie::Movie;

#[derive(Serialize, Deserialize, Debug)]
pub struct LocationShowtimes {
    pub loc: Location,
    pub movie: Movie,
    pub showtimes: Vec<Showtime>,
}
