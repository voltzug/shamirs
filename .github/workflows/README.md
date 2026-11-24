# CI

All workflows run without dependency or build caching.
no-default-features test checks that configuration.

## Patrol

checks formatting, runs Clippy, and verifies the crate compiles.

## Tests

runs test suites.

## Coverage

measures test coverage with Tarpaulin and uploads reports to Codecov when credentials are available.

## Security audit

checks dependencies for known vulnerabilities on code changes and on a daily schedule.
