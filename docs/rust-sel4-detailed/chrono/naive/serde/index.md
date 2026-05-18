*[chrono](../../index.md) / [naive](../index.md) / [serde](index.md)*

---

# Module `serde`

Serialization/Deserialization of `NaiveDateTime` in alternate formats

The various modules in here are intended to be used with serde's [`with` annotation] to
serialize as something other than the default ISO 8601 format.


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
  - [`NaiveDateTimeVisitor`](#naivedatetimevisitor)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ts_nanoseconds`](#ts-nanoseconds) | mod | Used to serialize/deserialize from nanosecond-precision timestamps |
| [`ts_nanoseconds_option`](#ts-nanoseconds-option) | mod | Ser/de to/from optional timestamps in nanoseconds |
| [`ts_microseconds`](#ts-microseconds) | mod | Used to serialize/deserialize from microsecond-precision timestamps |
| [`ts_microseconds_option`](#ts-microseconds-option) | mod | Ser/de to/from optional timestamps in microseconds |
| [`ts_milliseconds`](#ts-milliseconds) | mod | Used to serialize/deserialize from millisecond-precision timestamps |
| [`ts_milliseconds_option`](#ts-milliseconds-option) | mod | Ser/de to/from optional timestamps in milliseconds |
| [`ts_seconds`](#ts-seconds) | mod | Used to serialize/deserialize from second-precision timestamps |
| [`ts_seconds_option`](#ts-seconds-option) | mod | Ser/de to/from optional timestamps in seconds |
| [`NaiveDateTimeVisitor`](#naivedatetimevisitor) | struct |  |

## Modules

- [`ts_nanoseconds`](ts_nanoseconds/index.md) — Used to serialize/deserialize from nanosecond-precision timestamps
- [`ts_nanoseconds_option`](ts_nanoseconds_option/index.md) — Ser/de to/from optional timestamps in nanoseconds
- [`ts_microseconds`](ts_microseconds/index.md) — Used to serialize/deserialize from microsecond-precision timestamps
- [`ts_microseconds_option`](ts_microseconds_option/index.md) — Ser/de to/from optional timestamps in microseconds
- [`ts_milliseconds`](ts_milliseconds/index.md) — Used to serialize/deserialize from millisecond-precision timestamps
- [`ts_milliseconds_option`](ts_milliseconds_option/index.md) — Ser/de to/from optional timestamps in milliseconds
- [`ts_seconds`](ts_seconds/index.md) — Used to serialize/deserialize from second-precision timestamps
- [`ts_seconds_option`](ts_seconds_option/index.md) — Ser/de to/from optional timestamps in seconds

## Structs

### `NaiveDateTimeVisitor`

```rust
struct NaiveDateTimeVisitor;
```

#### Trait Implementations

##### `impl Expected for NaiveDateTimeVisitor`

- <span id="naivedatetimevisitor-expected-fmt"></span>`fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>`

##### `impl Visitor for NaiveDateTimeVisitor`

- <span id="naivedatetimevisitor-visitor-type-value"></span>`type Value = NaiveDateTime`

- <span id="naivedatetimevisitor-visitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="naivedatetimevisitor-visitor-visit-str"></span>`fn visit_str<E>(self, value: &str) -> Result<<Self as >::Value, E>`

