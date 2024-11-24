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

# Database tables
diesel migration generate [migration-name]

# Run migrations
./scripts/migrate.sh
```

### Run

```bash
./scripts/serve-backend.sh dev
```

### Deploy

```bash
./scripts/build-migration.sh
./scripts/build-backend.sh

cd deployments
docker compose up -d
```

Resources:
- 🤔 https://www.reddit.com/r/rust/comments/16bswvl/looking_for_the_perfect_dockerfile_for_rust/
- https://stackoverflow.com/questions/10319652/check-if-a-file-is-executable
- https://www.shuttle.dev/blog/2024/01/09/getting-started-tracing-rust
- ~~https://stackoverflow.com/questions/49098753/unable-to-run-a-docker-image-with-a-rust-executable~~
- https://users.rust-lang.org/t/release-binary-not-working-in-docker/36383/4
- ~~https://stackoverflow.com/questions/30780780/difference-between-stdout-and-dev-stdout~~
- ~~https://stackoverflow.com/questions/74957107/how-to-conditionally-use-tracings-non-blocking-writer-instead-of-stdout~~

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
- https://users.rust-lang.org/t/rust-arrays-and-vectors/117607

## Learning Rust

- https://doc.rust-lang.org/cargo/guide/project-layout.html
- https://doc.rust-lang.org/cargo/reference/manifest.html
- https://github.com/janos-r/axum-template

### To move or to borrow? That is the question

- https://www.reddit.com/r/learnrust/comments/13gbrf4/to_move_or_to_borrow/
- https://users.rust-lang.org/t/rationale-for-move-copy-borrow-syntax/87493

Why This Design? Rust’s Ownership Model

```rust
let mut b1 = 1;
let b2 = &mut b1;
let b3 = &mut b1;  // Fail. Cannot mutably borrow when already mutably borrowed
println!("{:?} {:?} {:?}", b1, b2, b3);
```
1.	First Mutable Borrow: The line `let b2 = &mut b1;` creates a mutable reference to `b1`. At this point, `b1` is mutably borrowed by `b2`.
2.	Second Mutable Borrow: The line `let b3 = &mut b1;` attempts to create another mutable reference to `b1`. This violates Rust’s borrowing rules because `b1` is already mutably borrowed by `b2`. Rust does not allow multiple mutable borrows at the same time to prevent data races.
3.	Borrow Checker Error: The compiler throws an error because it detects that `b1` is being borrowed mutably more than once at the same time, which could lead to undefined behavior if allowed.

Rust’s design aims to prevent data races at compile time by enforcing these borrowing rules. Data races occur when two or more threads access shared data simultaneously, and at least one of the accesses is a write. By ensuring that only one mutable reference exists at any given time, Rust guarantees that no other part of the program can modify the data unexpectedly, thus maintaining memory safety and preventing data races.
This approach allows Rust to provide high performance and safety guarantees without needing a garbage collector, making it suitable for systems programming where both efficiency and reliability are critical.

- https://www.openmymind.net/Rust-Ownership-Move-and-Borrow-part-1/

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

