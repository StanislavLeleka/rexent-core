use super::location::LocationResponse;
use chrono::{NaiveDateTime, TimeZone, Utc};
use chrono_tz::Tz;
use prost_types::Timestamp;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Deserialize)]
pub struct NewContextRequest {
    pub mood: Mood,
    pub tz: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Context {
    pub user_id: String,
    pub location: LocationResponse,
    pub activity: Activity,
    pub time_of_day: TimeOfDay,
    pub mood: Mood,
    pub weather: Weather,
    pub nearby_friends: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Activity {
    pub activity_type: ActivityType,
    pub duration: i32,
    pub metadata: HashMap<String, String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Mood {
    pub emotion: Emotion,
    pub intensity: i32,
    pub comment: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Weather {
    pub temp_c: f32,
    pub temp_f: f32,
    pub is_day: i32,
    pub condition: Condition,
    pub wind_mph: f32,
    pub wind_kph: f32,
    pub wind_degree: i32,
    pub wind_dir: String,
    pub humidity: i32,
    pub cloud: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Condition {
    pub text: String,
    pub icon: String,
    pub code: i32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TimeOfDay {
    pub time_of_day: NaiveDateTime,
    pub tz: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ActivityType {
    RESTING,
    WALKING,
    RUNNING,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Emotion {
    Happy,
    Sad,
    Excited,
    Stressed,
    Relaxed,
    Anxious,
    Bored,
    Energetic,
}

impl ActivityType {
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(ActivityType::RESTING),
            1 => Some(ActivityType::WALKING),
            2 => Some(ActivityType::RUNNING),
            _ => None,
        }
    }
}

impl Emotion {
    pub fn from_i32(value: i32) -> Option<Self> {
        match value {
            0 => Some(Emotion::Happy),
            1 => Some(Emotion::Sad),
            2 => Some(Emotion::Excited),
            3 => Some(Emotion::Stressed),
            4 => Some(Emotion::Relaxed),
            5 => Some(Emotion::Anxious),
            6 => Some(Emotion::Bored),
            7 => Some(Emotion::Energetic),
            _ => None,
        }
    }
}

impl NewContextRequest {
    pub fn to_grpc(&self, user_id: String) -> proto::services::context::NewContextRequest {
        proto::services::context::NewContextRequest {
            user_id,
            mood: Some(proto::services::context::Mood {
                emotion: self.mood.emotion.clone() as i32,
                intensity: self.mood.intensity,
                comment: self.mood.comment.clone(),
            }),
            tz: self.tz.clone(),
        }
    }
}

impl From<proto::services::context::ContextResponse> for Context {
    fn from(value: proto::services::context::ContextResponse) -> Self {
        let context = value.context.unwrap();
        let location = context.location.unwrap();
        let activity = context.activity.unwrap();
        let time_of_day = context.time_of_day.unwrap();
        let mood = context.mood.unwrap();
        let weather = context.weather.unwrap();
        let weather_cond = weather.condition.unwrap();

        Context {
            user_id: context.user_id.clone(),
            location: LocationResponse {
                lat: location.lat,
                lng: location.lng,
                country_code: location.country_code.unwrap_or_default(),
                country_name: location.country_name.unwrap_or_default(),
                city_name: location.city_name.unwrap_or_default(),
                city_code: location.city_code.unwrap_or_default(),
                formatted_address: location.formatted_address.unwrap_or_default(),
                user_id: context.user_id,
            },
            activity: Activity {
                activity_type: ActivityType::from_i32(activity.activity_type).unwrap(),
                duration: activity.duration,
                metadata: activity.metadata,
            },
            time_of_day: TimeOfDay {
                time_of_day: convert_timestamp_to_naive_date_time(
                    time_of_day.time_of_day.unwrap(),
                    time_of_day.tz.clone(),
                ),
                tz: time_of_day.tz,
            },
            mood: Mood {
                emotion: Emotion::from_i32(mood.emotion).unwrap(),
                intensity: mood.intensity,
                comment: mood.comment,
            },
            weather: Weather {
                temp_c: weather.temp_c,
                temp_f: weather.temp_f,
                is_day: weather.is_day as i32,
                condition: Condition {
                    text: weather_cond.text,
                    icon: weather_cond.icon,
                    code: weather_cond.code,
                },
                wind_mph: weather.wind_mph,
                wind_kph: weather.wind_kph,
                wind_degree: weather.wind_degree,
                wind_dir: weather.wind_dir,
                humidity: weather.humidity,
                cloud: weather.cloud,
            },
            nearby_friends: context.nearby_friends,
        }
    }
}

fn convert_timestamp_to_naive_date_time(timestamp: Timestamp, tz: String) -> NaiveDateTime {
    let seconds = timestamp.seconds;
    let nanoseconds = timestamp.nanos;

    let dt = Utc.timestamp_opt(seconds, nanoseconds as u32).unwrap();
    let timezone: Tz = tz.parse().unwrap();
    let dt = dt.with_timezone(&timezone);

    dt.naive_utc()
}
