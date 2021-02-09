# Setup

- The [Rust toolchain][toolchain] is a prerequisite.
- The nightly version of the toolchain because of the benchmarking feature
  Run the following in the project directory

```
rustup override set nightly
```

# Tests

```
cargo test
```

# Benchmarks

```
cargo bench
```

[toolchain]: https://rustup.rs
