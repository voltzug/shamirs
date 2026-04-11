# ShamirS

[![Crates](https://img.shields.io/crates/v/shamirs.svg)](https://crates.io/crates/shamirs)

Small and secure [Shamir's Secret Sharing](https://en.wikipedia.org/wiki/Shamir%27s_Secret_Sharing) library crate

Documentation:
-    [API reference (docs.rs)](https://docs.rs/blahaj)

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

To get started using ShamirS, see the [Rust docs](https://docs.rs/blahaj)

## Features

### Developer friendly
The API is simple and to the point, with minimal configuration.

### Fast and small
The code is as idiomatic and clean as possible, with minimum external dependencies.

### Secure by design
The implementation forbids the user to choose parameters that would result in an insecure application,
like generating more shares than what's allowed by the finite field length.

## Limitations

Because the Galois finite field it uses is [GF256](https://en.wikipedia.org/wiki/Finite_field#GF(p2)_for_an_odd_prime_p),
only up to 255 shares can be generated for a given secret. A larger number would be insecure as shares would start duplicating.
Nevertheless, the secret can be arbitrarily long as computations are performed on single byte chunks.

## Testing

This crate contains both unit and benchmark tests (as well as the examples included in the docs).
You can run them with `cargo test` and `cargo bench`.

### Benchmark results [min mean max]

| CPU                                       | obtain_shares_dealer            | step_shares_dealer              | recover_secret                  | share_from_bytes                | share_to_bytes                  |
| ----------------------------------------- | ------------------------------- | ------------------------------- | ------------------------------- | ------------------------------- | ------------------------------- |
| ? CPU @ ?GHz  | | | | | |

# License

ShamirS is distributed under the terms of the MIT license.

> [!NOTE]
> ShamirS is a fork of [Blahaj](https://git.distrust.co/public/blahaj), which itself is a fork of the [Sharks](https://github.com/c0dearm/sharks) library.

See [LICENSE-MIT](LICENSE-MIT) and [COPYRIGHT](COPYRIGHT) for details.
