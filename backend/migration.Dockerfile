# TODO: Fix the dockerfile to use multi-stage builds
# ################################################################################
# # Builder stage
# FROM rust as builder

# RUN apt update
# RUN apt install -y libpq-dev

# RUN cargo install diesel_cli --no-default-features --features postgres
# RUN mkdir -p /out && cp $(which diesel) /out/diesel

# ################################################################################
# # Runtime stage
# FROM alpine:latest
# WORKDIR /app

# COPY --from=builder /out/diesel /bin/

# COPY ./backend/ ./
# COPY ./scripts/migrate.sh ./entrypoint.sh

# # ENTRYPOINT ["/app/entrypoint.sh"]
# CMD ["sleep", "infinity"]

################################################################################
FROM rust
WORKDIR /app

RUN apt update
RUN apt install -y libpq-dev

RUN cargo install diesel_cli --no-default-features --features postgres

COPY ./backend/diesel.toml ./
COPY ./backend/migrations ./migrations
COPY ./scripts/migrate.sh ./entrypoint.sh

ENTRYPOINT ["/app/entrypoint.sh"]
