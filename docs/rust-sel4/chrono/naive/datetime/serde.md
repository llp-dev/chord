**chrono > naive > datetime > serde**

# Module: naive::datetime::serde

## Contents

**Modules**

- [`ts_microseconds`](#ts_microseconds) - Used to serialize/deserialize from microsecond-precision timestamps
- [`ts_microseconds_option`](#ts_microseconds_option) - Ser/de to/from optional timestamps in microseconds
- [`ts_milliseconds`](#ts_milliseconds) - Used to serialize/deserialize from millisecond-precision timestamps
- [`ts_milliseconds_option`](#ts_milliseconds_option) - Ser/de to/from optional timestamps in milliseconds
- [`ts_nanoseconds`](#ts_nanoseconds) - Used to serialize/deserialize from nanosecond-precision timestamps
- [`ts_nanoseconds_option`](#ts_nanoseconds_option) - Ser/de to/from optional timestamps in nanoseconds
- [`ts_seconds`](#ts_seconds) - Used to serialize/deserialize from second-precision timestamps
- [`ts_seconds_option`](#ts_seconds_option) - Ser/de to/from optional timestamps in seconds

---

## Module: ts_microseconds

Used to serialize/deserialize from microsecond-precision timestamps

# Example:

```rust
# use chrono::{NaiveDate, NaiveDateTime};
# use serde_derive::{Deserialize, Serialize};
use chrono::naive::serde::ts_microseconds;
#[derive(Deserialize, Serialize)]
struct S {
    #[serde(with = "ts_microseconds")]
    time: NaiveDateTime,
}

let time = NaiveDate::from_ymd_opt(2018, 5, 17)
    .unwrap()
    .and_hms_micro_opt(02, 04, 59, 918355)
    .unwrap();
let my_s = S { time: time.clone() };

let as_string = serde_json::to_string(&my_s)?;
assert_eq!(as_string, r#"{"time":1526522699918355}"#);
let my_s: S = serde_json::from_str(&as_string)?;
assert_eq!(my_s.time, time);
# Ok::<(), serde_json::Error>(())
```



## Module: ts_microseconds_option

Ser/de to/from optional timestamps in microseconds

Intended for use with `serde`'s `with` attribute.

# Example:

```rust
# use chrono::naive::{NaiveDate, NaiveDateTime};
# use serde_derive::{Deserialize, Serialize};
use chrono::naive::serde::ts_microseconds_option;
#[derive(Deserialize, Serialize)]
struct S {
    #[serde(with = "ts_microseconds_option")]
    time: Option<NaiveDateTime>,
}

let time = Some(
    NaiveDate::from_ymd_opt(2018, 5, 17)
        .unwrap()
        .and_hms_micro_opt(02, 04, 59, 918355)
        .unwrap(),
);
let my_s = S { time: time.clone() };

let as_string = serde_json::to_string(&my_s)?;
assert_eq!(as_string, r#"{"time":1526522699918355}"#);
let my_s: S = serde_json::from_str(&as_string)?;
assert_eq!(my_s.time, time);
# Ok::<(), serde_json::Error>(())
```



## Module: ts_milliseconds

Used to serialize/deserialize from millisecond-precision timestamps

# Example:

```rust
# use chrono::{NaiveDate, NaiveDateTime};
# use serde_derive::{Deserialize, Serialize};
use chrono::naive::serde::ts_milliseconds;
#[derive(Deserialize, Serialize)]
struct S {
    #[serde(with = "ts_milliseconds")]
    time: NaiveDateTime,
}

let time =
    NaiveDate::from_ymd_opt(2018, 5, 17).unwrap().and_hms_milli_opt(02, 04, 59, 918).unwrap();
let my_s = S { time: time.clone() };

let as_string = serde_json::to_string(&my_s)?;
assert_eq!(as_string, r#"{"time":1526522699918}"#);
let my_s: S = serde_json::from_str(&as_string)?;
assert_eq!(my_s.time, time);
# Ok::<(), serde_json::Error>(())
```



## Module: ts_milliseconds_option

Ser/de to/from optional timestamps in milliseconds

Intended for use with `serde`'s `with` attribute.

# Example:

```rust
# use chrono::naive::{NaiveDate, NaiveDateTime};
# use serde_derive::{Deserialize, Serialize};
use chrono::naive::serde::ts_milliseconds_option;
#[derive(Deserialize, Serialize)]
struct S {
    #[serde(with = "ts_milliseconds_option")]
    time: Option<NaiveDateTime>,
}

let time = Some(
    NaiveDate::from_ymd_opt(2018, 5, 17).unwrap().and_hms_milli_opt(02, 04, 59, 918).unwrap(),
);
let my_s = S { time: time.clone() };

let as_string = serde_json::to_string(&my_s)?;
assert_eq!(as_string, r#"{"time":1526522699918}"#);
let my_s: S = serde_json::from_str(&as_string)?;
assert_eq!(my_s.time, time);
# Ok::<(), serde_json::Error>(())
```



## Module: ts_nanoseconds

Used to serialize/deserialize from nanosecond-precision timestamps

# Example:

```rust
# use chrono::{NaiveDate, NaiveDateTime};
# use serde_derive::{Deserialize, Serialize};
use chrono::naive::serde::ts_nanoseconds;
#[derive(Deserialize, Serialize)]
struct S {
    #[serde(with = "ts_nanoseconds")]
    time: NaiveDateTime,
}

let time = NaiveDate::from_ymd_opt(2018, 5, 17)
    .unwrap()
    .and_hms_nano_opt(02, 04, 59, 918355733)
    .unwrap();
let my_s = S { time: time.clone() };

let as_string = serde_json::to_string(&my_s)?;
assert_eq!(as_string, r#"{"time":1526522699918355733}"#);
let my_s: S = serde_json::from_str(&as_string)?;
assert_eq!(my_s.time, time);
# Ok::<(), serde_json::Error>(())
```



## Module: ts_nanoseconds_option

Ser/de to/from optional timestamps in nanoseconds

Intended for use with `serde`'s `with` attribute.

# Example:

```rust
# use chrono::naive::{NaiveDate, NaiveDateTime};
# use serde_derive::{Deserialize, Serialize};
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
# Ok::<(), serde_json::Error>(())
```



## Module: ts_seconds

Used to serialize/deserialize from second-precision timestamps

# Example:

```rust
# use chrono::{NaiveDate, NaiveDateTime};
# use serde_derive::{Deserialize, Serialize};
use chrono::naive::serde::ts_seconds;
#[derive(Deserialize, Serialize)]
struct S {
    #[serde(with = "ts_seconds")]
    time: NaiveDateTime,
}

let time = NaiveDate::from_ymd_opt(2015, 5, 15).unwrap().and_hms_opt(10, 0, 0).unwrap();
let my_s = S { time: time.clone() };

let as_string = serde_json::to_string(&my_s)?;
assert_eq!(as_string, r#"{"time":1431684000}"#);
let my_s: S = serde_json::from_str(&as_string)?;
assert_eq!(my_s.time, time);
# Ok::<(), serde_json::Error>(())
```



## Module: ts_seconds_option

Ser/de to/from optional timestamps in seconds

Intended for use with `serde`'s `with` attribute.

# Example:

```rust
# use chrono::naive::{NaiveDate, NaiveDateTime};
# use serde_derive::{Deserialize, Serialize};
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
# Ok::<(), serde_json::Error>(())
```



