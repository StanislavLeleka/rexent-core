use super::context::{Activity, ActivityType, Condition, Context, Location, Weather};
use chrono::NaiveDateTime;
use proto::services::context::ContextResponse;
use weatherapi::models::weather::WeatherData;

impl From<proto::services::location::Location> for Location {
    fn from(value: proto::services::location::Location) -> Self {
        Location {
            lat: value.lat,
            lng: value.lng,
            country_code: value.country_code.unwrap(),
            city_code: value.city_code.unwrap(),
            formatted_address: value.formatted_address.unwrap(),
        }
    }
}

impl From<proto::services::context::Activity> for Activity {
    fn from(value: proto::services::context::Activity) -> Self {
        Activity {
            activity_type: ActivityType::from_i32(value.activity_type).unwrap(),
            duration: value.duration,
            metadata: value.metadata,
        }
    }
}

impl From<WeatherData> for Weather {
    fn from(value: WeatherData) -> Self {
        Self {
            temp_c: value.current.temp_c,
            temp_f: value.current.temp_f,
            is_day: value.current.is_day,
            condition: Condition {
                text: value.current.condition.text,
                icon: value.current.condition.icon,
                code: value.current.condition.code,
            },
            wind_mph: value.current.wind_mph,
            wind_kph: value.current.wind_kph,
            wind_degree: value.current.wind_degree,
            wind_dir: value.current.wind_dir,
            humidity: value.current.humidity,
            cloud: value.current.cloud,
        }
    }
}

impl From<Context> for ContextResponse {
    fn from(value: Context) -> Self {
        ContextResponse {
            context: Some(proto::services::context::Context {
                user_id: value.user_id,
                location: Some(proto::services::location::Location {
                    lat: value.location.lat,
                    lng: value.location.lng,
                    country_code: Some(value.location.country_code),
                    country_name: None,
                    city_name: None,
                    city_code: Some(value.location.city_code),
                    formatted_address: Some(value.location.formatted_address),
                }),
                activity: Some(proto::services::context::Activity {
                    activity_type: value.activity.activity_type as i32,
                    duration: value.activity.duration,
                    metadata: value.activity.metadata,
                }),
                time_of_day: Some(proto::services::context::TimeOfDay {
                    time_of_day: Some(naive_datetime_to_timestamp(value.time_of_day.time_of_day)),
                    tz: value.time_of_day.tz,
                }),
                mood: Some(proto::services::context::Mood {
                    emotion: value.mood.emotion as i32,
                    intensity: value.mood.intensity,
                    comment: value.mood.comment,
                }),
                weather: Some(proto::services::context::Weather {
                    temp_c: value.weather.temp_c,
                    temp_f: value.weather.temp_f,
                    is_day: value.weather.is_day as f32,
                    condition: Some(proto::services::context::Condition {
                        text: value.weather.condition.text,
                        icon: value.weather.condition.icon,
                        code: value.weather.condition.code,
                    }),
                    wind_mph: value.weather.wind_mph,
                    wind_kph: value.weather.wind_kph,
                    wind_degree: value.weather.wind_degree,
                    wind_dir: value.weather.wind_dir,
                    humidity: value.weather.humidity,
                    cloud: value.weather.cloud,
                }),
                nearby_friends: value.nearby_friends,
            }),
        }
    }
}

fn naive_datetime_to_timestamp(dt: NaiveDateTime) -> prost_types::Timestamp {
    let seconds = dt.timestamp();
    let nanos = dt.timestamp_subsec_nanos();
    prost_types::Timestamp {
        seconds,
        nanos: nanos as i32,
    }
}
