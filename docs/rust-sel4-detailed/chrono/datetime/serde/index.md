*[chrono](../../index.md) / [datetime](../index.md) / [serde](index.md)*

---

# Module `serde`

documented at re-export site

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

