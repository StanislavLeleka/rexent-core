use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::Client;
use std::{fs::File, io::Write};
use tokio_stream::StreamExt;

pub async fn get_client() -> Client {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    log::info!("Initialized AWS client");

    client
}

pub async fn get_object(
    client: &Client,
    bucket: String,
    object: String,
) -> Result<usize, Box<dyn std::error::Error>> {
    log::trace!("bucket:      {}", bucket);
    log::trace!("object:      {}", object);

    let mut file = File::create(format!("out/{}", object.clone()))?;
    log::info!("Created output file: out/{}", object);

    let mut object = client
        .get_object()
        .bucket(bucket.clone())
        .key(object)
        .send()
        .await?;

    log::info!("Retrieved object from AWS S3: {}", bucket);

    let mut byte_count = 0_usize;
    while let Some(bytes) = object.body.try_next().await? {
        let bytes = file.write(&bytes)?;
        byte_count += bytes;
        log::trace!("Intermediate write of {bytes}");
    }

    log::info!("Finished writing {} bytes to file", byte_count);

    Ok(byte_count)
}
