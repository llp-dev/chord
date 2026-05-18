**chrono > datetime > serde > ts_seconds**

# Module: datetime::serde::ts_seconds

## Contents

**Functions**

- [`deserialize`](#deserialize) - Deserialize a `DateTime` from a seconds timestamp
- [`serialize`](#serialize) - Serialize a UTC datetime into an integer number of seconds since the epoch

---

## chrono::datetime::serde::ts_seconds::deserialize

*Function*

Deserialize a `DateTime` from a seconds timestamp

Intended for use with `serde`s `deserialize_with` attribute.

# Example:

```rust
# use chrono::{DateTime, TimeZone, Utc};
# use serde_derive::Deserialize;
use chrono::serde::ts_seconds::deserialize as from_ts;
#[derive(Debug, PartialEq, Deserialize)]
struct S {
    #[serde(deserialize_with = "from_ts")]
    time: DateTime<Utc>,
}

let my_s: S = serde_json::from_str(r#"{ "time": 1431684000 }"#)?;
assert_eq!(my_s, S { time: Utc.timestamp_opt(1431684000, 0).unwrap() });
# Ok::<(), serde_json::Error>(())
```

```rust
fn deserialize<'de, D>(d: D) -> Result<crate::DateTime<crate::Utc>, <D as >::Error>
```



## chrono::datetime::serde::ts_seconds::serialize

*Function*

Serialize a UTC datetime into an integer number of seconds since the epoch

Intended for use with `serde`s `serialize_with` attribute.

# Example:

```rust
# use chrono::{TimeZone, DateTime, Utc};
# use serde_derive::Serialize;
use chrono::serde::ts_seconds::serialize as to_ts;
#[derive(Serialize)]
struct S {
    #[serde(serialize_with = "to_ts")]
    time: DateTime<Utc>,
}

let my_s = S { time: Utc.with_ymd_and_hms(2015, 5, 15, 10, 0, 0).unwrap() };
let as_string = serde_json::to_string(&my_s)?;
assert_eq!(as_string, r#"{"time":1431684000}"#);
# Ok::<(), serde_json::Error>(())
```

```rust
fn serialize<S>(dt: &crate::DateTime<crate::Utc>, serializer: S) -> Result<<S as >::Ok, <S as >::Error>
```



