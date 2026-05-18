*[chrono](../../../../index.md) / [naive](../../../index.md) / [datetime](../../index.md) / [serde](../index.md) / [ts_seconds_option](index.md)*

---

# Module `ts_seconds_option`

Ser/de to/from optional timestamps in seconds

Intended for use with `serde`'s `with` attribute.

# Example:

```rust
use chrono::naive::{NaiveDate, NaiveDateTime};
use serde_derive::{Deserialize, Serialize};
use chrono::naive::serde::ts_seconds_option;
#[derive(Deserialize, Serialize)]
struct S {
    #[serde(with = "ts_seconds_option")]
    time: Option<NaiveDateTime>,
}

let time = Some(NaiveDate::from_ymd_opt(2018, 5, 17).unwrap().and_hms_opt(02, 04, 59).unwrap());
let my_s = S { time: time.clone() };

let as_string = serde_json::to_string(&my_s)?;
assert_eq!(as_string, r#"{"time":1526522699}"#);
let my_s: S = serde_json::from_str(&as_string)?;
assert_eq!(my_s.time, time);
Ok::<(), serde_json::Error>(())
```

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`OptionSecondsTimestampVisitor`](#optionsecondstimestampvisitor) | struct |  |
| [`serialize`](#serialize) | fn | Serialize a datetime into an integer number of seconds since the epoch or none |
| [`deserialize`](#deserialize) | fn | Deserialize a `NaiveDateTime` from a second timestamp or none |

## Structs

### `OptionSecondsTimestampVisitor`

```rust
struct OptionSecondsTimestampVisitor;
```

#### Trait Implementations

##### `impl Expected for OptionSecondsTimestampVisitor`

- <span id="optionsecondstimestampvisitor-expected-fmt"></span>`fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>`

##### `impl Visitor for OptionSecondsTimestampVisitor`

- <span id="optionsecondstimestampvisitor-visitor-type-value"></span>`type Value = Option<NaiveDateTime>`

- <span id="optionsecondstimestampvisitor-visitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="optionsecondstimestampvisitor-visitor-visit-some"></span>`fn visit_some<D>(self, d: D) -> Result<<Self as >::Value, <D as >::Error>`

  Deserialize a timestamp in seconds since the epoch

- <span id="optionsecondstimestampvisitor-visitor-visit-none"></span>`fn visit_none<E>(self) -> Result<<Self as >::Value, E>`

  Deserialize a timestamp in seconds since the epoch

- <span id="optionsecondstimestampvisitor-visitor-visit-unit"></span>`fn visit_unit<E>(self) -> Result<<Self as >::Value, E>`

  Deserialize a timestamp in seconds since the epoch

## Functions

### `serialize`

```rust
fn serialize<S>(opt: &Option<crate::NaiveDateTime>, serializer: S) -> Result<<S as >::Ok, <S as >::Error>
where
    S: ser::Serializer
```

Serialize a datetime into an integer number of seconds since the epoch or none

Intended for use with `serde`s `serialize_with` attribute.

# Example:

```rust
use chrono::naive::{NaiveDate, NaiveDateTime};
use serde_derive::Serialize;
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
Ok::<(), serde_json::Error>(())
```

### `deserialize`

```rust
fn deserialize<'de, D>(d: D) -> Result<Option<crate::NaiveDateTime>, <D as >::Error>
where
    D: de::Deserializer<'de>
```

Deserialize a `NaiveDateTime` from a second timestamp or none

Intended for use with `serde`s `deserialize_with` attribute.

# Example:

```rust
use chrono::{DateTime, NaiveDateTime};
use serde_derive::Deserialize;
use chrono::naive::serde::ts_seconds_option::deserialize as from_tsopt;
#[derive(Debug, PartialEq, Deserialize)]
struct S {
    #[serde(deserialize_with = "from_tsopt")]
    time: Option<NaiveDateTime>,
}

let my_s: S = serde_json::from_str(r#"{ "time": 1431684000 }"#)?;
let expected = DateTime::from_timestamp_secs(1431684000).unwrap().naive_utc();
assert_eq!(my_s, S { time: Some(expected) });
Ok::<(), serde_json::Error>(())
```

