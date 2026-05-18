*[chrono](../../index.md) / [offset](../index.md) / [fixed](index.md)*

---

# Module `fixed`

The time zone which has a fixed offset from UTC.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`FixedOffset`](#fixedoffset) | struct | The time zone with fixed offset, from UTC-23:59:59 to UTC+23:59:59. |

## Structs

### `FixedOffset`

```rust
struct FixedOffset {
    local_minus_utc: i32,
}
```

The time zone with fixed offset, from UTC-23:59:59 to UTC+23:59:59.

Using the [`TimeZone`](./trait.TimeZone.html) methods
on a `FixedOffset` struct is the preferred way to construct
`DateTime<FixedOffset>` instances. See the [`east_opt`](#method.east_opt) and
[`west_opt`](#method.west_opt) methods for examples.

#### Implementations

- <span id="fixedoffset-east"></span>`fn east(secs: i32) -> FixedOffset` — [`FixedOffset`](#fixedoffset)

  Makes a new `FixedOffset` for the Eastern Hemisphere with given timezone difference.

  The negative `secs` means the Western Hemisphere.

  

  Panics on the out-of-bound `secs`.

- <span id="fixedoffset-east-opt"></span>`const fn east_opt(secs: i32) -> Option<FixedOffset>` — [`FixedOffset`](#fixedoffset)

  Makes a new `FixedOffset` for the Eastern Hemisphere with given timezone difference.

  The negative `secs` means the Western Hemisphere.

  

  Returns `None` on the out-of-bound `secs`.

  

  # Example

  

  ```rust

  #[cfg(feature = "alloc")] {

  use chrono::{FixedOffset, TimeZone};

  let hour = 3600;

  let datetime =

      FixedOffset::east_opt(5 * hour).unwrap().with_ymd_and_hms(2016, 11, 08, 0, 0, 0).unwrap();

  assert_eq!(&datetime.to_rfc3339(), "2016-11-08T00:00:00+05:00")

  }

  ```

- <span id="fixedoffset-west"></span>`fn west(secs: i32) -> FixedOffset` — [`FixedOffset`](#fixedoffset)

  Makes a new `FixedOffset` for the Western Hemisphere with given timezone difference.

  The negative `secs` means the Eastern Hemisphere.

  

  Panics on the out-of-bound `secs`.

- <span id="fixedoffset-west-opt"></span>`const fn west_opt(secs: i32) -> Option<FixedOffset>` — [`FixedOffset`](#fixedoffset)

  Makes a new `FixedOffset` for the Western Hemisphere with given timezone difference.

  The negative `secs` means the Eastern Hemisphere.

  

  Returns `None` on the out-of-bound `secs`.

  

  # Example

  

  ```rust

  #[cfg(feature = "alloc")] {

  use chrono::{FixedOffset, TimeZone};

  let hour = 3600;

  let datetime =

      FixedOffset::west_opt(5 * hour).unwrap().with_ymd_and_hms(2016, 11, 08, 0, 0, 0).unwrap();

  assert_eq!(&datetime.to_rfc3339(), "2016-11-08T00:00:00-05:00")

  }

  ```

- <span id="fixedoffset-local-minus-utc"></span>`const fn local_minus_utc(&self) -> i32`

  Returns the number of seconds to add to convert from UTC to the local time.

- <span id="fixedoffset-utc-minus-local"></span>`const fn utc_minus_local(&self) -> i32`

  Returns the number of seconds to add to convert from the local time to UTC.

#### Trait Implementations

##### `impl<Tz: TimeZone> Add for DateTime<Tz>`

- <span id="datetime-add-type-output"></span>`type Output = DateTime<Tz>`

- <span id="datetime-add"></span>`fn add(self, rhs: FixedOffset) -> DateTime<Tz>` — [`FixedOffset`](#fixedoffset), [`DateTime`](../../datetime/index.md#datetime)

##### `impl Clone for FixedOffset`

- <span id="fixedoffset-clone"></span>`fn clone(&self) -> FixedOffset` — [`FixedOffset`](#fixedoffset)

##### `impl Copy for FixedOffset`

##### `impl Debug for FixedOffset`

- <span id="fixedoffset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for FixedOffset`

- <span id="fixedoffset-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for FixedOffset`

##### `impl FromStr for FixedOffset`

- <span id="fixedoffset-fromstr-type-err"></span>`type Err = ParseError`

- <span id="fixedoffset-fromstr-from-str"></span>`fn from_str(s: &str) -> Result<Self, <Self as >::Err>`

##### `impl Hash for FixedOffset`

- <span id="fixedoffset-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Offset for FixedOffset`

- <span id="fixedoffset-offset-fix"></span>`fn fix(&self) -> FixedOffset` — [`FixedOffset`](#fixedoffset)

##### `impl PartialEq for FixedOffset`

- <span id="fixedoffset-partialeq-eq"></span>`fn eq(&self, other: &FixedOffset) -> bool` — [`FixedOffset`](#fixedoffset)

##### `impl StructuralPartialEq for FixedOffset`

##### `impl<Tz: TimeZone> Sub for DateTime<Tz>`

- <span id="datetime-sub-type-output"></span>`type Output = DateTime<Tz>`

- <span id="datetime-sub"></span>`fn sub(self, rhs: FixedOffset) -> DateTime<Tz>` — [`FixedOffset`](#fixedoffset), [`DateTime`](../../datetime/index.md#datetime)

##### `impl TimeZone for FixedOffset`

- <span id="fixedoffset-timezone-type-offset"></span>`type Offset = FixedOffset`

- <span id="fixedoffset-timezone-from-offset"></span>`fn from_offset(offset: &FixedOffset) -> FixedOffset` — [`FixedOffset`](#fixedoffset)

- <span id="fixedoffset-timezone-offset-from-local-date"></span>`fn offset_from_local_date(&self, _local: &NaiveDate) -> MappedLocalTime<FixedOffset>` — [`NaiveDate`](../../naive/date/index.md#naivedate), [`MappedLocalTime`](../index.md#mappedlocaltime), [`FixedOffset`](#fixedoffset)

- <span id="fixedoffset-timezone-offset-from-local-datetime"></span>`fn offset_from_local_datetime(&self, _local: &NaiveDateTime) -> MappedLocalTime<FixedOffset>` — [`NaiveDateTime`](../../naive/datetime/index.md#naivedatetime), [`MappedLocalTime`](../index.md#mappedlocaltime), [`FixedOffset`](#fixedoffset)

- <span id="fixedoffset-timezone-offset-from-utc-date"></span>`fn offset_from_utc_date(&self, _utc: &NaiveDate) -> FixedOffset` — [`NaiveDate`](../../naive/date/index.md#naivedate), [`FixedOffset`](#fixedoffset)

- <span id="fixedoffset-timezone-offset-from-utc-datetime"></span>`fn offset_from_utc_datetime(&self, _utc: &NaiveDateTime) -> FixedOffset` — [`NaiveDateTime`](../../naive/datetime/index.md#naivedatetime), [`FixedOffset`](#fixedoffset)

##### `impl ToString for FixedOffset`

- <span id="fixedoffset-tostring-to-string"></span>`fn to_string(&self) -> String`

