FROM --platform=linux/arm64 rust:alpine AS builder

WORKDIR /app

COPY src src
COPY Cargo.toml Cargo.toml

RUN apk update && apk upgrade && apk add musl-dev
RUN rustup target add aarch64-unknown-linux-musl

RUN cargo build --target aarch64-unknown-linux-musl --release

FROM --platform=linux/arm64 scratch

WORKDIR /app

COPY --from=builder /app/target/aarch64-unknown-linux-musl/release/cnitch-server /app/cnitch-server

ENTRYPOINT [ "/app/cnitch-server" ]