fn main() -> Result<(), Box<dyn std::error::Error>> {
    let location = "./proto/location.proto";
    let favorites = "./proto/favorites.proto";
    let users = "./proto/users.proto";
    let context = "./proto/context.proto";
    let search = "./proto/search.proto";
    let reco = "./proto/reco.proto";
    let events = "./proto/events.proto";

    let mut config = prost_build::Config::new();
    config.protoc_arg("--experimental_allow_proto3_optional");

    tonic_build::configure()
        .build_server(true)
        .out_dir("./src/services")
        .compile_with_config(
            config,
            &[location, favorites, users, context, search, reco, events],
            &["./proto"],
        )
        .unwrap_or_else(|e| panic!("protobuf compile error: {}", e));

    Ok(())
}
