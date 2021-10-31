FROM rust:1.56.0

WORKDIR /app

COPY . .
RUN cargo build --release

ENTRYPOINT ["/app/target/release/scraper"]