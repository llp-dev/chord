**chrono > datetime > serde > ts_milliseconds**

# Module: datetime::serde::ts_milliseconds

## Contents

**Functions**

- [`deserialize`](#deserialize) - Deserialize a `DateTime` from a millisecond timestamp
- [`serialize`](#serialize) - Serialize a UTC datetime into an integer number of milliseconds since the epoch

---

## chrono::datetime::serde::ts_milliseconds::deserialize

*Function*

Deserialize a `DateTime` from a millisecond timestamp

Intended for use with `serde`s `deserialize_with` attribute.

# Example:

```rust
# use chrono::{DateTime, TimeZone, Utc};
# use serde_derive::Deserialize;
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
# Ok::<(), serde_json::Error>(())
```

```rust
fn deserialize<'de, D>(d: D) -> Result<crate::DateTime<crate::Utc>, <D as >::Error>
```



## chrono::datetime::serde::ts_milliseconds::serialize

*Function*

Serialize a UTC datetime into an integer number of milliseconds since the epoch

Intended for use with `serde`s `serialize_with` attribute.

# Example:

```rust
# use chrono::{DateTime, Utc, NaiveDate};
# use serde_derive::Serialize;
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
# Ok::<(), serde_json::Error>(())
```

```rust
fn serialize<S>(dt: &crate::DateTime<crate::Utc>, serializer: S) -> Result<<S as >::Ok, <S as >::Error>
```



