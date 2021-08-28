FROM rust:1.54.0 as builder
WORKDIR /usr/src/spotify-crawler
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install -y openssl && rm -rf /var/lib/apt/lists/*
RUN rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/spotify-crawler /usr/local/bin/spotify-crawler
WORKDIR /usr/src
CMD ["spotify-crawler", "new-release"]
