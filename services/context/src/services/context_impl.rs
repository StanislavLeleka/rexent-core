use super::data_collector;
use crate::{
    clients::redis_client,
    models::context::{Context, Emotion, Mood, TimeOfDay},
};
use proto::{
    clients::location::LocationServiceGrpcClient,
    services::{
        context::{Activity, GetContextRequest, NewContextRequest},
        location::Location,
    },
};
use redis::Client;
use tonic::Status;
use weatherapi::{client::WeatherapiClient, models::weather::WeatherData};

#[derive(Clone)]
pub struct ContextServiceImpl {
    pub(crate) redis_client: Client,
    pub(crate) loc_client: LocationServiceGrpcClient,
    pub(crate) weather_client: WeatherapiClient,
}

impl ContextServiceImpl {
    pub fn new(
        redis_client: Client,
        loc_client: LocationServiceGrpcClient,
        weather_client: WeatherapiClient,
    ) -> Self {
        Self {
            redis_client,
            loc_client,
            weather_client,
        }
    }

    pub(super) async fn create(&self, request: NewContextRequest) -> Result<Context, Status> {
        log::info!("Creating context for user_id: {}", request.user_id);

        let location = self.get_location(request.user_id.clone()).await?;
        let weather = self.get_weather(location.lat, location.lng).await?;
        let activity = self.get_activity().await?;
        let mood = self.get_mood(&request)?;
        let naive_local_time = data_collector::get_local_time(request.tz.clone());

        let context = Context {
            user_id: request.user_id,
            location: location.into(),
            activity: activity.into(),
            time_of_day: TimeOfDay {
                time_of_day: naive_local_time,
                tz: request.tz,
            },
            mood,
            weather: weather.into(),
            nearby_friends: vec![],
        };

        log::info!("Saving context to Redis...");

        redis_client::set_context(&self.redis_client, &context).map_err(|err| {
            log::error!("Error saving context to redis: {:?}", err);
            Status::internal("Failed to save context")
        })?;

        log::info!(
            "Context creation successful for user_id: {}",
            context.user_id
        );

        Ok(context)
    }

    pub(super) async fn get(&self, request: GetContextRequest) -> Result<Context, Status> {
        log::info!("Fetching context for user_id: {}", request.user_id);

        let context =
            redis_client::get_context(&self.redis_client, &request.user_id).map_err(|err| {
                log::error!("Error fetching context from Redis: {:?}", err);
                Status::internal("Failed to fetch context")
            })?;

        log::info!(
            "Successfully fetched context for user_id: {}",
            request.user_id
        );

        Ok(context)
    }

    async fn get_location(&self, user_id: String) -> Result<Location, Status> {
        data_collector::get_location(&self.loc_client, user_id.clone())
            .await
            .map_err(|err| {
                log::error!("Error getting user location: {:?}", err);
                Status::internal("Failed to get user location")
            })
            .and_then(|res| {
                res.location.ok_or_else(|| {
                    log::error!("Location was not found");
                    Status::internal("Location was not found")
                })
            })
    }

    async fn get_weather(&self, lat: f32, lng: f32) -> Result<WeatherData, Status> {
        data_collector::get_weather(lat, lng, &self.weather_client)
            .await
            .map_err(|err| {
                log::error!("Error getting weather: {:?}", err);
                Status::internal("Failed to get weather")
            })
    }

    async fn get_activity(&self) -> Result<Activity, Status> {
        data_collector::get_user_activity().await.map_err(|err| {
            log::error!("Error getting user activity: {:?}", err);
            Status::internal("Failed to get user activity")
        })
    }

    fn get_mood(&self, request: &NewContextRequest) -> Result<Mood, Status> {
        match &request.mood {
            Some(mood) => Ok(Mood {
                emotion: Emotion::from_i32(mood.emotion)
                    .ok_or_else(|| Status::internal("Invalid emotion"))?,
                intensity: mood.intensity,
                comment: mood.comment.clone(),
            }),
            None => return Err(Status::internal("Mood was not provided")),
        }
    }
}
