use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use tonic::Status;

use crate::DbPool;

type DbConnection = PooledConnection<ConnectionManager<PgConnection>>;

pub fn get_db_conn(db: DbPool) -> Result<DbConnection, Status> {
    log::debug!("Getting DB connection");

    let conn = db.get().map_err(|err| {
        log::error!("Error connecting to database: {:?}", err);
        Status::internal("Failed to connect to database")
    })?;

    Ok(conn)
}
