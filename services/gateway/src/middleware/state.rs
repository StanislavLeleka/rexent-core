use proto::clients::{
    context::ContextServiceGrpcClient, events::EventsServiceGrpcClient,
    favorites::FavoritesServiceGrpcClient, location::LocationServiceGrpcClient,
    reco::RecoServiceGrpcClient, search::SearchServiceGrpcClient, users::UserServiceGrpcClient,
};

#[derive(Clone)]
pub struct State {
    pub users_client: UserServiceGrpcClient,
    pub loc_client: LocationServiceGrpcClient,
    pub context_client: ContextServiceGrpcClient,
    pub search_client: SearchServiceGrpcClient,
    pub favorites_client: FavoritesServiceGrpcClient,
    pub reco_client: RecoServiceGrpcClient,
    pub events_client: EventsServiceGrpcClient,
}
