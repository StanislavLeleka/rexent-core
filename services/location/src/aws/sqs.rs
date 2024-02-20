use std::env;

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_sqs::{config::Region, meta::PKG_VERSION, Client, Error};

use crate::models::{
    city::{City, NewLocationSQSMessage},
    country::Country,
};

#[derive(Clone)]
pub struct SQS {
    client: Client,
    queue: String,
}

impl SQS {
    pub async fn new() -> Result<Self, Error> {
        let region = env::var("AWS_DEFAULT_REGION").expect("$AWS_DEFAULT_REGION must be set");
        let region_provider = RegionProviderChain::first_try(Region::new(region));

        log::info!("SQS client version:   {}", PKG_VERSION);
        log::info!(
            "Region:               {}",
            region_provider.region().await.unwrap().as_ref()
        );

        let shared_config = aws_config::from_env().region(region_provider).load().await;
        let client = Client::new(&shared_config);
        let queue = env::var("AWS_QUEUE_URL").expect("$AWS_QUEUE_URL must be set");

        Ok(Self { client, queue })
    }

    pub async fn send(&self, message: &String) -> Result<(), Error> {
        log::info!("Sending message to queue with URL: {}", &self.queue);

        if let Err(err) = self
            .client
            .send_message()
            .queue_url(&self.queue)
            .message_body(message)
            .send()
            .await
        {
            log::warn!("Error during sending the message: {:#?}", err);
        } else {
            log::info!("Sent message to queue");
        }

        Ok(())
    }

    pub async fn notify_new_location(
        &self,
        country: &Country,
        city: &City,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let message = NewLocationSQSMessage {
            country_code: country.country_code.clone(),
            country_name: country.country_name.clone(),
            city_code: city.city_code.clone(),
            city_name: city.city_name.clone(),
        };

        let json = serde_json::to_string(&message)?;
        self.send(&json).await.map_err(|err| {
            log::error!("Error during sending message to queue: {:?}", err);
            err.into()
        })
    }
}
