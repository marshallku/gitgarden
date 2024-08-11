FROM rust:1.80-alpine AS base

WORKDIR /usr/src/http_server_template

RUN set -eux; \
    apk add --no-cache musl-dev pkgconfig libressl-dev; \
    rm -rf $CARGO_HOME/registry

COPY Cargo.* .

RUN mkdir src && \
    echo 'fn main() {println!("Hello, world!");}' > src/main.rs && \
    cargo build --release && \
    rm target/release/http_server_template* && \
    rm target/release/deps/http_server_template* && \
    rm -rf src

FROM base AS builder

COPY src src
RUN cargo build --release

FROM alpine:3.20.2

WORKDIR /usr/local/bin

COPY --from=builder /usr/src/http_server_template/target/release/http_server_template .

EXPOSE ${PORT}

CMD ["./http_server_template"]