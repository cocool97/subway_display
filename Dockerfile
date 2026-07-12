FROM rust:alpine3.24 AS BUILDER

RUN mkdir -p /app
WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY proto ./proto
COPY server ./server

RUN cargo build --release -p server

FROM alpine:3.24

RUN mkdir -p /app

COPY --from=BUILDER /app/target/release/server /app/server

ENTRYPOINT ["/app/server"]
