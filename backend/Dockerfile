################################################################################
# Base image
FROM lukemathwalker/cargo-chef:latest as chef
WORKDIR /app

################################################################################
# Build planner
FROM chef AS planner
COPY ./backend/Cargo.toml ./backend/Cargo.lock ./
COPY ./backend/src ./src
RUN cargo chef prepare --recipe-path recipe.json

################################################################################
# Build stage
FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY ./backend/Cargo.toml ./backend/Cargo.lock ./
COPY ./backend/src ./src

RUN cargo build --release --bin backend
RUN mv ./target/release/backend ./entrypoint

################################################################################
# Runtime stage
FROM debian:stable-slim AS runtime
WORKDIR /app

# Install PostgreSQL client libraries
RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/entrypoint ./
COPY ./backend/assets ./assets

ENTRYPOINT ["/app/entrypoint"]
