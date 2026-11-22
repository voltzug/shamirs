# ShamirS

[![Crates](https://img.shields.io/crates/v/shamirs.svg)](https://crates.io/crates/shamirs)

Small and secure [Shamir's Secret Sharing](https://en.wikipedia.org/wiki/Shamir%27s_Secret_Sharing) library crate

Documentation:

- [API reference (docs.rs)](https://docs.rs/shamirs)

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
shamirs = "0.7"
```

If your environment doesn't support `std`:

```toml
[dependencies]
shamirs = { version = "0.7", default-features = false }
```

To get started using ShamirS, see the [Rust docs](https://docs.rs/shamirs)

## Features

The default features are `std` and `zeroize_memory`.

| Feature | Effect |
| --- | --- |
| `std` | Enables the standard library and the thread-local `dealer` method. |
| `zeroize_memory` | Returns recovered secrets in a wrapper that wipes the buffer on drop and zeroizes shares on drop. |
| `fuzzing` | Enables `std` and derives `Arbitrary` for field values and shares, plus `Debug` for shares. |

## Implementation

Field arithmetic is GF(2^8) with reduction polynomial `0x11D`.
Multiplication is branch-free shift-and-add and uses no lookup tables.
Division multiplies by the Fermat inverse and branches once on a zero divisor, so the implementation is not unconditionally constant-time.

## Limitations

The dealer assigns X coordinates from 1 through 255, so it issues at most 255 shares for a secret.
The threshold is the minimum number of distinct shares needed to recover a secret, and a threshold of `k` builds a polynomial of degree `k - 1` for each secret byte.
Secrets can be of any length, including empty; empty secrets produce shares with empty value vectors.
Each share carries one field value per secret byte.

## Testing

The crate has unit tests, contract tests, and doctests.
Run the default-feature suite with `cargo test` and the no-default-feature suite with `cargo test --no-default-features`.
Run the Criterion benchmarks with `cargo bench`.

### Benchmark results [min mean max]

<!-- cargo bench --bench benchmarks -- --save-baseline baseline | tee ~/shamirs-bench.txt -->

| Version | CPU | toolchain | obtain_shares_dealer | step_shares_dealer | recover_secret | share_from_bytes | share_to_bytes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| [v0.7.1 (unreleased)](https://github.com/voltzug/shamirs/releases) | AMD Ryzen 3 3200U @ 2.60GHz | rustc 1.99.1 , criterion 0.8.2 | 1.6755 µs / 1.6837 µs / 1.6922 µs | 2.4324 ns / 2.4893 ns / 2.5768 ns | 7.7791 ms / 7.8102 ms / 7.8428 ms | 38.850 ns / 40.087 ns / 41.346 ns | 29.935 ns / 30.178 ns / 30.464 ns |
| [pre-fix fork fc517a8](https://github.com/voltzug/shamirs/commit/fc517a8) | AMD Ryzen 3 3200U @ 2.60GHz | rustc 1.99.1 , criterion 0.3 | 1.6560 µs / 1.6653 µs / 1.6757 µs | 2.0557 ns / 2.0854 ns / 2.1182 ns | 275.54 µs / 284.47 µs / 292.72 µs | 45.964 ns / 46.222 ns / 46.521 ns | 21.971 ns / 22.583 ns / 23.336 ns |

## Contributing

See [CONTRIBUTING] for the contribution process.

## License

ShamirS is distributed under the terms of the MIT license.

> [!NOTE]
> ShamirS is a fork of [Blahaj](https://git.distrust.co/public/blahaj), which itself is a fork of the [Sharks](https://github.com/c0dearm/sharks) library.

See [LICENSE-MIT] and [COPYRIGHT] for details.

[CONTRIBUTING]: CONTRIBUTING.md
[LICENSE-MIT]: LICENSE-MIT
[COPYRIGHT]: COPYRIGHT
