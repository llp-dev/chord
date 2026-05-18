*[chrono](../index.md) / [month](index.md)*

---

# Module `month`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`month_serde`](#month-serde) | mod |  |
| [`Months`](#months) | struct | A duration in calendar months |
| [`ParseMonthError`](#parsemontherror) | struct | An error resulting from reading `<Month>` value with `FromStr`. |
| [`Month`](#month) | enum | The month of the year. |

## Modules

- [`month_serde`](month_serde/index.md)

## Structs

### `Months`

```rust
struct Months(u32);
```

A duration in calendar months

#### Implementations

- <span id="months-new"></span>`const fn new(num: u32) -> Self`

  Construct a new `Months` from a number of months

- <span id="months-as-u32"></span>`const fn as_u32(&self) -> u32`

  Returns the total number of months in the `Months` instance.

#### Trait Implementations

##### `impl<Tz: TimeZone> Add for DateTime<Tz>`

- <span id="datetime-add-type-output"></span>`type Output = DateTime<Tz>`

- <span id="datetime-add"></span>`fn add(self, rhs: Months) -> <Self as >::Output` — [`Months`](#months)

##### `impl Clone for Months`

- <span id="months-clone"></span>`fn clone(&self) -> Months` — [`Months`](#months)

##### `impl Copy for Months`

##### `impl Debug for Months`

- <span id="months-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Months`

##### `impl Hash for Months`

- <span id="months-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for Months`

- <span id="months-ord-cmp"></span>`fn cmp(&self, other: &Months) -> cmp::Ordering` — [`Months`](#months)

##### `impl PartialEq for Months`

- <span id="months-partialeq-eq"></span>`fn eq(&self, other: &Months) -> bool` — [`Months`](#months)

##### `impl PartialOrd for Months`

- <span id="months-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Months) -> option::Option<cmp::Ordering>` — [`Months`](#months)

##### `impl StructuralPartialEq for Months`

##### `impl<Tz: TimeZone> Sub for DateTime<Tz>`

- <span id="datetime-sub-type-output"></span>`type Output = DateTime<Tz>`

- <span id="datetime-sub"></span>`fn sub(self, rhs: Months) -> <Self as >::Output` — [`Months`](#months)

### `ParseMonthError`

```rust
struct ParseMonthError {
    _dummy: (),
}
```

An error resulting from reading `<Month>` value with `FromStr`.

#### Trait Implementations

##### `impl Clone for ParseMonthError`

- <span id="parsemontherror-clone"></span>`fn clone(&self) -> ParseMonthError` — [`ParseMonthError`](#parsemontherror)

##### `impl Debug for ParseMonthError`

- <span id="parsemontherror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for ParseMonthError`

- <span id="parsemontherror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ParseMonthError`

##### `impl PartialEq for ParseMonthError`

- <span id="parsemontherror-partialeq-eq"></span>`fn eq(&self, other: &ParseMonthError) -> bool` — [`ParseMonthError`](#parsemontherror)

##### `impl StructuralPartialEq for ParseMonthError`

##### `impl ToString for ParseMonthError`

- <span id="parsemontherror-tostring-to-string"></span>`fn to_string(&self) -> String`

## Enums

### `Month`

```rust
enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}
```

The month of the year.

This enum is just a convenience implementation.
The month in dates created by DateLike objects does not return this enum.

It is possible to convert from a date to a month independently
```rust
use chrono::prelude::*;
let date = Utc.with_ymd_and_hms(2019, 10, 28, 9, 10, 11).unwrap();
// `2019-10-28T09:10:11Z`
let month = Month::try_from(u8::try_from(date.month()).unwrap()).ok();
assert_eq!(month, Some(Month::October))
```
Or from a Month to an integer usable by dates
```rust
use chrono::prelude::*;
let month = Month::January;
let dt = Utc.with_ymd_and_hms(2019, month.number_from_month(), 28, 9, 10, 11).unwrap();
assert_eq!((dt.year(), dt.month(), dt.day()), (2019, 1, 28));
```
Allows mapping from and to month, from 1-January to 12-December.
Can be Serialized/Deserialized with serde

#### Variants

- **`January`**

  January

- **`February`**

  February

- **`March`**

  March

- **`April`**

  April

- **`May`**

  May

- **`June`**

  June

- **`July`**

  July

- **`August`**

  August

- **`September`**

  September

- **`October`**

  October

- **`November`**

  November

- **`December`**

  December

#### Implementations

- <span id="month-succ"></span>`const fn succ(&self) -> Month` — [`Month`](#month)

  The next month.

  

  `m`:        | `January`  | `February` | `...` | `December`

  ----------- | ---------  | ---------- | --- | ---------

  `m.succ()`: | `February` | `March`    | `...` | `January`

- <span id="month-pred"></span>`const fn pred(&self) -> Month` — [`Month`](#month)

  The previous month.

  

  `m`:        | `January`  | `February` | `...` | `December`

  ----------- | ---------  | ---------- | --- | ---------

  `m.pred()`: | `December` | `January`  | `...` | `November`

- <span id="month-number-from-month"></span>`const fn number_from_month(&self) -> u32`

  Returns a month-of-year number starting from January = 1.

  

  `m`:                     | `January` | `February` | `...` | `December`

  -------------------------| --------- | ---------- | --- | -----

  `m.number_from_month()`: | 1         | 2          | `...` | 12

- <span id="month-name"></span>`const fn name(&self) -> &'static str`

  Get the name of the month

  

  ```rust

  use chrono::Month;

  

  assert_eq!(Month::January.name(), "January")

  ```

- <span id="month-num-days"></span>`fn num_days(&self, year: i32) -> Option<u8>`

  Get the length in days of the month

  

  Yields `None` if `year` is out of range for `NaiveDate`.

#### Trait Implementations

##### `impl Clone for Month`

- <span id="month-clone"></span>`fn clone(&self) -> Month` — [`Month`](#month)

##### `impl Copy for Month`

##### `impl Debug for Month`

- <span id="month-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Deserialize for super::Month`

- <span id="supermonth-deserialize"></span>`fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>`

##### `impl DeserializeOwned for Month`

##### `impl Eq for Month`

##### `impl FromPrimitive for Month`

- <span id="month-fromprimitive-from-u64"></span>`fn from_u64(n: u64) -> Option<Month>` — [`Month`](#month)

  Returns an `Option<Month>` from a i64, assuming a 1-index, January = 1.

  

  `Month::from_i64(n: i64)`: | `1`                  | `2`                   | ... | `12`

  ---------------------------| -------------------- | --------------------- | ... | -----

  ``:                        | Some(Month::January) | Some(Month::February) | ... | Some(Month::December)

- <span id="month-fromprimitive-from-i64"></span>`fn from_i64(n: i64) -> Option<Month>` — [`Month`](#month)

- <span id="month-fromprimitive-from-u32"></span>`fn from_u32(n: u32) -> Option<Month>` — [`Month`](#month)

##### `impl FromStr for crate::Month`

- <span id="cratemonth-fromstr-type-err"></span>`type Err = ParseMonthError`

- <span id="cratemonth-fromstr-from-str"></span>`fn from_str(s: &str) -> Result<Self, <Self as >::Err>`

##### `impl Hash for Month`

- <span id="month-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for Month`

- <span id="month-ord-cmp"></span>`fn cmp(&self, other: &Month) -> cmp::Ordering` — [`Month`](#month)

##### `impl PartialEq for Month`

- <span id="month-partialeq-eq"></span>`fn eq(&self, other: &Month) -> bool` — [`Month`](#month)

##### `impl PartialOrd for Month`

- <span id="month-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Month) -> option::Option<cmp::Ordering>` — [`Month`](#month)

##### `impl Serialize for super::Month`

- <span id="supermonth-serialize"></span>`fn serialize<S>(&self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>`

##### `impl StructuralPartialEq for Month`

