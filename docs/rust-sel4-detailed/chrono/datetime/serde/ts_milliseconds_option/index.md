*[chrono](../../../index.md) / [datetime](../../index.md) / [serde](../index.md) / [ts_milliseconds_option](index.md)*

---

# Module `ts_milliseconds_option`

Ser/de to/from optional timestamps in milliseconds

Intended for use with `serde`s `with` attribute.

# Example

```rust
use chrono::{DateTime, Utc, NaiveDate};
use serde_derive::{Deserialize, Serialize};
use chrono::serde::ts_milliseconds_option;
#[derive(Deserialize, Serialize)]
struct S {
    #[serde(with = "ts_milliseconds_option")]
    time: Option<DateTime<Utc>>,
}

let time = Some(
    NaiveDate::from_ymd_opt(2018, 5, 17)
        .unwrap()
        .and_hms_milli_opt(02, 04, 59, 918)
        .unwrap()
        .and_utc(),
);
let my_s = S { time: time.clone() };

let as_string = serde_json::to_string(&my_s)?;
assert_eq!(as_string, r#"{"time":1526522699918}"#);
let my_s: S = serde_json::from_str(&as_string)?;
assert_eq!(my_s.time, time);
Ok::<(), serde_json::Error>(())
```

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`OptionMilliSecondsTimestampVisitor`](#optionmillisecondstimestampvisitor) | struct |  |
| [`serialize`](#serialize) | fn | Serialize a UTC datetime into an integer number of milliseconds since the epoch or none |
| [`deserialize`](#deserialize) | fn | Deserialize a `DateTime` from a millisecond timestamp or none |

## Structs

### `OptionMilliSecondsTimestampVisitor`

```rust
struct OptionMilliSecondsTimestampVisitor;
```

#### Trait Implementations

##### `impl Expected for OptionMilliSecondsTimestampVisitor`

- <span id="optionmillisecondstimestampvisitor-expected-fmt"></span>`fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>`

##### `impl Visitor for OptionMilliSecondsTimestampVisitor`

- <span id="optionmillisecondstimestampvisitor-visitor-type-value"></span>`type Value = Option<DateTime<Utc>>`

- <span id="optionmillisecondstimestampvisitor-visitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="optionmillisecondstimestampvisitor-visitor-visit-some"></span>`fn visit_some<D>(self, d: D) -> Result<<Self as >::Value, <D as >::Error>`

  Deserialize a timestamp in milliseconds since the epoch

- <span id="optionmillisecondstimestampvisitor-visitor-visit-none"></span>`fn visit_none<E>(self) -> Result<<Self as >::Value, E>`

  Deserialize a timestamp in milliseconds since the epoch

- <span id="optionmillisecondstimestampvisitor-visitor-visit-unit"></span>`fn visit_unit<E>(self) -> Result<<Self as >::Value, E>`

  Deserialize a timestamp in milliseconds since the epoch

## Functions

### `serialize`

```rust
fn serialize<S>(opt: &Option<crate::DateTime<crate::Utc>>, serializer: S) -> Result<<S as >::Ok, <S as >::Error>
where
    S: ser::Serializer
```

Serialize a UTC datetime into an integer number of milliseconds since the epoch or none

Intended for use with `serde`s `serialize_with` attribute.

# Example:

```rust
use chrono::{DateTime, Utc, NaiveDate};
use serde_derive::Serialize;
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
Ok::<(), serde_json::Error>(())
```

### `deserialize`

```rust
fn deserialize<'de, D>(d: D) -> Result<Option<crate::DateTime<crate::Utc>>, <D as >::Error>
where
    D: de::Deserializer<'de>
```

Deserialize a `DateTime` from a millisecond timestamp or none

Intended for use with `serde`s `deserialize_with` attribute.

# Example:

```rust
use chrono::{TimeZone, DateTime, Utc};
use serde_derive::Deserialize;
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
Ok::<(), serde_json::Error>(())
```

