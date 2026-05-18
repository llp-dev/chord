**chrono > datetime > serde > ts_milliseconds_option**

# Module: datetime::serde::ts_milliseconds_option

## Contents

**Functions**

- [`deserialize`](#deserialize) - Deserialize a `DateTime` from a millisecond timestamp or none
- [`serialize`](#serialize) - Serialize a UTC datetime into an integer number of milliseconds since the epoch or none

---

## chrono::datetime::serde::ts_milliseconds_option::deserialize

*Function*

Deserialize a `DateTime` from a millisecond timestamp or none

Intended for use with `serde`s `deserialize_with` attribute.

# Example:

```rust
# use chrono::{TimeZone, DateTime, Utc};
# use serde_derive::Deserialize;
use chrono::serde::ts_milliseconds_option::deserialize as from_milli_tsopt;

#[derive(Deserialize, PartialEq, Debug)]
#[serde(untagged)]
enum E<T> {
    V(T),
}

#[derive(Deserialize, PartialEq, Debug)]
struct S {
    #[serde(default, deserialize_with = "from_milli_tsopt")]
    time: Option<DateTime<Utc>>,
}

let my_s: E<S> = serde_json::from_str(r#"{ "time": 1526522699918 }"#)?;
assert_eq!(my_s, E::V(S { time: Some(Utc.timestamp_opt(1526522699, 918000000).unwrap()) }));
let s: E<S> = serde_json::from_str(r#"{ "time": null }"#)?;
assert_eq!(s, E::V(S { time: None }));
let t: E<S> = serde_json::from_str(r#"{}"#)?;
assert_eq!(t, E::V(S { time: None }));
# Ok::<(), serde_json::Error>(())
```

```rust
fn deserialize<'de, D>(d: D) -> Result<Option<crate::DateTime<crate::Utc>>, <D as >::Error>
```



## chrono::datetime::serde::ts_milliseconds_option::serialize

*Function*

Serialize a UTC datetime into an integer number of milliseconds since the epoch or none

Intended for use with `serde`s `serialize_with` attribute.

# Example:

```rust
# use chrono::{DateTime, Utc, NaiveDate};
# use serde_derive::Serialize;
use chrono::serde::ts_milliseconds_option::serialize as to_milli_tsopt;
#[derive(Serialize)]
struct S {
    #[serde(serialize_with = "to_milli_tsopt")]
    time: Option<DateTime<Utc>>,
}

let my_s = S {
    time: Some(
        NaiveDate::from_ymd_opt(2018, 5, 17)
            .unwrap()
            .and_hms_milli_opt(02, 04, 59, 918)
            .unwrap()
            .and_utc(),
    ),
};
let as_string = serde_json::to_string(&my_s)?;
assert_eq!(as_string, r#"{"time":1526522699918}"#);
# Ok::<(), serde_json::Error>(())
```

```rust
fn serialize<S>(opt: &Option<crate::DateTime<crate::Utc>>, serializer: S) -> Result<<S as >::Ok, <S as >::Error>
```



