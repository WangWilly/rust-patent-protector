################################################################################
# Builder stage
FROM rust as builder
WORKDIR /out

RUN apt update
RUN apt install -y libpq-dev

RUN cargo install diesel_cli --no-default-features --features postgres --root /out

################################################################################
# Runtime stage
FROM debian:stable-slim AS runtime
WORKDIR /app

COPY --from=builder /out/bin/diesel /bin/

# Install PostgreSQL client libraries
RUN apt-get update && apt-get install -y libpq-dev && rm -rf /var/lib/apt/lists/*

COPY ./backend/diesel.toml ./
COPY ./backend/migrations ./migrations
COPY ./scripts/migrate.sh ./entrypoint.sh
COPY ./scripts/is-migrated.sh ./is-migrated.sh

ENTRYPOINT ["/app/entrypoint.sh"]
