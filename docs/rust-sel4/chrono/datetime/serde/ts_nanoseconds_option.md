**chrono > datetime > serde > ts_nanoseconds_option**

# Module: datetime::serde::ts_nanoseconds_option

## Contents

**Functions**

- [`deserialize`](#deserialize) - Deserialize a `DateTime` from a nanosecond timestamp or none
- [`serialize`](#serialize) - Serialize a UTC datetime into an integer number of nanoseconds since the epoch or none

---

## chrono::datetime::serde::ts_nanoseconds_option::deserialize

*Function*

Deserialize a `DateTime` from a nanosecond timestamp or none

Intended for use with `serde`s `deserialize_with` attribute.

# Example:

```rust
# use chrono::{DateTime, TimeZone, Utc};
# use serde_derive::Deserialize;
use chrono::serde::ts_nanoseconds_option::deserialize as from_nano_tsopt;
#[derive(Debug, PartialEq, Deserialize)]
struct S {
    #[serde(deserialize_with = "from_nano_tsopt")]
    time: Option<DateTime<Utc>>,
}

let my_s: S = serde_json::from_str(r#"{ "time": 1526522699918355733 }"#)?;
assert_eq!(my_s, S { time: Utc.timestamp_opt(1526522699, 918355733).single() });
# Ok::<(), serde_json::Error>(())
```

```rust
fn deserialize<'de, D>(d: D) -> Result<Option<crate::DateTime<crate::Utc>>, <D as >::Error>
```



## chrono::datetime::serde::ts_nanoseconds_option::serialize

*Function*

Serialize a UTC datetime into an integer number of nanoseconds since the epoch or none

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
use chrono::serde::ts_nanoseconds_option::serialize as to_nano_tsopt;
#[derive(Serialize)]
struct S {
    #[serde(serialize_with = "to_nano_tsopt")]
    time: Option<DateTime<Utc>>,
}

let my_s = S {
    time: Some(
        NaiveDate::from_ymd_opt(2018, 5, 17)
            .unwrap()
            .and_hms_nano_opt(02, 04, 59, 918355733)
            .unwrap()
            .and_utc(),
    ),
};
let as_string = serde_json::to_string(&my_s)?;
assert_eq!(as_string, r#"{"time":1526522699918355733}"#);
# Ok::<(), serde_json::Error>(())
```

```rust
fn serialize<S>(opt: &Option<crate::DateTime<crate::Utc>>, serializer: S) -> Result<<S as >::Ok, <S as >::Error>
```



