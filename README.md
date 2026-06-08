# osom_lib

![Build](https://github.com/RafalSzefler/osom_lib/actions/workflows/main.yml/badge.svg)
![GitHub Tag](https://img.shields.io/github/v/tag/RafalSzefler/osom_lib)

A Rust workspace of `no_std`-first data structures and algorithms for osom projects and elsewhere.

## Design goals

- **Stable ABI** — types intended for cross Rust FFI use, `#[repr(C)]` layouts.
- **`no_std` by default** — most crates avoid the standard library; OS access is opt-in via the `std` feature on the umbrella crate or individual libraries. The most important thing that `std` provides is the standard allocator.
- **Minimal dependencies** — runtime deps are limited where possible; OS integration typically uses only `libc` and `windows-sys`.
- **Composable crates** — use the [`osom_lib`](https://docs.rs/osom_lib) facade or depend on individual workspace crates.
- **serde support** - optional support with `serde` feature flag.

Requires **Rust 1.95** or newer (see `rust-version` in the workspace `Cargo.toml`).

## Quick start

Add the umbrella crate to `Cargo.toml`:

```toml
[dependencies]
osom_lib = { version = "0.1" }
```

or add each create separately:

```toml
[dependencies]
osom_lib_alloc = { version = "0.1" }
osom_lib_arrays = { version = "0.1" }
```

## Features (`osom_lib`)

| Feature | Description |
|---------|-------------|
| *(default)* | `no_std`; no OS entropy or `std` helpers. No `serde` support. |
| `std` | Standard-library and OS-backed code (allocators, entropy, and related paths). |
| `serde` | Serde support on participating crates (e.g. `arrays`, `hash_tables`, `strings`, etc). |

Individual workspace crates may define their own `std` / `serde` features; see each crate’s `Cargo.toml`.

## Workspace crates

Libraries are platform-agnostic unless noted; entropy crates require the `std` feature (or their own OS integration) to read system randomness.

| Crate | Role |
|-------|------|
| [`osom_lib`](https://rafalszefler.github.io/osom_lib/osom_lib) | Facade that re-exports the workspace; enable `std` / `serde` here for the full stack. |
| [`osom_lib_primitives`](https://rafalszefler.github.io/osom_lib/osom_lib_primitives) | Core ABI-stable primitives (`Length`, `CResult`, alignment helpers, and related types). |
| [`osom_lib_reprc`](https://rafalszefler.github.io/osom_lib/osom_lib_reprc) | Macros and traits for working with `#[repr(C)]` types. |
| [`osom_lib_macros`](https://rafalszefler.github.io/osom_lib/osom_lib_macros) | General-purpose macros shared across the workspace. |
| [`osom_lib_alloc`](https://rafalszefler.github.io/osom_lib/osom_lib_alloc) | `Allocator` trait and allocation-related utilities. |
| [`osom_lib_arrays`](https://rafalszefler.github.io/osom_lib/osom_lib_arrays) | ABI-stable array types and helpers. |
| [`osom_lib_arc`](https://rafalszefler.github.io/osom_lib/osom_lib_arc) | ABI-stable reference-counted smart pointers. |
| [`osom_lib_boxed`](https://rafalszefler.github.io/osom_lib/osom_lib_boxed) | ABI-stable box type for single-owner heap values. |
| [`osom_lib_btree`](https://rafalszefler.github.io/osom_lib/osom_lib_btree) | ABI-stable data structures and algorithms for B-Tree. |
| [`osom_lib_hash_tables`](https://rafalszefler.github.io/osom_lib/osom_lib_hash_tables) | ABI-stable hash table implementations. |
| [`osom_lib_hashes`](https://rafalszefler.github.io/osom_lib/osom_lib_hashes) | ABI-stable hash functions. |
| [`osom_lib_entropy`](https://rafalszefler.github.io/osom_lib/osom_lib_entropy) | Collecting randomness from the operating system. |
| [`osom_lib_entropy_cprng`](https://rafalszefler.github.io/osom_lib/osom_lib_entropy_cprng) | CSPRNG seeded from OS entropy. |
| [`osom_lib_prng`](https://rafalszefler.github.io/osom_lib/osom_lib_prng) | Pseudo-random number generators, including cryptographically secure algorithms. |
| [`osom_lib_numbers`](https://rafalszefler.github.io/osom_lib/osom_lib_numbers) | Numeric helpers and algorithms. |
| [`osom_lib_strings`](https://rafalszefler.github.io/osom_lib/osom_lib_strings) | String types and string-oriented algorithms. |
| [`osom_lib_try_clone`](https://rafalszefler.github.io/osom_lib/osom_lib_try_clone) | `TryClone` trait and its implementation for some primitives. |

All crates starting with `priv_osom_lib_` prefix are intended for internal usage only, and as such
should not be used.

## Building and testing

From the repository root:

```bash
cargo build --workspace --all-features
cargo test --workspace --all-features
cargo clippy --workspace --all-features
```

## License

Licensed under the [MIT License](LICENSE).
