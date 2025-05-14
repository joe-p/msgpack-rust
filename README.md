# RMP - Rust MessagePack

RMP is a complete pure-Rust [MessagePack](http://msgpack.org) implementation. MessagePack a compact self-describing binary serialization format.

This project consists of three crates:

* [RMP-Serde][crates-rmps-url] ([Documentation][rmps-docs-url]) — easy serializing/deserializing via [Serde](https://serde.rs).
* [RMP-Value][crates-rmpv-url] ([Documentation][rmpv-docs-url]) — a universal `Value` enum that can hold any MessagePack type. Allows deserializing arbitrary messages without a known schema.
* [RMP][crates-rmp-url] ([Documentation][rmp-docs-url]) — low-level functions for reading/writing encoded data.

Both RMP and RMP-Value support `no_std` environments with the `alloc` crate.

## Features

- **Convenient and powerful APIs**

  RMP is designed to be lightweight and straightforward. There is a high-level API with support for Serde,
  which provides you convenient interface for encode/decode Rust's data structures using `derive` attribute.
  There are also low-level APIs, which give you full control over data encoding/decoding process,
  with no-std support (when the `std` feature is disabled).

- **Zero-copy value decoding**

  RMP allows to decode bytes from a buffer in a zero-copy manner. Parsing is implemented in safe Rust.

- **Robust, stable and tested**

  This project is developed using TDD and CI, so any found bugs will be fixed without breaking
  existing functionality.

## Why MessagePack?

It's smaller and much simpler to parse than JSON. The encoded data is self-describing and extensible, without using any schema definitions. It supports the same data types as JSON, plus binary data, non-string map keys, all float values, and 64-bit numbers. Msgpack values use `<lenght><data>` encoding, so they can be safely concatenated and read from a stream.

MessagePack is similar to CBOR, but has simpler data types (no bignums, decimal floats, dates, or indefinite-length sets, etc.)

## Requirements

- An up-to-date stable version of [Rust](https://www.rust-lang.org), preferably from [rustup](https://rustup.rs).

## Usage in `no_std` Environments

Both `rmp` and `rmpv` support `no_std` environments. To use them in a `no_std` context:

```toml
[dependencies]
rmp = { version = "0.8", default-features = false }
rmpv = { version = "1.3", default-features = false }
```

To compile with no_std support, use:

```bash
# To build without std:
cargo build --no-default-features
```

The main test suite requires the standard library. To run these tests:

```bash
# To run all tests (with std enabled):
cargo test

# To run only the no_std compatible tests:
cargo test --no-default-features
```

When using `no_std`, you'll need to provide the `alloc` crate in your project since MessagePack data structures require dynamic allocation:

```rust
#![no_std]
extern crate alloc;

use alloc::vec::Vec;
use rmpv::Value;
use rmp::decode::Bytes;
use rmp::encode::ByteBuf;

// Create a MessagePack value
let array = Value::Array(vec![
    Value::from(1),
    Value::from("hello"),
    Value::from(true),
]);

// Encode to bytes
let mut buf = ByteBuf::new();
rmpv::encode::write_value(&mut buf, &array).unwrap();
let encoded = buf.as_slice();

// Decode from bytes
let mut bytes = Bytes::new(encoded);
let decoded: Value = rmpv::decode::read_value(&mut bytes).unwrap();

// Zero-copy decoding with ValueRef
let value_ref = rmpv::decode::read_value_ref(&mut &*encoded).unwrap();
```

Note these key differences in no_std mode:

1. Use `rmp::decode::Bytes` or `&[u8]` instead of `std::io::Read`
2. Use `rmp::encode::ByteBuf` instead of `std::io::Write`
3. The `RmpRead` and `RmpWrite` traits are sealed, so you can't implement them directly
4. Remember to import types from the `alloc` crate (`Vec`, `String`, etc.)
5. Import format macros with `use alloc::format;` if needed
6. For no_std tests, use `#[cfg(not(feature = "std"))]` to conditionally compile tests that work without std

[rustc-serialize]: https://github.com/rust-lang-nursery/rustc-serialize
[serde]: https://github.com/serde-rs/serde

[ci-img]: https://github.com/3Hren/msgpack-rust/actions/workflows/ci.yml/badge.svg
[ci-url]: https://github.com/3Hren/msgpack-rust/actions/workflows/ci.yml

[coveralls-img]: https://coveralls.io/repos/3Hren/msgpack-rust/badge.svg?branch=master&service=github
[coveralls-url]: https://coveralls.io/github/3Hren/msgpack-rust?branch=master

[rmp-docs-url]: https://docs.rs/rmp
[rmps-docs-url]: https://docs.rs/rmp-serde
[rmpv-docs-url]: https://docs.rs/rmpv

[crates-rmp-url]: https://lib.rs/crates/rmp
[crates-rmps-url]: https://lib.rs/crates/rmp-serde
[crates-rmpv-url]: https://lib.rs/crates/rmpv


[![Build][ci-img]][ci-url] [![Coverage Status][coveralls-img]][coveralls-url]
