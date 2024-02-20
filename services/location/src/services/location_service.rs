use crate::{clients::geolocation::Geolocation, DbPool};

pub struct LocationServiceImpl {
    pub(super) db: DbPool,
    pub(super) geolocation: Geolocation,
}

impl LocationServiceImpl {
    pub fn new(db: DbPool, geolocation: Geolocation) -> Self {
        log::info!("Initializing LocationService");
        Self { db, geolocation }
    }
}
