FROM rust:latest

RUN apt update
RUN apt install -y libpq-dev
RUN apt install -y protobuf-compiler

WORKDIR /usr/src/app

COPY . .

RUN cd services/gateway && cargo install --path .

WORKDIR /usr/src/app/services/gateway

CMD bash -c "gateway"