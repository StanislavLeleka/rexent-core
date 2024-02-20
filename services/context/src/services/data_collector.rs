use chrono::{NaiveDateTime, Utc};
use chrono_tz::Tz;
use proto::{
    clients::location::LocationServiceGrpcClient,
    services::{
        context::{Activity, ActivityType},
        location::{GetLocationRequest, LocationResponse},
    },
};
use std::collections::HashMap;
use weatherapi::{client::WeatherapiClient, models::weather::WeatherData};

pub async fn get_location(
    loc_client: &LocationServiceGrpcClient,
    user_id: String,
) -> Result<LocationResponse, Box<dyn std::error::Error>> {
    let mut cloned_client = loc_client.clone();
    let user_location = cloned_client
        .get_location(GetLocationRequest { user_id })
        .await?;
    Ok(user_location)
}

pub async fn get_weather(
    lat: f32,
    lng: f32,
    weather_client: &WeatherapiClient,
) -> Result<WeatherData, Box<dyn std::error::Error>> {
    let weather = weather_client.get_current(lat, lng).await?;
    Ok(weather.into())
}

pub async fn get_user_activity() -> Result<Activity, Box<dyn std::error::Error>> {
    let mut metadata = HashMap::<String, String>::new();
    metadata.insert("key1".to_string(), "value1".to_string());

    Ok(Activity {
        activity_type: ActivityType::Walking.into(),
        duration: 3,
        metadata,
    })
}

pub fn get_local_time(tz: String) -> NaiveDateTime {
    let timezone: Tz = tz.parse().unwrap();
    let utc = Utc::now();
    let local_time = utc.with_timezone(&timezone);

    local_time.naive_local()
}
