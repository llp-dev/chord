*[chrono](../../../../index.md) / [naive](../../../index.md) / [datetime](../../index.md) / [serde](../index.md) / [ts_nanoseconds_option](index.md)*

---

# Module `ts_nanoseconds_option`

Ser/de to/from optional timestamps in nanoseconds

Intended for use with `serde`'s `with` attribute.

# Example:

```rust
use chrono::naive::{NaiveDate, NaiveDateTime};
use serde_derive::{Deserialize, Serialize};
use chrono::naive::serde::ts_nanoseconds_option;
#[derive(Deserialize, Serialize)]
struct S {
    #[serde(with = "ts_nanoseconds_option")]
    time: Option<NaiveDateTime>,
}

let time = Some(
    NaiveDate::from_ymd_opt(2018, 5, 17)
        .unwrap()
        .and_hms_nano_opt(02, 04, 59, 918355733)
        .unwrap(),
);
let my_s = S { time: time.clone() };

let as_string = serde_json::to_string(&my_s)?;
assert_eq!(as_string, r#"{"time":1526522699918355733}"#);
let my_s: S = serde_json::from_str(&as_string)?;
assert_eq!(my_s.time, time);
Ok::<(), serde_json::Error>(())
```

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`OptionNanoSecondsTimestampVisitor`](#optionnanosecondstimestampvisitor) | struct |  |
| [`serialize`](#serialize) | fn | Serialize a datetime into an integer number of nanoseconds since the epoch or none |
| [`deserialize`](#deserialize) | fn | Deserialize a `NaiveDateTime` from a nanosecond timestamp or none |

## Structs

### `OptionNanoSecondsTimestampVisitor`

```rust
struct OptionNanoSecondsTimestampVisitor;
```

#### Trait Implementations

##### `impl Expected for OptionNanoSecondsTimestampVisitor`

- <span id="optionnanosecondstimestampvisitor-expected-fmt"></span>`fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>`

##### `impl Visitor for OptionNanoSecondsTimestampVisitor`

- <span id="optionnanosecondstimestampvisitor-visitor-type-value"></span>`type Value = Option<NaiveDateTime>`

- <span id="optionnanosecondstimestampvisitor-visitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="optionnanosecondstimestampvisitor-visitor-visit-some"></span>`fn visit_some<D>(self, d: D) -> Result<<Self as >::Value, <D as >::Error>`

  Deserialize a timestamp in nanoseconds since the epoch

- <span id="optionnanosecondstimestampvisitor-visitor-visit-none"></span>`fn visit_none<E>(self) -> Result<<Self as >::Value, E>`

  Deserialize a timestamp in nanoseconds since the epoch

- <span id="optionnanosecondstimestampvisitor-visitor-visit-unit"></span>`fn visit_unit<E>(self) -> Result<<Self as >::Value, E>`

  Deserialize a timestamp in nanoseconds since the epoch

## Functions

### `serialize`

```rust
fn serialize<S>(opt: &Option<crate::NaiveDateTime>, serializer: S) -> Result<<S as >::Ok, <S as >::Error>
where
    S: ser::Serializer
```

Serialize a datetime into an integer number of nanoseconds since the epoch or none

Intended for use with `serde`s `serialize_with` attribute.

# Errors

An `i64` with nanosecond precision can span a range of ~584 years. This function returns an
error on an out of range `DateTime`.

The dates that can be represented as nanoseconds are between 1677-09-21T00:12:44.0 and
2262-04-11T23:47:16.854775804.

# Example:

```rust
use chrono::naive::{NaiveDate, NaiveDateTime};
use serde_derive::Serialize;
use chrono::naive::serde::ts_nanoseconds_option::serialize as to_nano_tsopt;
#[derive(Serialize)]
struct S {
    #[serde(serialize_with = "to_nano_tsopt")]
    time: Option<NaiveDateTime>,
}

let my_s = S {
    time: Some(
        NaiveDate::from_ymd_opt(2018, 5, 17)
            .unwrap()
            .and_hms_nano_opt(02, 04, 59, 918355733)
            .unwrap(),
    ),
};
let as_string = serde_json::to_string(&my_s)?;
assert_eq!(as_string, r#"{"time":1526522699918355733}"#);
Ok::<(), serde_json::Error>(())
```

### `deserialize`

```rust
fn deserialize<'de, D>(d: D) -> Result<Option<crate::NaiveDateTime>, <D as >::Error>
where
    D: de::Deserializer<'de>
```

Deserialize a `NaiveDateTime` from a nanosecond timestamp or none

Intended for use with `serde`s `deserialize_with` attribute.

# Example:

```rust
use chrono::{DateTime, NaiveDateTime};
use serde_derive::Deserialize;
use chrono::naive::serde::ts_nanoseconds_option::deserialize as from_nano_tsopt;
#[derive(Debug, PartialEq, Deserialize)]
struct S {
    #[serde(deserialize_with = "from_nano_tsopt")]
    time: Option<NaiveDateTime>,
}

let my_s: S = serde_json::from_str(r#"{ "time": 1526522699918355733 }"#)?;
let expected = DateTime::from_timestamp(1526522699, 918355733).unwrap().naive_utc();
assert_eq!(my_s, S { time: Some(expected) });

let my_s: S = serde_json::from_str(r#"{ "time": -1 }"#)?;
let expected = DateTime::from_timestamp(-1, 999_999_999).unwrap().naive_utc();
assert_eq!(my_s, S { time: Some(expected) });
Ok::<(), serde_json::Error>(())
```

