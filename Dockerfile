FROM rust:1.95-alpine3.23 as chef
RUN cargo install cargo-chef --locked
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
RUN apk add pkgconf openssl-dev openssl openssl-libs-static musl-dev
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json --target x86_64-unknown-linux-musl
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM alpine:3.23
ARG UID=1000
ARG GID=1000
WORKDIR /app
RUN addgroup -S -g ${GID} app && adduser -S -G app -u ${UID} app
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/rss-feed-bot .
USER app
ENTRYPOINT ["/app/rss-feed-bot"]
