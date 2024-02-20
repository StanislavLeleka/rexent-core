use std::env;

pub struct Config {
    pub(crate) access_token: String,
}

impl Config {
    pub fn from_env() -> Result<Self, env::VarError> {
        let access_token = env::var("THEMOVIEDB_API_READ_ACCESS_TOKEN")?;
        Ok(Self { access_token })
    }
}
