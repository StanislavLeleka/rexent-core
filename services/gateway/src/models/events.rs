use crawler::models::showtime::{Show, Showtime, Theater};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Showtimes {
    pub showtimes: Vec<Showtime>,
}

#[derive(Serialize, Deserialize)]
pub struct GetShowtimesRequest {
    pub movie_id: i32,
    pub country: String,
    pub city: String,
}

impl From<GetShowtimesRequest> for proto::services::events::GetShowtimesRequest {
    fn from(value: GetShowtimesRequest) -> Self {
        proto::services::events::GetShowtimesRequest {
            movie_id: value.movie_id,
            country: value.country,
            city: value.city,
        }
    }
}

impl From<proto::services::events::ShowtimesResponse> for Showtimes {
    fn from(value: proto::services::events::ShowtimesResponse) -> Self {
        let showtimes = value
            .showtimes
            .into_iter()
            .map(|showtime| from_grpc(&showtime))
            .collect();
        Showtimes { showtimes }
    }
}

fn from_grpc(showtime: &proto::services::events::Showtime) -> Showtime {
    let theaters = showtime
        .theaters
        .iter()
        .map(|theater| Theater {
            name: theater.name.clone(),
            link: theater.link.clone(),
            distance: "".to_string(),
            address: theater.address.clone(),
            showing: theater
                .showing
                .iter()
                .map(|show| Show {
                    time: show.time.clone(),
                    r#type: show.r#type.clone(),
                })
                .collect(),
        })
        .collect();
    Showtime {
        day: showtime.day.clone(),
        date: showtime.date.clone(),
        theaters,
    }
}
