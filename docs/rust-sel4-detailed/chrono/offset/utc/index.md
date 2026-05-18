*[chrono](../../index.md) / [offset](../index.md) / [utc](index.md)*

---

# Module `utc`

The UTC (Coordinated Universal Time) time zone.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Utc`](#utc) | struct | The UTC time zone. |

## Structs

### `Utc`

```rust
struct Utc;
```

The UTC time zone. This is the most efficient time zone when you don't need the local time.
It is also used as an offset (which is also a dummy type).

Using the [`TimeZone`](./trait.TimeZone.html) methods
on the UTC struct is the preferred way to construct `DateTime<Utc>`
instances.

# Example

```rust
use chrono::{DateTime, TimeZone, Utc};

let dt = DateTime::from_timestamp(61, 0).unwrap();

assert_eq!(Utc.timestamp_opt(61, 0).unwrap(), dt);
assert_eq!(Utc.with_ymd_and_hms(1970, 1, 1, 0, 1, 1).unwrap(), dt);
```

#### Trait Implementations

##### `impl Clone for Utc`

- <span id="utc-clone"></span>`fn clone(&self) -> Utc` — [`Utc`](#utc)

##### `impl Copy for Utc`

##### `impl Debug for Utc`

- <span id="utc-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Utc`

- <span id="utc-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Utc`

##### `impl Hash for Utc`

- <span id="utc-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Offset for Utc`

- <span id="utc-offset-fix"></span>`fn fix(&self) -> FixedOffset` — [`FixedOffset`](../fixed/index.md#fixedoffset)

##### `impl PartialEq for Utc`

- <span id="utc-partialeq-eq"></span>`fn eq(&self, other: &Utc) -> bool` — [`Utc`](#utc)

##### `impl StructuralPartialEq for Utc`

##### `impl TimeZone for Utc`

- <span id="utc-timezone-type-offset"></span>`type Offset = Utc`

- <span id="utc-timezone-from-offset"></span>`fn from_offset(_state: &Utc) -> Utc` — [`Utc`](#utc)

- <span id="utc-timezone-offset-from-local-date"></span>`fn offset_from_local_date(&self, _local: &NaiveDate) -> MappedLocalTime<Utc>` — [`NaiveDate`](../../naive/date/index.md#naivedate), [`MappedLocalTime`](../index.md#mappedlocaltime), [`Utc`](#utc)

- <span id="utc-timezone-offset-from-local-datetime"></span>`fn offset_from_local_datetime(&self, _local: &NaiveDateTime) -> MappedLocalTime<Utc>` — [`NaiveDateTime`](../../naive/datetime/index.md#naivedatetime), [`MappedLocalTime`](../index.md#mappedlocaltime), [`Utc`](#utc)

- <span id="utc-timezone-offset-from-utc-date"></span>`fn offset_from_utc_date(&self, _utc: &NaiveDate) -> Utc` — [`NaiveDate`](../../naive/date/index.md#naivedate), [`Utc`](#utc)

- <span id="utc-timezone-offset-from-utc-datetime"></span>`fn offset_from_utc_datetime(&self, _utc: &NaiveDateTime) -> Utc` — [`NaiveDateTime`](../../naive/datetime/index.md#naivedatetime), [`Utc`](#utc)

##### `impl ToString for Utc`

- <span id="utc-tostring-to-string"></span>`fn to_string(&self) -> String`

