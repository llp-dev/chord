*[chrono](../../../index.md) / [datetime](../../index.md) / [serde](../index.md) / [ts_microseconds_option](index.md)*

---

# Module `ts_microseconds_option`

Ser/de to/from optional timestamps in microseconds

Intended for use with `serde`'s `with` attribute.

# Example:

```rust
use chrono::{DateTime, Utc, NaiveDate};
use serde_derive::{Deserialize, Serialize};
use chrono::serde::ts_microseconds_option;
#[derive(Deserialize, Serialize)]
struct S {
    #[serde(with = "ts_microseconds_option")]
    time: Option<DateTime<Utc>>,
}

let time = Some(
    NaiveDate::from_ymd_opt(2018, 5, 17)
        .unwrap()
        .and_hms_micro_opt(02, 04, 59, 918355)
        .unwrap()
        .and_utc(),
);
let my_s = S { time: time.clone() };

let as_string = serde_json::to_string(&my_s)?;
assert_eq!(as_string, r#"{"time":1526522699918355}"#);
let my_s: S = serde_json::from_str(&as_string)?;
assert_eq!(my_s.time, time);
Ok::<(), serde_json::Error>(())
```

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`OptionMicroSecondsTimestampVisitor`](#optionmicrosecondstimestampvisitor) | struct |  |
| [`serialize`](#serialize) | fn | Serialize a UTC datetime into an integer number of microseconds since the epoch or none |
| [`deserialize`](#deserialize) | fn | Deserialize a `DateTime` from a microsecond timestamp or none |

## Structs

### `OptionMicroSecondsTimestampVisitor`

```rust
struct OptionMicroSecondsTimestampVisitor;
```

#### Trait Implementations

##### `impl Expected for OptionMicroSecondsTimestampVisitor`

- <span id="optionmicrosecondstimestampvisitor-expected-fmt"></span>`fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error>`

##### `impl Visitor for OptionMicroSecondsTimestampVisitor`

- <span id="optionmicrosecondstimestampvisitor-visitor-type-value"></span>`type Value = Option<DateTime<Utc>>`

- <span id="optionmicrosecondstimestampvisitor-visitor-expecting"></span>`fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result`

- <span id="optionmicrosecondstimestampvisitor-visitor-visit-some"></span>`fn visit_some<D>(self, d: D) -> Result<<Self as >::Value, <D as >::Error>`

  Deserialize a timestamp in microseconds since the epoch

- <span id="optionmicrosecondstimestampvisitor-visitor-visit-none"></span>`fn visit_none<E>(self) -> Result<<Self as >::Value, E>`

  Deserialize a timestamp in microseconds since the epoch

- <span id="optionmicrosecondstimestampvisitor-visitor-visit-unit"></span>`fn visit_unit<E>(self) -> Result<<Self as >::Value, E>`

  Deserialize a timestamp in microseconds since the epoch

## Functions

### `serialize`

```rust
fn serialize<S>(opt: &Option<crate::DateTime<crate::Utc>>, serializer: S) -> Result<<S as >::Ok, <S as >::Error>
where
    S: ser::Serializer
```

Serialize a UTC datetime into an integer number of microseconds since the epoch or none

Intended for use with `serde`s `serialize_with` attribute.

# Example:

```rust
use chrono::{DateTime, Utc, NaiveDate};
use serde_derive::Serialize;
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
Ok::<(), serde_json::Error>(())
```

### `deserialize`

```rust
fn deserialize<'de, D>(d: D) -> Result<Option<crate::DateTime<crate::Utc>>, <D as >::Error>
where
    D: de::Deserializer<'de>
```

Deserialize a `DateTime` from a microsecond timestamp or none

Intended for use with `serde`s `deserialize_with` attribute.

# Example:

```rust
use chrono::{DateTime, TimeZone, Utc};
use serde_derive::Deserialize;
use chrono::serde::ts_microseconds_option::deserialize as from_micro_tsopt;
#[derive(Debug, PartialEq, Deserialize)]
struct S {
    #[serde(deserialize_with = "from_micro_tsopt")]
    time: Option<DateTime<Utc>>,
}

let my_s: S = serde_json::from_str(r#"{ "time": 1526522699918355 }"#)?;
assert_eq!(my_s, S { time: Utc.timestamp_opt(1526522699, 918355000).single() });
Ok::<(), serde_json::Error>(())
```

