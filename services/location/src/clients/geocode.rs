use std::collections::HashSet;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Geocode {
    pub address_components: Vec<LocationInfo>,
    pub formatted_address: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LocationInfo {
    pub long_name: String,
    pub short_name: String,
    pub types: HashSet<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeocodeResult {
    pub results: Vec<Geocode>,
}

pub fn get_address(geocode: &Geocode, address_type: &str) -> LocationInfo {
    let locations = geocode
        .address_components
        .iter()
        .filter(|g| g.types.contains(address_type))
        .cloned()
        .collect::<Vec<LocationInfo>>();
    locations
        .first()
        .cloned()
        .expect("No country found in geodata.")
}
