use mongodb::error::Error as MongoError;
use mongodb::{
    bson::doc,
    options::{ClientOptions, ServerApi, ServerApiVersion},
    Client,
};
use std::env;

pub async fn connect() -> Result<Client, MongoError> {
    const MONGO_CONN_STR: &str = "MONGO_CONN_STR";

    let conn_uri = env::var(MONGO_CONN_STR).expect(&*format!("${} is not set", MONGO_CONN_STR));
    let mut client_options = ClientOptions::parse(conn_uri).await?;

    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);

    match Client::with_options(client_options) {
        Ok(client) => {
            client
                .database("admin")
                .run_command(doc! {"ping": 1}, None)
                .await?;
            log::info!("Successfully connected to MongoDB");
            Ok(client)
        }
        Err(e) => {
            log::error!("Failed to connect to MongoDB: {}", e);
            Err(e)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::mongo::connect;

    #[tokio::test]
    async fn test_mongodb_connection() {
        let client = connect().await;
        assert!(client.is_ok(), "Failed to connect to MongoDB");
    }
}
