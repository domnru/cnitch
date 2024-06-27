FROM rust AS builder

WORKDIR /app

COPY src src
COPY Cargo.toml Cargo.toml

RUN cargo build --release

FROM debian

WORKDIR /app

COPY --from=builder /app/target/release/cnitch-client /app/cnitch-client

CMD [ "sleep infitity" ]