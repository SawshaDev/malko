# syntax=docker/dockerfile:1
FROM rust:slim-buster as builder

WORKDIR /mochi

# Cargo workspace "necessities"
COPY ./croissant/Cargo.toml /croissant/Cargo.toml
RUN mkdir /croissant/src 
RUN touch /croissant/src/main.rs

COPY ./Cargo.toml /Cargo.toml
COPY ./mochi .

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/target \
    cargo build --release
# Other image cannot access the target folder.
RUN --mount=type=cache,target=/target \
    cp /target/release/mochi /usr/local/bin/mochi

FROM debian:buster-slim

ENV MOCHI_ADDRESS 0.0.0.0
ENV RUST_LOG debug

COPY --from=builder /usr/local/bin/mochi /bin/mochi

CMD ["/bin/mochi"]