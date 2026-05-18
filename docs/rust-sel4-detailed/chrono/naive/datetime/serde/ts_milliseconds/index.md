*[chrono](../../../../index.md) / [naive](../../../index.md) / [datetime](../../index.md) / [serde](../index.md) / [ts_milliseconds](index.md)*

---

# Module `ts_milliseconds`

Used to serialize/deserialize from millisecond-precision timestamps

# Example:

```rust
use chrono::{NaiveDate, NaiveDateTime};
use serde_derive::{Deserialize, Serialize};
use chrono::naive::serde::ts_milliseconds;
#[derive(Deserialize, Serialize)]
struct S {
    #[serde(with = "ts_milliseconds")]
    time: NaiveDateTime,
}

let time =
    NaiveDate::from_ymd_opt(2018, 5, 17).unwrap().and_hms_milli_opt(02, 04, 59, 918).unwrap();
let my_s = S { time: time.clone() };

let as_string = serde_json::to_string(&my_s)?;
assert_eq!(as_string, r#"{"time":1526522699918}"#);
let my_s: S = serde_json::from_str(&as_string)?;
assert_eq!(my_s.time, time);
Ok::<(), serde_json::Error>(())
```

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`MilliSecondsTimestampVisitor`](#millisecondstimestampvisitor) | struct |  |
| [`serialize`](#serialize) | fn | Serialize a datetime into an integer number of milliseconds since the epoch |
| [`deserialize`](#deserialize) | fn | Deserialize a `NaiveDateTime` from a milliseconds timestamp |

## Structs

### `MilliSecondsTimestampVisitor`

```rust
struct MilliSecondsTimestampVisitor;
```

#### Trait Implementations

##### `impl Expected for MilliSecondsTimestampVisitor`

- <span id="millisecondstimestampvisitor-expected-fmt"></span>`fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>`

##### `impl Visitor for MilliSecondsTimestampVisitor`

- <span id="millisecondstimestampvisitor-visitor-type-value"></span>`type Value = NaiveDateTime`

- <span id="millisecondstimestampvisitor-visitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="millisecondstimestampvisitor-visitor-visit-i64"></span>`fn visit_i64<E>(self, value: i64) -> Result<<Self as >::Value, E>`

- <span id="millisecondstimestampvisitor-visitor-visit-u64"></span>`fn visit_u64<E>(self, value: u64) -> Result<<Self as >::Value, E>`

## Functions

### `serialize`

```rust
fn serialize<S>(dt: &crate::NaiveDateTime, serializer: S) -> Result<<S as >::Ok, <S as >::Error>
where
    S: ser::Serializer
```

Serialize a datetime into an integer number of milliseconds since the epoch

Intended for use with `serde`s `serialize_with` attribute.

# Example:

```rust
use chrono::{NaiveDate, NaiveDateTime};
use serde_derive::Serialize;
use chrono::naive::serde::ts_milliseconds::serialize as to_milli_ts;
#[derive(Serialize)]
struct S {
    #[serde(serialize_with = "to_milli_ts")]
    time: NaiveDateTime,
}

let my_s = S {
    time: NaiveDate::from_ymd_opt(2018, 5, 17)
        .unwrap()
        .and_hms_milli_opt(02, 04, 59, 918)
        .unwrap(),
};
let as_string = serde_json::to_string(&my_s)?;
assert_eq!(as_string, r#"{"time":1526522699918}"#);
Ok::<(), serde_json::Error>(())
```

### `deserialize`

```rust
fn deserialize<'de, D>(d: D) -> Result<crate::NaiveDateTime, <D as >::Error>
where
    D: de::Deserializer<'de>
```

Deserialize a `NaiveDateTime` from a milliseconds timestamp

Intended for use with `serde`s `deserialize_with` attribute.

# Example:

```rust
use chrono::{DateTime, NaiveDateTime};
use serde_derive::Deserialize;
use chrono::naive::serde::ts_milliseconds::deserialize as from_milli_ts;
#[derive(Debug, PartialEq, Deserialize)]
struct S {
    #[serde(deserialize_with = "from_milli_ts")]
    time: NaiveDateTime,
}

let my_s: S = serde_json::from_str(r#"{ "time": 1526522699918 }"#)?;
let expected = DateTime::from_timestamp(1526522699, 918000000).unwrap().naive_utc();
assert_eq!(my_s, S { time: expected });

let my_s: S = serde_json::from_str(r#"{ "time": -1 }"#)?;
let expected = DateTime::from_timestamp(-1, 999_000_000).unwrap().naive_utc();
assert_eq!(my_s, S { time: expected });
Ok::<(), serde_json::Error>(())
```

