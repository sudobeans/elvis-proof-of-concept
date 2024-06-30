# syntax=docker/dockerfile:1

# Comments are provided throughout this file to help you get started.
# If you need more help, visit the Dockerfile reference guide at
# https://docs.docker.com/go/dockerfile-reference/

# Want to help us make this template better? Share your feedback here: https://forms.gle/ybq9Krt8jtBL3iCk7

ARG RUST_VERSION=1.79.0

################################################################################
# Create a stage for building the application.

FROM rust:${RUST_VERSION}-alpine AS final
WORKDIR /app

# Install host build dependencies.
RUN apk add --no-cache clang lld musl-dev git gcc
RUN rustup target add x86_64-unknown-linux-gnu

ADD ./main /app/main
ADD ./plugin /app/plugin
ADD ./Cargo.lock /app/
ADD ./Cargo.toml /app/

CMD RUSTFLAGS="-C target-feature=-crt-static" cargo run

