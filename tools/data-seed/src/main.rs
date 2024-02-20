use env_logger::Env;

mod users;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    log::info!("Creating users");
    users::create_users(10, "New York".to_string(), 50.456140, 30.490745).await?;

    Ok(())
}
