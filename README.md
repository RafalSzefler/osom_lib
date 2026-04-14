osom_lib
========

![Build](https://github.com/RafalSzefler/osom_lib/actions/workflows/main.yml/badge.svg)
![GitHub Tag](https://img.shields.io/github/v/tag/RafalSzefler/osom_lib)

This project implements various data structures and algorithms for use
in osom projects and not only.

It also defines stable ABI, in the sense that all types used here are `repr(C)`.
Most of the libraries are #![no_std], although some do use os syscalls. And some
do enable std through `std` feature (which typically is enabled by default).

Libraries here try to have as little runtime dependencies as possible. Typically
this means only `libc` and `windows-sys` dependencies whenever access to the
operating system is needed.

osom_lib crates
---------------

The libraries here are independent on the operating system and can be used anywhere
Rust runs.

* [`osom_lib`](https://rafalszefler.github.io/osom_lib/osom_lib) This library simply
gathers and re-exports other osom libs defined here. It also supports `std` feature
for enabling standard (often meaning: os dependent) implementations.
* [`osom_lib_reprc`](https://rafalszefler.github.io/osom_lib/osom_lib_reprc) defines macros, and
traits that help working with `#[repr(C)]` representations.
* [`osom_lib_entropy`](https://rafalszefler.github.io/osom_lib/osom_lib_entropy) holds tools for
generating randomness from the surrounding entropy, i.e. the operating system.
* [`osom_lib_entropy_cprng`](https://rafalszefler.github.io/osom_lib/osom_lib_entropy) implements
an entropy generator that generates data from cryptographically secure PRNG, but is seeded from
the OS-specific entropy.
* [`osom_lib_prng`](https://rafalszefler.github.io/osom_lib/osom_lib_prng) defines pseudo
random number generator algorithms, including cryptographically secure.
* [`osom_lib_hash_tables`](https://rafalszefler.github.io/osom_lib/osom_lib_hash_tables) defines several
ABI stable hash tables.
* [`osom_lib_hashes`](https://rafalszefler.github.io/osom_lib/osom_lib_hashes) defines several
ABI stable hashing functions.
* [`osom_lib_macros`](https://rafalszefler.github.io/osom_lib/osom_lib_macros) several general macros
useful in various context.
* [`osom_lib_cfg_ext`](https://rafalszefler.github.io/osom_lib/osom_lib_cfg_ext) proc-macros
that make working with cfg easier.
* [`osom_lib_primitives`](https://rafalszefler.github.io/osom_lib/osom_lib_primitives) holds
several primitives (like Length and CResult) used by other osom libs.
* [`osom_lib_alloc`](https://rafalszefler.github.io/osom_lib/osom_lib_alloc) defines Allocator
trait and related stuff.
* [`osom_lib_arrays`](https://rafalszefler.github.io/osom_lib/osom_lib_arrays) defines and
implements various ABI stable arrays.
* [`osom_lib_numbers`](https://rafalszefler.github.io/osom_lib/osom_lib_numbers) various helpers and
numeric algorithms.
