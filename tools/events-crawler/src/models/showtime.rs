use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Showtime {
    pub day: String,
    pub date: String,
    pub theaters: Vec<Theater>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Theater {
    pub name: String,
    pub link: String,
    pub distance: String,
    pub address: String,
    pub showing: Vec<Show>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Show {
    pub time: Vec<String>,
    pub r#type: Option<String>,
}

impl From<&Value> for Showtime {
    fn from(value: &Value) -> Self {
        let result: Result<Showtime, serde_json::Error> = serde_json::from_value(value.clone());
        match result {
            Ok(showtime) => showtime,
            Err(e) => panic!("error during parsing showtime response: {}", e),
        }
    }
}
