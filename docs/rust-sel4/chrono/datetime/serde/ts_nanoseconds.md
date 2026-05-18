**chrono > datetime > serde > ts_nanoseconds**

# Module: datetime::serde::ts_nanoseconds

## Contents

**Functions**

- [`deserialize`](#deserialize) - Deserialize a [`DateTime`] from a nanosecond timestamp
- [`serialize`](#serialize) - Serialize a UTC datetime into an integer number of nanoseconds since the epoch

---

## chrono::datetime::serde::ts_nanoseconds::deserialize

*Function*

Deserialize a [`DateTime`] from a nanosecond timestamp

Intended for use with `serde`s `deserialize_with` attribute.

# Example:

```rust
# use chrono::{DateTime, TimeZone, Utc};
# use serde_derive::Deserialize;
use chrono::serde::ts_nanoseconds::deserialize as from_nano_ts;
#[derive(Debug, PartialEq, Deserialize)]
struct S {
    #[serde(deserialize_with = "from_nano_ts")]
    time: DateTime<Utc>,
}

let my_s: S = serde_json::from_str(r#"{ "time": 1526522699918355733 }"#)?;
assert_eq!(my_s, S { time: Utc.timestamp_opt(1526522699, 918355733).unwrap() });

let my_s: S = serde_json::from_str(r#"{ "time": -1 }"#)?;
assert_eq!(my_s, S { time: Utc.timestamp_opt(-1, 999_999_999).unwrap() });
# Ok::<(), serde_json::Error>(())
```

```rust
fn deserialize<'de, D>(d: D) -> Result<crate::DateTime<crate::Utc>, <D as >::Error>
```



## chrono::datetime::serde::ts_nanoseconds::serialize

*Function*

Serialize a UTC datetime into an integer number of nanoseconds since the epoch

Intended for use with `serde`s `serialize_with` attribute.

# Errors

An `i64` with nanosecond precision can span a range of ~584 years. This function returns an
error on an out of range `DateTime`.

The dates that can be represented as nanoseconds are between 1677-09-21T00:12:44.0 and
2262-04-11T23:47:16.854775804.

# Example:

```rust
# use chrono::{DateTime, Utc, NaiveDate};
# use serde_derive::Serialize;
use chrono::serde::ts_nanoseconds::serialize as to_nano_ts;
#[derive(Serialize)]
struct S {
    #[serde(serialize_with = "to_nano_ts")]
    time: DateTime<Utc>,
}

let my_s = S {
    time: NaiveDate::from_ymd_opt(2018, 5, 17)
        .unwrap()
        .and_hms_nano_opt(02, 04, 59, 918355733)
        .unwrap()
        .and_utc(),
};
let as_string = serde_json::to_string(&my_s)?;
assert_eq!(as_string, r#"{"time":1526522699918355733}"#);
# Ok::<(), serde_json::Error>(())
```

```rust
fn serialize<S>(dt: &crate::DateTime<crate::Utc>, serializer: S) -> Result<<S as >::Ok, <S as >::Error>
```



