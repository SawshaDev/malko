# syntax=docker/dockerfile:1
FROM rust:slim-buster as builder

WORKDIR /croissant

# Cargo workspace "necessities"
COPY ./mochi/Cargo.toml /mochi/Cargo.toml
RUN mkdir /mochi/src
RUN touch /mochi/src/main.rs 

COPY ./Cargo.toml /Cargo.toml
COPY ./croissant .

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/target \
    cargo build --release
# Other image cannot access the target folder.
RUN --mount=type=cache,target=/target \
    cp /target/release/croissant /usr/local/bin/croissant


FROM debian:buster-slim

COPY --from=builder /usr/local/bin/croissant /bin/croissant

# Don't forget to also publish these ports in the docker-compose.yml file.
ARG PORT=9934

EXPOSE $PORT
ENV ROCKET_ADDRESS 0.0.0.0
ENV CROISSANT_PORT $PORT

ENV RUST_LOG debug

CMD ["/bin/croissant"]