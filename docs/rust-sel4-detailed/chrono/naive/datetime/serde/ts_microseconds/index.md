*[chrono](../../../../index.md) / [naive](../../../index.md) / [datetime](../../index.md) / [serde](../index.md) / [ts_microseconds](index.md)*

---

# Module `ts_microseconds`

Used to serialize/deserialize from microsecond-precision timestamps

# Example:

```rust
use chrono::{NaiveDate, NaiveDateTime};
use serde_derive::{Deserialize, Serialize};
use chrono::naive::serde::ts_microseconds;
#[derive(Deserialize, Serialize)]
struct S {
    #[serde(with = "ts_microseconds")]
    time: NaiveDateTime,
}

let time = NaiveDate::from_ymd_opt(2018, 5, 17)
    .unwrap()
    .and_hms_micro_opt(02, 04, 59, 918355)
    .unwrap();
let my_s = S { time: time.clone() };

let as_string = serde_json::to_string(&my_s)?;
assert_eq!(as_string, r#"{"time":1526522699918355}"#);
let my_s: S = serde_json::from_str(&as_string)?;
assert_eq!(my_s.time, time);
Ok::<(), serde_json::Error>(())
```

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`MicroSecondsTimestampVisitor`](#microsecondstimestampvisitor) | struct |  |
| [`serialize`](#serialize) | fn | Serialize a datetime into an integer number of microseconds since the epoch |
| [`deserialize`](#deserialize) | fn | Deserialize a `NaiveDateTime` from a microseconds timestamp |

## Structs

### `MicroSecondsTimestampVisitor`

```rust
struct MicroSecondsTimestampVisitor;
```

#### Trait Implementations

##### `impl Expected for MicroSecondsTimestampVisitor`

- <span id="microsecondstimestampvisitor-expected-fmt"></span>`fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>`

##### `impl Visitor for MicroSecondsTimestampVisitor`

- <span id="microsecondstimestampvisitor-visitor-type-value"></span>`type Value = NaiveDateTime`

- <span id="microsecondstimestampvisitor-visitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="microsecondstimestampvisitor-visitor-visit-i64"></span>`fn visit_i64<E>(self, value: i64) -> Result<<Self as >::Value, E>`

- <span id="microsecondstimestampvisitor-visitor-visit-u64"></span>`fn visit_u64<E>(self, value: u64) -> Result<<Self as >::Value, E>`

## Functions

### `serialize`

```rust
fn serialize<S>(dt: &crate::NaiveDateTime, serializer: S) -> Result<<S as >::Ok, <S as >::Error>
where
    S: ser::Serializer
```

Serialize a datetime into an integer number of microseconds since the epoch

Intended for use with `serde`s `serialize_with` attribute.

# Example:

```rust
use chrono::{NaiveDate, NaiveDateTime};
use serde_derive::Serialize;
use chrono::naive::serde::ts_microseconds::serialize as to_micro_ts;
#[derive(Serialize)]
struct S {
    #[serde(serialize_with = "to_micro_ts")]
    time: NaiveDateTime,
}

let my_s = S {
    time: NaiveDate::from_ymd_opt(2018, 5, 17)
        .unwrap()
        .and_hms_micro_opt(02, 04, 59, 918355)
        .unwrap(),
};
let as_string = serde_json::to_string(&my_s)?;
assert_eq!(as_string, r#"{"time":1526522699918355}"#);
Ok::<(), serde_json::Error>(())
```

### `deserialize`

```rust
fn deserialize<'de, D>(d: D) -> Result<crate::NaiveDateTime, <D as >::Error>
where
    D: de::Deserializer<'de>
```

Deserialize a `NaiveDateTime` from a microseconds timestamp

Intended for use with `serde`s `deserialize_with` attribute.

# Example:

```rust
use chrono::{DateTime, NaiveDateTime};
use serde_derive::Deserialize;
use chrono::naive::serde::ts_microseconds::deserialize as from_micro_ts;
#[derive(Debug, PartialEq, Deserialize)]
struct S {
    #[serde(deserialize_with = "from_micro_ts")]
    time: NaiveDateTime,
}

let my_s: S = serde_json::from_str(r#"{ "time": 1526522699918355 }"#)?;
let expected = DateTime::from_timestamp(1526522699, 918355000).unwrap().naive_utc();
assert_eq!(my_s, S { time: expected });

let my_s: S = serde_json::from_str(r#"{ "time": -1 }"#)?;
let expected = DateTime::from_timestamp(-1, 999_999_000).unwrap().naive_utc();
assert_eq!(my_s, S { time: expected });
Ok::<(), serde_json::Error>(())
```

