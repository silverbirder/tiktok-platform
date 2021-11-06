FROM rust:1.56.0 AS builder
RUN USER=root cargo new --bin app
WORKDIR /app
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs
COPY ./src ./src
RUN cargo install --locked --path .

# @see: https://zenn.dev/ucwork/articles/acec204571362b
FROM debian:buster-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/storer .
COPY ./.env ./.env
ENTRYPOINT ["./storer"]