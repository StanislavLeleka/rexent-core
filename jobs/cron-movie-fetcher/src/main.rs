use env_logger::Env;
use movie_fetcher::fetcher;
use movie_reco_client::movie_reco_client::MovieRecoClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    log::info!("Starting movie fetcher job...");

    let fetch_result = fetcher::fetch().await;
    let movie_reco_client = MovieRecoClient::new();

    match fetch_result {
        Err(err) => {
            log::error!("Error during now playing movies fetch: {:?}", err);
        }
        _ => {
            log::info!("Successfully fetched now playing movies");

            log::info!("Sending retrain request to movie reco service");
            movie_reco_client.retrain().await?;
        }
    }

    Ok(())
}
