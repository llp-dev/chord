**chrono > naive > datetime > serde > ts_nanoseconds**

# Module: naive::datetime::serde::ts_nanoseconds

## Contents

**Functions**

- [`deserialize`](#deserialize) - Deserialize a `NaiveDateTime` from a nanoseconds timestamp
- [`serialize`](#serialize) - Serialize a datetime into an integer number of nanoseconds since the epoch

---

## chrono::naive::datetime::serde::ts_nanoseconds::deserialize

*Function*

Deserialize a `NaiveDateTime` from a nanoseconds timestamp

Intended for use with `serde`s `deserialize_with` attribute.

# Example:

```rust
# use chrono::{DateTime, NaiveDateTime};
# use serde_derive::Deserialize;
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
# Ok::<(), serde_json::Error>(())
```

```rust
fn deserialize<'de, D>(d: D) -> Result<crate::NaiveDateTime, <D as >::Error>
```



## chrono::naive::datetime::serde::ts_nanoseconds::serialize

*Function*

Serialize a datetime into an integer number of nanoseconds since the epoch

Intended for use with `serde`s `serialize_with` attribute.

# Errors

An `i64` with nanosecond precision can span a range of ~584 years. This function returns an
error on an out of range `DateTime`.

The dates that can be represented as nanoseconds are between 1677-09-21T00:12:44.0 and
2262-04-11T23:47:16.854775804.

# Example:

```rust
# use chrono::{NaiveDate, NaiveDateTime};
# use serde_derive::Serialize;
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
# Ok::<(), serde_json::Error>(())
```

```rust
fn serialize<S>(dt: &crate::NaiveDateTime, serializer: S) -> Result<<S as >::Ok, <S as >::Error>
```



