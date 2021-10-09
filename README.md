# embassy-size-benchmark

## Prerequisites

- `rustup component add llvm-tools-preview`
- `cargo install cargo-binutils`

## size

- `cargo size --release`
- `cargo size --release --features=executor`
- `cargo size --release --features=executor,spawn`
- `cargo size --release --features=executor,spawn,join`

At the time of writing

```text
executor 296 bytes text 44 bytes bss
spawn     88 bytes text 20 bytes bss
join      80 bytes text  0 bytes bss
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE)
- MIT license ([LICENSE-MIT](LICENSE-MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
