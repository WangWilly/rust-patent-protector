################################################################################
# Base image
FROM lukemathwalker/cargo-chef:latest as chef
WORKDIR /app

# TODO:
# ################################################################################
# # Build planner
# FROM chef AS planner
# COPY ./backend/Cargo.toml ./
# COPY ./backend/src ./src
# RUN cargo chef prepare

################################################################################
# Build stage
FROM chef AS builder
# TODO: COPY --from=planner /app/recipe.json .

COPY ./backend/Cargo.toml ./backend/Cargo.lock ./
COPY ./backend/src ./src
# TODO: RUN cargo chef cook --release

RUN cargo build --release
RUN mv ./target/release/backend ./app

################################################################################
# Runtime stage
FROM debian:stable-slim AS runtime
WORKDIR /app

# Install PostgreSQL client libraries
RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/app ./entrypoint
COPY ./backend/assets ./assets

ENTRYPOINT ["/app/entrypoint"]
