*[chrono](../../../index.md) / [datetime](../../index.md) / [serde](../index.md) / [ts_milliseconds](index.md)*

---

# Module `ts_milliseconds`

Ser/de to/from timestamps in milliseconds

Intended for use with `serde`s `with` attribute.

# Example

```rust
use chrono::{DateTime, Utc, NaiveDate};
use serde_derive::{Deserialize, Serialize};
use chrono::serde::ts_milliseconds;
#[derive(Deserialize, Serialize)]
struct S {
    #[serde(with = "ts_milliseconds")]
    time: DateTime<Utc>,
}

let time = NaiveDate::from_ymd_opt(2018, 5, 17)
    .unwrap()
    .and_hms_milli_opt(02, 04, 59, 918)
    .unwrap()
    .and_utc();
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
| [`serialize`](#serialize) | fn | Serialize a UTC datetime into an integer number of milliseconds since the epoch |
| [`deserialize`](#deserialize) | fn | Deserialize a `DateTime` from a millisecond timestamp |

## Functions

### `serialize`

```rust
fn serialize<S>(dt: &crate::DateTime<crate::Utc>, serializer: S) -> Result<<S as >::Ok, <S as >::Error>
where
    S: ser::Serializer
```

Serialize a UTC datetime into an integer number of milliseconds since the epoch

Intended for use with `serde`s `serialize_with` attribute.

# Example:

```rust
use chrono::{DateTime, Utc, NaiveDate};
use serde_derive::Serialize;
use chrono::serde::ts_milliseconds::serialize as to_milli_ts;
#[derive(Serialize)]
struct S {
    #[serde(serialize_with = "to_milli_ts")]
    time: DateTime<Utc>,
}

let my_s = S {
    time: NaiveDate::from_ymd_opt(2018, 5, 17)
        .unwrap()
        .and_hms_milli_opt(02, 04, 59, 918)
        .unwrap()
        .and_utc(),
};
let as_string = serde_json::to_string(&my_s)?;
assert_eq!(as_string, r#"{"time":1526522699918}"#);
Ok::<(), serde_json::Error>(())
```

### `deserialize`

```rust
fn deserialize<'de, D>(d: D) -> Result<crate::DateTime<crate::Utc>, <D as >::Error>
where
    D: de::Deserializer<'de>
```

Deserialize a `DateTime` from a millisecond timestamp

Intended for use with `serde`s `deserialize_with` attribute.

# Example:

```rust
use chrono::{DateTime, TimeZone, Utc};
use serde_derive::Deserialize;
use chrono::serde::ts_milliseconds::deserialize as from_milli_ts;
#[derive(Debug, PartialEq, Deserialize)]
struct S {
    #[serde(deserialize_with = "from_milli_ts")]
    time: DateTime<Utc>,
}

let my_s: S = serde_json::from_str(r#"{ "time": 1526522699918 }"#)?;
assert_eq!(my_s, S { time: Utc.timestamp_opt(1526522699, 918000000).unwrap() });

let my_s: S = serde_json::from_str(r#"{ "time": -1 }"#)?;
assert_eq!(my_s, S { time: Utc.timestamp_opt(-1, 999_000_000).unwrap() });
Ok::<(), serde_json::Error>(())
```

