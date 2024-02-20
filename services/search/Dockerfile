FROM rust:latest

RUN apt update
RUN apt install -y libpq-dev
RUN apt install -y protobuf-compiler

WORKDIR /usr/src/app

COPY . .

RUN cd services/search && cargo install --path .

WORKDIR /usr/src/app/services/search

CMD bash -c "search"