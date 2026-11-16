# Apache Spark

Apache Spark is a SigmaOS component providing a functional domain model for `apache-spark` with validated state, deterministic storage, and typed errors.

## Description

Apache Spark big data processing compatibility layer for SigmaOS

## Domain model

- `BigData` — domain entity

## API

- `new()`

## Testing

```sh
cargo test
```

Tests are deterministic, isolated, and self-validating.
