# Contributing

Every change should start with an issue.
Resolving existing issues, especially ones that are not assigned, is valued more highly.
A task issue carries the work-item identifier in its title.
Its pull request closes that issue.

## Toolchain

This project builds on stable Rust.
Install the `rustfmt` and `clippy` components with your toolchain, since the formatting and lint checks need them.

The coverage workflow uses `cargo-tarpaulin` and uploads its report to Codecov.
The security workflow uses `cargo-audit`.
Benchmarks use Criterion through the crate's dev-dependencies.
None of these tools is required for a local change, but the dedicated workflows run them.

## Checks

Run these before requesting review:

```sh
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo test --no-default-features
cargo check
```

Formatting, Clippy with warnings denied, the default-feature tests, the no-default-feature tests, and the compile check all run in CI.

## Commits

A commit summary has the form `type(scope): summary`.
The scope names the work item as in `071-04`.
The permitted types are `feat`, `fix`, `refactor`, `test`, `docs`, and `chore`.
Keep the first line under 72 characters.
The summary states what the change does, not how it was arrived at.

## Branches

Each version has one integration branch named for the version, such as `dev-v0.7.1/main`.
Each task issue gets a branch from that integration branch, named with the task identifier and a slug of the issue title.
The repository uses `dev-v0.7.1/main`, `dev-v0.7.1/T-071-01-gf256-div-zero` and `dev-v0.7.1/T-071-02-enhance-unit-tests` as examples.

A task pull request targets its version's integration branch and is squash-merged into it.
After that version's planned tasks have landed, the integration branch is squash-merged into the default branch.
Deployments are created from the default branch.

## Pull requests

A pull request title must name the task or issue it belongs to, as in `T-071-01 GF256 div zero`.
