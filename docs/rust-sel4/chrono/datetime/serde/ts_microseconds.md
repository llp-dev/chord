**chrono > datetime > serde > ts_microseconds**

# Module: datetime::serde::ts_microseconds

## Contents

**Functions**

- [`deserialize`](#deserialize) - Deserialize a `DateTime` from a microsecond timestamp
- [`serialize`](#serialize) - Serialize a UTC datetime into an integer number of microseconds since the epoch

---

## chrono::datetime::serde::ts_microseconds::deserialize

*Function*

Deserialize a `DateTime` from a microsecond timestamp

Intended for use with `serde`s `deserialize_with` attribute.

# Example:

```rust
# use chrono::{DateTime, TimeZone, Utc};
# use serde_derive::Deserialize;
use chrono::serde::ts_microseconds::deserialize as from_micro_ts;
#[derive(Debug, PartialEq, Deserialize)]
struct S {
    #[serde(deserialize_with = "from_micro_ts")]
    time: DateTime<Utc>,
}

let my_s: S = serde_json::from_str(r#"{ "time": 1526522699918355 }"#)?;
assert_eq!(my_s, S { time: Utc.timestamp_opt(1526522699, 918355000).unwrap() });

let my_s: S = serde_json::from_str(r#"{ "time": -1 }"#)?;
assert_eq!(my_s, S { time: Utc.timestamp_opt(-1, 999_999_000).unwrap() });
# Ok::<(), serde_json::Error>(())
```

```rust
fn deserialize<'de, D>(d: D) -> Result<crate::DateTime<crate::Utc>, <D as >::Error>
```



## chrono::datetime::serde::ts_microseconds::serialize

*Function*

Serialize a UTC datetime into an integer number of microseconds since the epoch

Intended for use with `serde`s `serialize_with` attribute.

# Example:

```rust
# use chrono::{DateTime, Utc, NaiveDate};
# use serde_derive::Serialize;
use chrono::serde::ts_microseconds::serialize as to_micro_ts;
#[derive(Serialize)]
struct S {
    #[serde(serialize_with = "to_micro_ts")]
    time: DateTime<Utc>,
}

let my_s = S {
    time: NaiveDate::from_ymd_opt(2018, 5, 17)
        .unwrap()
        .and_hms_micro_opt(02, 04, 59, 918355)
        .unwrap()
        .and_utc(),
};
let as_string = serde_json::to_string(&my_s)?;
assert_eq!(as_string, r#"{"time":1526522699918355}"#);
# Ok::<(), serde_json::Error>(())
```

```rust
fn serialize<S>(dt: &crate::DateTime<crate::Utc>, serializer: S) -> Result<<S as >::Ok, <S as >::Error>
```



