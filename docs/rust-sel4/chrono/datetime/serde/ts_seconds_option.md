**chrono > datetime > serde > ts_seconds_option**

# Module: datetime::serde::ts_seconds_option

## Contents

**Functions**

- [`deserialize`](#deserialize) - Deserialize a `DateTime` from a seconds timestamp or none
- [`serialize`](#serialize) - Serialize a UTC datetime into an integer number of seconds since the epoch or none

---

## chrono::datetime::serde::ts_seconds_option::deserialize

*Function*

Deserialize a `DateTime` from a seconds timestamp or none

Intended for use with `serde`s `deserialize_with` attribute.

# Example:

```rust
# use chrono::{DateTime, TimeZone, Utc};
# use serde_derive::Deserialize;
use chrono::serde::ts_seconds_option::deserialize as from_tsopt;
#[derive(Debug, PartialEq, Deserialize)]
struct S {
    #[serde(deserialize_with = "from_tsopt")]
    time: Option<DateTime<Utc>>,
}

let my_s: S = serde_json::from_str(r#"{ "time": 1431684000 }"#)?;
assert_eq!(my_s, S { time: Utc.timestamp_opt(1431684000, 0).single() });
# Ok::<(), serde_json::Error>(())
```

```rust
fn deserialize<'de, D>(d: D) -> Result<Option<crate::DateTime<crate::Utc>>, <D as >::Error>
```



## chrono::datetime::serde::ts_seconds_option::serialize

*Function*

Serialize a UTC datetime into an integer number of seconds since the epoch or none

Intended for use with `serde`s `serialize_with` attribute.

# Example:

```rust
# use chrono::{TimeZone, DateTime, Utc};
# use serde_derive::Serialize;
use chrono::serde::ts_seconds_option::serialize as to_tsopt;
#[derive(Serialize)]
struct S {
    #[serde(serialize_with = "to_tsopt")]
    time: Option<DateTime<Utc>>,
}

let my_s = S { time: Some(Utc.with_ymd_and_hms(2015, 5, 15, 10, 0, 0).unwrap()) };
let as_string = serde_json::to_string(&my_s)?;
assert_eq!(as_string, r#"{"time":1431684000}"#);
# Ok::<(), serde_json::Error>(())
```

```rust
fn serialize<S>(opt: &Option<crate::DateTime<crate::Utc>>, serializer: S) -> Result<<S as >::Ok, <S as >::Error>
```



