use crawler::models::showtime::{Show, Showtime, Theater};

pub fn to_grpc(showtime: Showtime) -> proto::services::events::Showtime {
    proto::services::events::Showtime {
        day: showtime.day,
        date: showtime.date,
        theaters: showtime
            .theaters
            .iter()
            .map(|t| to_grpc_theater(t))
            .collect(),
    }
}

fn to_grpc_theater(theater: &Theater) -> proto::services::events::Theater {
    proto::services::events::Theater {
        name: theater.name.clone(),
        link: theater.link.clone(),
        address: theater.address.clone(),
        showing: theater.showing.iter().map(|s| to_grpc_show(s)).collect(),
    }
}

fn to_grpc_show(show: &Show) -> proto::services::events::Show {
    proto::services::events::Show {
        time: show.time.clone(),
        r#type: show.r#type.clone(),
    }
}
