*[chrono](../../../../index.md) / [naive](../../../index.md) / [datetime](../../index.md) / [serde](../index.md) / [ts_nanoseconds](index.md)*

---

# Module `ts_nanoseconds`

Used to serialize/deserialize from nanosecond-precision timestamps

# Example:

```rust
use chrono::{NaiveDate, NaiveDateTime};
use serde_derive::{Deserialize, Serialize};
use chrono::naive::serde::ts_nanoseconds;
#[derive(Deserialize, Serialize)]
struct S {
    #[serde(with = "ts_nanoseconds")]
    time: NaiveDateTime,
}

let time = NaiveDate::from_ymd_opt(2018, 5, 17)
    .unwrap()
    .and_hms_nano_opt(02, 04, 59, 918355733)
    .unwrap();
let my_s = S { time: time.clone() };

let as_string = serde_json::to_string(&my_s)?;
assert_eq!(as_string, r#"{"time":1526522699918355733}"#);
let my_s: S = serde_json::from_str(&as_string)?;
assert_eq!(my_s.time, time);
Ok::<(), serde_json::Error>(())
```

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`NanoSecondsTimestampVisitor`](#nanosecondstimestampvisitor) | struct |  |
| [`serialize`](#serialize) | fn | Serialize a datetime into an integer number of nanoseconds since the epoch |
| [`deserialize`](#deserialize) | fn | Deserialize a `NaiveDateTime` from a nanoseconds timestamp |

## Structs

### `NanoSecondsTimestampVisitor`

```rust
struct NanoSecondsTimestampVisitor;
```

#### Trait Implementations

##### `impl Expected for NanoSecondsTimestampVisitor`

- <span id="nanosecondstimestampvisitor-expected-fmt"></span>`fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>`

##### `impl Visitor for NanoSecondsTimestampVisitor`

- <span id="nanosecondstimestampvisitor-visitor-type-value"></span>`type Value = NaiveDateTime`

- <span id="nanosecondstimestampvisitor-visitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="nanosecondstimestampvisitor-visitor-visit-i64"></span>`fn visit_i64<E>(self, value: i64) -> Result<<Self as >::Value, E>`

- <span id="nanosecondstimestampvisitor-visitor-visit-u64"></span>`fn visit_u64<E>(self, value: u64) -> Result<<Self as >::Value, E>`

## Functions

### `serialize`

```rust
fn serialize<S>(dt: &crate::NaiveDateTime, serializer: S) -> Result<<S as >::Ok, <S as >::Error>
where
    S: ser::Serializer
```

Serialize a datetime into an integer number of nanoseconds since the epoch

Intended for use with `serde`s `serialize_with` attribute.

# Errors

An `i64` with nanosecond precision can span a range of ~584 years. This function returns an
error on an out of range `DateTime`.

The dates that can be represented as nanoseconds are between 1677-09-21T00:12:44.0 and
2262-04-11T23:47:16.854775804.

# Example:

```rust
use chrono::{NaiveDate, NaiveDateTime};
use serde_derive::Serialize;
use chrono::naive::serde::ts_nanoseconds::serialize as to_nano_ts;
#[derive(Serialize)]
struct S {
    #[serde(serialize_with = "to_nano_ts")]
    time: NaiveDateTime,
}

let my_s = S {
    time: NaiveDate::from_ymd_opt(2018, 5, 17)
        .unwrap()
        .and_hms_nano_opt(02, 04, 59, 918355733)
        .unwrap(),
};
let as_string = serde_json::to_string(&my_s)?;
assert_eq!(as_string, r#"{"time":1526522699918355733}"#);
Ok::<(), serde_json::Error>(())
```

### `deserialize`

```rust
fn deserialize<'de, D>(d: D) -> Result<crate::NaiveDateTime, <D as >::Error>
where
    D: de::Deserializer<'de>
```

Deserialize a `NaiveDateTime` from a nanoseconds timestamp

Intended for use with `serde`s `deserialize_with` attribute.

# Example:

```rust
use chrono::{DateTime, NaiveDateTime};
use serde_derive::Deserialize;
use chrono::naive::serde::ts_nanoseconds::deserialize as from_nano_ts;
#[derive(Debug, PartialEq, Deserialize)]
struct S {
    #[serde(deserialize_with = "from_nano_ts")]
    time: NaiveDateTime,
}

let my_s: S = serde_json::from_str(r#"{ "time": 1526522699918355733 }"#)?;
let expected = DateTime::from_timestamp(1526522699, 918355733).unwrap().naive_utc();
assert_eq!(my_s, S { time: expected });

let my_s: S = serde_json::from_str(r#"{ "time": -1 }"#)?;
let expected = DateTime::from_timestamp(-1, 999_999_999).unwrap().naive_utc();
assert_eq!(my_s, S { time: expected });
Ok::<(), serde_json::Error>(())
```

