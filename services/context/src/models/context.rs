use std::collections::HashMap;

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Context {
    pub user_id: String,
    pub location: Location,
    pub activity: Activity,
    pub time_of_day: TimeOfDay,
    pub mood: Mood,
    pub weather: Weather,
    pub nearby_friends: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Location {
    pub lat: f32,
    pub lng: f32,
    pub country_code: String,
    pub city_code: String,
    pub formatted_address: String,
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
