**chrono > naive > datetime > serde > ts_milliseconds**

# Module: naive::datetime::serde::ts_milliseconds

## Contents

**Functions**

- [`deserialize`](#deserialize) - Deserialize a `NaiveDateTime` from a milliseconds timestamp
- [`serialize`](#serialize) - Serialize a datetime into an integer number of milliseconds since the epoch

---

## chrono::naive::datetime::serde::ts_milliseconds::deserialize

*Function*

Deserialize a `NaiveDateTime` from a milliseconds timestamp

Intended for use with `serde`s `deserialize_with` attribute.

# Example:

```rust
# use chrono::{DateTime, NaiveDateTime};
# use serde_derive::Deserialize;
use chrono::naive::serde::ts_milliseconds::deserialize as from_milli_ts;
#[derive(Debug, PartialEq, Deserialize)]
struct S {
    #[serde(deserialize_with = "from_milli_ts")]
    time: NaiveDateTime,
}

let my_s: S = serde_json::from_str(r#"{ "time": 1526522699918 }"#)?;
let expected = DateTime::from_timestamp(1526522699, 918000000).unwrap().naive_utc();
assert_eq!(my_s, S { time: expected });

let my_s: S = serde_json::from_str(r#"{ "time": -1 }"#)?;
let expected = DateTime::from_timestamp(-1, 999_000_000).unwrap().naive_utc();
assert_eq!(my_s, S { time: expected });
# Ok::<(), serde_json::Error>(())
```

```rust
fn deserialize<'de, D>(d: D) -> Result<crate::NaiveDateTime, <D as >::Error>
```



## chrono::naive::datetime::serde::ts_milliseconds::serialize

*Function*

Serialize a datetime into an integer number of milliseconds since the epoch

Intended for use with `serde`s `serialize_with` attribute.

# Example:

```rust
# use chrono::{NaiveDate, NaiveDateTime};
# use serde_derive::Serialize;
use chrono::naive::serde::ts_milliseconds::serialize as to_milli_ts;
#[derive(Serialize)]
struct S {
    #[serde(serialize_with = "to_milli_ts")]
    time: NaiveDateTime,
}

let my_s = S {
    time: NaiveDate::from_ymd_opt(2018, 5, 17)
        .unwrap()
        .and_hms_milli_opt(02, 04, 59, 918)
        .unwrap(),
};
let as_string = serde_json::to_string(&my_s)?;
assert_eq!(as_string, r#"{"time":1526522699918}"#);
# Ok::<(), serde_json::Error>(())
```

```rust
fn serialize<S>(dt: &crate::NaiveDateTime, serializer: S) -> Result<<S as >::Ok, <S as >::Error>
```



