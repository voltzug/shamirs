# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.7.0] - 2026-04-11
### Changed
- When `zeroize_memory` is enabled, recovered secrets are returned as `Zeroizing<Vec<u8>>`, ensuring memory is wiped on drop
- Bumped `zeroize` to 1.8.2, `hashbrown` to 0.15.5, and crate version to 0.7.0
- Rename project to `shamirs`

### Fixed
- Fixed cache-timing side channel in GF256 math by removing table lookups and using constant-time shift/add multiplication and exponentiation (no LOG/EXP tables, no branches, no cache leaks)
- `recover()` now rejects duplicate X coordinates in shares, preventing Lagrange math from breaking and returning invalid results


## [0.6.0] - 2024-11-16
### Changed
- Rename project to `blahaj` following soft-fork
- Use `u8::MAX` instead of deprecated `u8::max_value()`
- Upgraded project dependencies

### Fixed
- Lower bound of polynomial coefficients changed from 1 to 0

## [0.5.0] - 2021-03-14
### Added
- Zeroize memory on drop for generated secret shares

## [0.4.3] - 2021-02-04
### Changed
- Upgraded project dependencies

## [0.4.2] - 2020-08-03
### Fixed
- Small fix in docs

## [0.4.1] - 2020-04-23
### Added
- Fuzz tests

### Fixed
- Unexpected panic when trying to recover secret from different length shares
- Unexpected panic when trying to convert less than 2 bytes to `Share`

## [0.4.0] - 2020-04-02
### Added
- It is now possible to compile without `std` with `--no-default-features`

## [0.3.3] - 2020-03-23
### Changed
- Fix codecov badge

## [0.3.2] - 2020-03-09
### Changed
- Share structs now derives the `Clone` trait

## [0.3.1] - 2020-01-23
### Changed
- Sharks recover method now accepts any iterable collection

## [0.3.0] - 2020-01-22
### Added
- Share struct which allows to convert from/to byte vectors

### Changed
- Methods use the new Share struct, instead of (GF245, Vec<GF256>) tuples

## [0.2.0] - 2020-01-21
### Added
- Computations performed over GF256 (much faster)
- Secret can now be arbitrarily long

### Changed
- Some method names and docs
- Maximum number of shares enforced by Rust static types instead of conditional branching

### Removed
- Modular arithmetic around Mersenne primes

## [0.1.1] - 2020-01-13
### Fixed
- Typo in cargo description

### Removed
- Maintenance badges in cargo file

## [0.1.0] - 2020-01-13
### Added
- Initial version
