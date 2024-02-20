#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Movie {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(string, tag = "2")]
    pub imdb_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub original_language: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub original_title: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub overview: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub poster_path: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub release_date: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub title: ::prost::alloc::string::String,
    #[prost(float, tag = "9")]
    pub vote_average: f32,
    #[prost(message, repeated, tag = "10")]
    pub genres: ::prost::alloc::vec::Vec<Genre>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Genre {
    #[prost(int32, tag = "1")]
    pub id: i32,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
}
