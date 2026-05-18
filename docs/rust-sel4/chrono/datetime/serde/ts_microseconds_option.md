**chrono > datetime > serde > ts_microseconds_option**

# Module: datetime::serde::ts_microseconds_option

## Contents

**Functions**

- [`deserialize`](#deserialize) - Deserialize a `DateTime` from a microsecond timestamp or none
- [`serialize`](#serialize) - Serialize a UTC datetime into an integer number of microseconds since the epoch or none

---

## chrono::datetime::serde::ts_microseconds_option::deserialize

*Function*

Deserialize a `DateTime` from a microsecond timestamp or none

Intended for use with `serde`s `deserialize_with` attribute.

# Example:

```rust
# use chrono::{DateTime, TimeZone, Utc};
# use serde_derive::Deserialize;
use chrono::serde::ts_microseconds_option::deserialize as from_micro_tsopt;
#[derive(Debug, PartialEq, Deserialize)]
struct S {
    #[serde(deserialize_with = "from_micro_tsopt")]
    time: Option<DateTime<Utc>>,
}

let my_s: S = serde_json::from_str(r#"{ "time": 1526522699918355 }"#)?;
assert_eq!(my_s, S { time: Utc.timestamp_opt(1526522699, 918355000).single() });
# Ok::<(), serde_json::Error>(())
```

```rust
fn deserialize<'de, D>(d: D) -> Result<Option<crate::DateTime<crate::Utc>>, <D as >::Error>
```



## chrono::datetime::serde::ts_microseconds_option::serialize

*Function*

Serialize a UTC datetime into an integer number of microseconds since the epoch or none

Intended for use with `serde`s `serialize_with` attribute.

# Example:

```rust
# use chrono::{DateTime, Utc, NaiveDate};
# use serde_derive::Serialize;
use chrono::serde::ts_microseconds_option::serialize as to_micro_tsopt;
#[derive(Serialize)]
struct S {
    #[serde(serialize_with = "to_micro_tsopt")]
    time: Option<DateTime<Utc>>,
}

let my_s = S {
    time: Some(
        NaiveDate::from_ymd_opt(2018, 5, 17)
            .unwrap()
            .and_hms_micro_opt(02, 04, 59, 918355)
            .unwrap()
            .and_utc(),
    ),
};
let as_string = serde_json::to_string(&my_s)?;
assert_eq!(as_string, r#"{"time":1526522699918355}"#);
# Ok::<(), serde_json::Error>(())
```

```rust
fn serialize<S>(opt: &Option<crate::DateTime<crate::Utc>>, serializer: S) -> Result<<S as >::Ok, <S as >::Error>
```



