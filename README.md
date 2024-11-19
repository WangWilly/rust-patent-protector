# rust-patent-protector

Toy project to learn Rust and Diesel.

## Backend (Rust)

### Setup

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Diesel CLI
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/diesel-rs/diesel/releases/latest/download/diesel_cli-installer.sh | sh

# Create database
cd develop-gears
docker compose up -d

# Run migrations
diesel migration run
```

### Run

```bash
./scripts/serve-backend.sh dev
```

### Misc

- https://diesel.rs/guides/getting-started
- https://www.reddit.com/r/rust/comments/16sj6af/diesel_table_scheme_migration/
- https://github.com/diesel-rs/diesel/blob/master/guide_drafts/trait_derives.md
