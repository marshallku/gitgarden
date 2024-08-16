FROM rust:1.80-alpine AS base

WORKDIR /usr/src/gitgarden

RUN set -eux; \
    apk add --no-cache musl-dev pkgconfig libressl-dev; \
    rm -rf $CARGO_HOME/registry

COPY Cargo.* .

RUN mkdir src && \
    echo 'fn main() {println!("Hello, world!");}' > src/main.rs && \
    cargo build --release && \
    rm target/release/gitgarden* && \
    rm target/release/deps/gitgarden* && \
    rm -rf src

FROM base AS builder

COPY src src
RUN cargo build --release

FROM alpine:3.20.2

WORKDIR /usr/local/bin

COPY --from=builder /usr/src/gitgarden/target/release/gitgarden .
COPY assets /usr/src/gitgarden/assets

EXPOSE ${PORT}

CMD ["./gitgarden"]