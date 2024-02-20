FROM rust:latest

RUN apt update
RUN apt install -y libpq-dev

RUN cargo install diesel_cli --no-default-features --features postgres

RUN apt install -y protobuf-compiler

WORKDIR /usr/src/app

COPY . .

RUN cd services/location && cargo install --path .

WORKDIR /usr/src/app/services/location

CMD bash -c "diesel setup && diesel migration run && location"