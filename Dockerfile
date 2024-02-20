FROM rust:latest as builder

RUN apt update
RUN apt install -y libpq-dev

RUN cargo install diesel_cli --no-default-features --features postgres

RUN apt install -y protobuf-compiler

WORKDIR /usr/src/app

COPY . .

# Build the users service
RUN cd services/users && cargo build --release

# Build the location service
RUN cd services/location && cargo build --release

# Build the search service
RUN cd services/search && cargo build --release

# Build the context service
RUN cd services/context && cargo build --release

# Final stage to copy the compiled binaries
FROM debian:buster-slim

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/app/users/target/release/users /usr/local/bin/users
COPY --from=builder /usr/src/app/location/target/release/location /usr/local/bin/location
COPY --from=builder /usr/src/app/search/target/release/search /usr/local/bin/search
COPY --from=builder /usr/src/app/context/target/release/context /usr/local/bin/context

# Expose the ports for each service
EXPOSE 50050 50051 50052 50053