*[chrono](../index.md) / [serde](index.md)*

---

# Module `serde`

Serialization/Deserialization with serde

The [`DateTime`](../datetime/index.md) type has default implementations for (de)serializing to/from the [RFC 3339]
format. This module provides alternatives for serializing to timestamps.

The alternatives are for use with serde's [`with` annotation] combined with the module name.
Alternatively the individual `serialize` and `deserialize` functions in each module can be used
with serde's `serialize_with` and `deserialize_with` annotations.

*Available on crate feature 'serde' only.*





## Contents

- [Modules](#modules)
  - [`ts_nanoseconds`](#ts-nanoseconds)
  - [`ts_nanoseconds_option`](#ts-nanoseconds-option)
  - [`ts_microseconds`](#ts-microseconds)
  - [`ts_microseconds_option`](#ts-microseconds-option)
  - [`ts_milliseconds`](#ts-milliseconds)
  - [`ts_milliseconds_option`](#ts-milliseconds-option)
  - [`ts_seconds`](#ts-seconds)
  - [`ts_seconds_option`](#ts-seconds-option)
- [Structs](#structs)
  - [`DateTimeVisitor`](#datetimevisitor)
- [Enums](#enums)
  - [`SerdeError`](#serdeerror)
- [Functions](#functions)
  - [`invalid_ts`](#invalid-ts)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ts_nanoseconds`](#ts-nanoseconds) | mod | Ser/de to/from timestamps in nanoseconds |
| [`ts_nanoseconds_option`](#ts-nanoseconds-option) | mod | Ser/de to/from optional timestamps in nanoseconds |
| [`ts_microseconds`](#ts-microseconds) | mod | Ser/de to/from timestamps in microseconds |
| [`ts_microseconds_option`](#ts-microseconds-option) | mod | Ser/de to/from optional timestamps in microseconds |
| [`ts_milliseconds`](#ts-milliseconds) | mod | Ser/de to/from timestamps in milliseconds |
| [`ts_milliseconds_option`](#ts-milliseconds-option) | mod | Ser/de to/from optional timestamps in milliseconds |
| [`ts_seconds`](#ts-seconds) | mod | Ser/de to/from timestamps in seconds |
| [`ts_seconds_option`](#ts-seconds-option) | mod | Ser/de to/from optional timestamps in seconds |
| [`DateTimeVisitor`](#datetimevisitor) | struct |  |
| [`SerdeError`](#serdeerror) | enum |  |
| [`invalid_ts`](#invalid-ts) | fn | Create a custom `de::Error` with `SerdeError::InvalidTimestamp`. |

## Modules

- [`ts_nanoseconds`](ts_nanoseconds/index.md) — Ser/de to/from timestamps in nanoseconds
- [`ts_nanoseconds_option`](ts_nanoseconds_option/index.md) — Ser/de to/from optional timestamps in nanoseconds
- [`ts_microseconds`](ts_microseconds/index.md) — Ser/de to/from timestamps in microseconds
- [`ts_microseconds_option`](ts_microseconds_option/index.md) — Ser/de to/from optional timestamps in microseconds
- [`ts_milliseconds`](ts_milliseconds/index.md) — Ser/de to/from timestamps in milliseconds
- [`ts_milliseconds_option`](ts_milliseconds_option/index.md) — Ser/de to/from optional timestamps in milliseconds
- [`ts_seconds`](ts_seconds/index.md) — Ser/de to/from timestamps in seconds
- [`ts_seconds_option`](ts_seconds_option/index.md) — Ser/de to/from optional timestamps in seconds

## Structs

### `DateTimeVisitor`

```rust
struct DateTimeVisitor;
```

#### Trait Implementations

##### `impl Expected for DateTimeVisitor`

- <span id="datetimevisitor-expected-fmt"></span>`fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>`

##### `impl Visitor for DateTimeVisitor`

- <span id="datetimevisitor-visitor-type-value"></span>`type Value = DateTime<FixedOffset>`

- <span id="datetimevisitor-visitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="datetimevisitor-visitor-visit-str"></span>`fn visit_str<E>(self, value: &str) -> Result<<Self as >::Value, E>`

## Enums

### `SerdeError<T: fmt::Display>`

```rust
enum SerdeError<T: fmt::Display> {
    InvalidTimestamp(T),
}
```

#### Trait Implementations

##### `impl<T: fmt::Display> Display for SerdeError<T>`

- <span id="serdeerror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> ToString for SerdeError<T>`

- <span id="serdeerror-tostring-to-string"></span>`fn to_string(&self) -> String`

## Functions

### `invalid_ts`

```rust
fn invalid_ts<E, T>(value: T) -> E
where
    E: de::Error,
    T: fmt::Display
```

Create a custom `de::Error` with `SerdeError::InvalidTimestamp`.

