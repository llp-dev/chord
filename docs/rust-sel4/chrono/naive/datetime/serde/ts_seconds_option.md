**chrono > naive > datetime > serde > ts_seconds_option**

# Module: naive::datetime::serde::ts_seconds_option

## Contents

**Functions**

- [`deserialize`](#deserialize) - Deserialize a `NaiveDateTime` from a second timestamp or none
- [`serialize`](#serialize) - Serialize a datetime into an integer number of seconds since the epoch or none

---

## chrono::naive::datetime::serde::ts_seconds_option::deserialize

*Function*

Deserialize a `NaiveDateTime` from a second timestamp or none

Intended for use with `serde`s `deserialize_with` attribute.

# Example:

```rust
# use chrono::{DateTime, NaiveDateTime};
# use serde_derive::Deserialize;
use chrono::naive::serde::ts_seconds_option::deserialize as from_tsopt;
#[derive(Debug, PartialEq, Deserialize)]
struct S {
    #[serde(deserialize_with = "from_tsopt")]
    time: Option<NaiveDateTime>,
}

let my_s: S = serde_json::from_str(r#"{ "time": 1431684000 }"#)?;
let expected = DateTime::from_timestamp_secs(1431684000).unwrap().naive_utc();
assert_eq!(my_s, S { time: Some(expected) });
# Ok::<(), serde_json::Error>(())
```

```rust
fn deserialize<'de, D>(d: D) -> Result<Option<crate::NaiveDateTime>, <D as >::Error>
```



## chrono::naive::datetime::serde::ts_seconds_option::serialize

*Function*

Serialize a datetime into an integer number of seconds since the epoch or none

Intended for use with `serde`s `serialize_with` attribute.

# Example:

```rust
# use chrono::naive::{NaiveDate, NaiveDateTime};
# use serde_derive::Serialize;
use chrono::naive::serde::ts_seconds_option::serialize as to_tsopt;
#[derive(Serialize)]
struct S {
    #[serde(serialize_with = "to_tsopt")]
    time: Option<NaiveDateTime>,
}

let expected = NaiveDate::from_ymd_opt(2018, 5, 17).unwrap().and_hms_opt(02, 04, 59).unwrap();
let my_s = S { time: Some(expected) };
let as_string = serde_json::to_string(&my_s)?;
assert_eq!(as_string, r#"{"time":1526522699}"#);
# Ok::<(), serde_json::Error>(())
```

```rust
fn serialize<S>(opt: &Option<crate::NaiveDateTime>, serializer: S) -> Result<<S as >::Ok, <S as >::Error>
```



