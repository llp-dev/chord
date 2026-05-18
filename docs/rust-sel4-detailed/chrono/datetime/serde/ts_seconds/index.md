*[chrono](../../../index.md) / [datetime](../../index.md) / [serde](../index.md) / [ts_seconds](index.md)*

---

# Module `ts_seconds`

Ser/de to/from timestamps in seconds

Intended for use with `serde`'s `with` attribute.

# Example:

```rust
use chrono::{TimeZone, DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use chrono::serde::ts_seconds;
#[derive(Deserialize, Serialize)]
struct S {
    #[serde(with = "ts_seconds")]
    time: DateTime<Utc>,
}

let time = Utc.with_ymd_and_hms(2015, 5, 15, 10, 0, 0).unwrap();
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
| [`serialize`](#serialize) | fn | Serialize a UTC datetime into an integer number of seconds since the epoch |
| [`deserialize`](#deserialize) | fn | Deserialize a `DateTime` from a seconds timestamp |

## Functions

### `serialize`

```rust
fn serialize<S>(dt: &crate::DateTime<crate::Utc>, serializer: S) -> Result<<S as >::Ok, <S as >::Error>
where
    S: ser::Serializer
```

Serialize a UTC datetime into an integer number of seconds since the epoch

Intended for use with `serde`s `serialize_with` attribute.

# Example:

```rust
use chrono::{TimeZone, DateTime, Utc};
use serde_derive::Serialize;
use chrono::serde::ts_seconds::serialize as to_ts;
#[derive(Serialize)]
struct S {
    #[serde(serialize_with = "to_ts")]
    time: DateTime<Utc>,
}

let my_s = S { time: Utc.with_ymd_and_hms(2015, 5, 15, 10, 0, 0).unwrap() };
let as_string = serde_json::to_string(&my_s)?;
assert_eq!(as_string, r#"{"time":1431684000}"#);
Ok::<(), serde_json::Error>(())
```

### `deserialize`

```rust
fn deserialize<'de, D>(d: D) -> Result<crate::DateTime<crate::Utc>, <D as >::Error>
where
    D: de::Deserializer<'de>
```

Deserialize a `DateTime` from a seconds timestamp

Intended for use with `serde`s `deserialize_with` attribute.

# Example:

```rust
use chrono::{DateTime, TimeZone, Utc};
use serde_derive::Deserialize;
use chrono::serde::ts_seconds::deserialize as from_ts;
#[derive(Debug, PartialEq, Deserialize)]
struct S {
    #[serde(deserialize_with = "from_ts")]
    time: DateTime<Utc>,
}

let my_s: S = serde_json::from_str(r#"{ "time": 1431684000 }"#)?;
assert_eq!(my_s, S { time: Utc.timestamp_opt(1431684000, 0).unwrap() });
Ok::<(), serde_json::Error>(())
```

