*[chrono](../../../../index.md) / [naive](../../../index.md) / [datetime](../../index.md) / [serde](../index.md) / [ts_seconds](index.md)*

---

# Module `ts_seconds`

Used to serialize/deserialize from second-precision timestamps

# Example:

```rust
use chrono::{NaiveDate, NaiveDateTime};
use serde_derive::{Deserialize, Serialize};
use chrono::naive::serde::ts_seconds;
#[derive(Deserialize, Serialize)]
struct S {
    #[serde(with = "ts_seconds")]
    time: NaiveDateTime,
}

let time = NaiveDate::from_ymd_opt(2015, 5, 15).unwrap().and_hms_opt(10, 0, 0).unwrap();
let my_s = S { time: time.clone() };

let as_string = serde_json::to_string(&my_s)?;
assert_eq!(as_string, r#"{"time":1431684000}"#);
let my_s: S = serde_json::from_str(&as_string)?;
assert_eq!(my_s.time, time);
Ok::<(), serde_json::Error>(())
```

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SecondsTimestampVisitor`](#secondstimestampvisitor) | struct |  |
| [`serialize`](#serialize) | fn | Serialize a datetime into an integer number of seconds since the epoch |
| [`deserialize`](#deserialize) | fn | Deserialize a `NaiveDateTime` from a seconds timestamp |

## Structs

### `SecondsTimestampVisitor`

```rust
struct SecondsTimestampVisitor;
```

#### Trait Implementations

##### `impl Expected for SecondsTimestampVisitor`

- <span id="secondstimestampvisitor-expected-fmt"></span>`fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>`

##### `impl Visitor for SecondsTimestampVisitor`

- <span id="secondstimestampvisitor-visitor-type-value"></span>`type Value = NaiveDateTime`

- <span id="secondstimestampvisitor-visitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="secondstimestampvisitor-visitor-visit-i64"></span>`fn visit_i64<E>(self, value: i64) -> Result<<Self as >::Value, E>`

- <span id="secondstimestampvisitor-visitor-visit-u64"></span>`fn visit_u64<E>(self, value: u64) -> Result<<Self as >::Value, E>`

## Functions

### `serialize`

```rust
fn serialize<S>(dt: &crate::NaiveDateTime, serializer: S) -> Result<<S as >::Ok, <S as >::Error>
where
    S: ser::Serializer
```

Serialize a datetime into an integer number of seconds since the epoch

Intended for use with `serde`s `serialize_with` attribute.

# Example:

```rust
use chrono::{NaiveDate, NaiveDateTime};
use serde_derive::Serialize;
use chrono::naive::serde::ts_seconds::serialize as to_ts;
#[derive(Serialize)]
struct S {
    #[serde(serialize_with = "to_ts")]
    time: NaiveDateTime,
}

let my_s =
    S { time: NaiveDate::from_ymd_opt(2015, 5, 15).unwrap().and_hms_opt(10, 0, 0).unwrap() };
let as_string = serde_json::to_string(&my_s)?;
assert_eq!(as_string, r#"{"time":1431684000}"#);
Ok::<(), serde_json::Error>(())
```

### `deserialize`

```rust
fn deserialize<'de, D>(d: D) -> Result<crate::NaiveDateTime, <D as >::Error>
where
    D: de::Deserializer<'de>
```

Deserialize a `NaiveDateTime` from a seconds timestamp

Intended for use with `serde`s `deserialize_with` attribute.

# Example:

```rust
use chrono::{DateTime, NaiveDateTime};
use serde_derive::Deserialize;
use chrono::naive::serde::ts_seconds::deserialize as from_ts;
#[derive(Debug, PartialEq, Deserialize)]
struct S {
    #[serde(deserialize_with = "from_ts")]
    time: NaiveDateTime,
}

let my_s: S = serde_json::from_str(r#"{ "time": 1431684000 }"#)?;
let expected = DateTime::from_timestamp_secs(1431684000).unwrap().naive_utc();
assert_eq!(my_s, S { time: expected });
Ok::<(), serde_json::Error>(())
```

