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
./scripts/migrate.sh
```

### Run

```bash
./scripts/serve-backend.sh dev
```

### Misc

- https://diesel.rs/guides/getting-started
- https://www.reddit.com/r/rust/comments/16sj6af/diesel_table_scheme_migration/
- https://github.com/diesel-rs/diesel/blob/master/guide_drafts/trait_derives.md
- https://stackoverflow.com/questions/27589054/what-is-the-correct-way-to-use-lifetimes-with-a-struct-in-rust

- https://stackoverflow.com/questions/77540941/rust-axum-router-sub-directories
- https://docs.rs/axum/latest/axum/struct.Router.html#method.merge
- https://medium.com/geekculture/dependency-injection-in-rust-3822bf689888

- https://play.rust-lang.org/?version=stable&mode=debug&edition=2015&gist=2794e5f6f7015cb3c018dca111cf732e

- https://users.rust-lang.org/t/why-use-diesel-when-its-not-async/90160

## Learning Rust

- https://doc.rust-lang.org/cargo/guide/project-layout.html
- https://doc.rust-lang.org/cargo/reference/manifest.html
- https://github.com/janos-r/axum-template

### Why using macros?

To illustrate the necessity of using macros in Rust, let's consider a situation where you need to generate repetitive or boilerplate code. Macros provide a powerful way to automate this process, reducing errors and improving maintainability.

[Read more](./docs/macro.md)

### Lifetimes explaination

Lifetimes are a fundamental concept in Rust that help the compiler ensure memory safety without the need for a garbage collector. Understanding lifetimes is crucial for writing safe and efficient Rust code.

[Read more](./docs/lifetime.md)

### Logging in Rust

- https://stackoverflow.com/questions/75009289/how-to-enable-logging-tracing-with-axum

### Error Handling in Rust

You should use `core::result::Result` when you need to represent the outcome of an operation that can either succeed or fail. This type is particularly useful in functions that may encounter errors and need to propagate them to the caller.

[Read more](./docs/error.md)

