use diesel::{
    r2d2::{ConnectionManager, Pool, PoolError},
    PgConnection,
};
use std::env;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn connect() -> Result<DbPool, PoolError> {
    const PG_CONN_STR: &str = "PG_CONN_STR";

    let conn_uri = env::var(PG_CONN_STR).expect(&format!("${} is not set", PG_CONN_STR));
    let manager = ConnectionManager::<PgConnection>::new(conn_uri);

    match Pool::builder().build(manager) {
        Ok(pool) => {
            log::info!("Successfully connected to PostgreSQL");
            Ok(pool)
        }
        Err(e) => {
            log::error!("Failed to connect to PostgreSQL: {}", e);
            Err(e)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::connect;

    #[test]
    fn test_postgres_connection() {
        let pool = connect();
        assert!(pool.is_ok(), "Failed to connect to PostgreSQL");
    }
}
