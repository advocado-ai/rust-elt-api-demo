# rust-elt-api (demo)

A small Rust workspace that turns raw ATP match rows (as they arrive from CSV: all strings, many blank) into
validated, typed match records, behind a storage trait. An Axum HTTP API on top of it is planned.

It is a learning and showcase project. Parts of it are still stubs; the layout below says which.

## Workspace

| Crate | Role | State |
|---|---|---|
| `elt-core` | Domain library, no web framework. Record types (`RawMatch`, `MatchRecord`, `Set`, enums), field parsers (surface, tournament level, round, date, score, optional numbers), `TransformError` / `RepositoryError` (via `thiserror`), a `MatchRepository` trait and an in-memory implementation | Parsers and in-memory repository implemented; `transform()` (row to record) is a stub |
| `api` | Thin Axum HTTP adapter around `elt-core` | Not started (empty binary) |

The boundary is deliberate: `elt-core` knows nothing about HTTP or async, so its logic is covered by plain `cargo test`.

## Build and test

The toolchain is pinned in `rust-toolchain.toml` (Rust 1.96.0, with rustfmt and clippy); rustup picks it up automatically.

```bash
cargo build --workspace
cargo test --workspace
cargo fmt --all --check
cargo clippy --workspace --all-targets -- -D warnings
```

## Data

No match data is committed. The record model follows the public ATP match results dataset compiled by Jeff Sackmann
(Tennis Abstract, <https://github.com/JeffSackmann/tennis_atp>), which is published under CC BY-NC-SA 4.0 (non-commercial,
attribution, share-alike). To experiment, download a yearly file such as `atp_matches_2024.csv` into `_data/`, which is
git-ignored. Please read the dataset's own license before reusing its data.

## Planned

- `transform()` and a full set of tests for it
- Kani proofs (bounded model checking) that the parsers never panic on any short ASCII input
- the Axum API (`GET /health`, `POST /imports`, `GET /records`) and a `sqlx`-backed repository

## How it is written

Much of the code is written by AI coding agents working under automated checks (formatting, clippy, tests and, as they
are added, formal proofs) with every change reviewed by a human. Commit trailers record AI co-authorship.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option. This license covers the code; it does not extend to third-party data.
