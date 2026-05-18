**chrono > naive > datetime > serde > ts_seconds**

# Module: naive::datetime::serde::ts_seconds

## Contents

**Functions**

- [`deserialize`](#deserialize) - Deserialize a `NaiveDateTime` from a seconds timestamp
- [`serialize`](#serialize) - Serialize a datetime into an integer number of seconds since the epoch

---

## chrono::naive::datetime::serde::ts_seconds::deserialize

*Function*

Deserialize a `NaiveDateTime` from a seconds timestamp

Intended for use with `serde`s `deserialize_with` attribute.

# Example:

```rust
# use chrono::{DateTime, NaiveDateTime};
# use serde_derive::Deserialize;
use chrono::naive::serde::ts_seconds::deserialize as from_ts;
#[derive(Debug, PartialEq, Deserialize)]
struct S {
    #[serde(deserialize_with = "from_ts")]
    time: NaiveDateTime,
}

let my_s: S = serde_json::from_str(r#"{ "time": 1431684000 }"#)?;
let expected = DateTime::from_timestamp_secs(1431684000).unwrap().naive_utc();
assert_eq!(my_s, S { time: expected });
# Ok::<(), serde_json::Error>(())
```

```rust
fn deserialize<'de, D>(d: D) -> Result<crate::NaiveDateTime, <D as >::Error>
```



## chrono::naive::datetime::serde::ts_seconds::serialize

*Function*

Serialize a datetime into an integer number of seconds since the epoch

Intended for use with `serde`s `serialize_with` attribute.

# Example:

```rust
# use chrono::{NaiveDate, NaiveDateTime};
# use serde_derive::Serialize;
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
# Ok::<(), serde_json::Error>(())
```

```rust
fn serialize<S>(dt: &crate::NaiveDateTime, serializer: S) -> Result<<S as >::Ok, <S as >::Error>
```



