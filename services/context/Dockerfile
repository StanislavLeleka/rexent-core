FROM rust:latest

RUN apt update
RUN apt install -y libpq-dev
RUN apt install -y protobuf-compiler

WORKDIR /usr/src/app

COPY . .

RUN cd services/context && cargo install --path .

WORKDIR /usr/src/app/services/context

CMD bash -c "context"