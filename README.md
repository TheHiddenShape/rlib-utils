# rlib-utils

A UNIX-like system library written in Rust.

## Build

```sh
cargo build              # debug
cargo build --release    # optimized
```

Artifacts:

```
target/debug/librlib_utils.rlib
target/release/librlib_utils.rlib
```

## Test

```sh
cargo test
```

## Use it

From another crate, add a path dependency in its `Cargo.toml`:

```toml
[dependencies]
rlib-utils = { path = "../rlib-utils" }
```

```rust
use rlib_utils::add;
```

## Personal notes

Originally, I wanted to reinterpret 42 Paris' libft project — written in C — in
Rust. But the two languages do not share the same philosophy at all.

On one side, a language that leaves memory entirely to the programmer: the
compiler checks types, not lifetimes or aliasing, and accepts code whose memory
behaviour it cannot reason about.

On the other, Rust, which demands that proof at compile time: the borrow
checker refuses to build until it can prove the program's memory behaviour is
sound.

So the point here is not to mirror C: transposing those internals, raw by
design, would mean reaching for `unsafe` everywhere, and at that point I would
just be writing C with a different syntax. It is to work within the boundaries
Rust sets: ownership, borrowing and types carrying the guarantees the C version
left to the caller.
