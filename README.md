# lucas-lehmer

[![crate](https://img.shields.io/crates/v/lucas_lehmer.svg)](https://crates.io/crates/lucas_lehmer)
[![documentation](https://docs.rs/lucas_lehmer/badge.svg)](https://docs.rs/lucas_lehmer)
![minimum rustc nightly](https://img.shields.io/badge/rustc-nightly-red.svg)
[![Travis status](https://travis-ci.org/dittusch/lucas_lehmer.svg?branch=master)](https://travis-ci.org/dittusch/lucas_lehmer)

Implementation of the [Lucas–Lehmer primality test](https://en.wikipedia.org/wiki/Lucas%E2%80%93Lehmer_primality_test) in Rust.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
lucas_lehmer = "0.1.0"
```

and this to your crate root:

```rust
extern crate lucas_lehmer;
```

## Tests

You can run tests by executing `cargo test --release`.
If you want to test a development build omit the `--release` option, but beware:
Testing a debug-build will take much longer. Don't be coconfused if cargo says:

    test m11213_is_a_mersenne_number ... test m11213_is_a_mersenne_number has been running for over 60 seconds

The test will continue running.

## Compatability
The 'lucas_lehmer' crate requires a nightly build of Rust.

## License
Licensed under the terms and conditions of the [MIT license](http://opensource.org/licenses/MIT) license.
