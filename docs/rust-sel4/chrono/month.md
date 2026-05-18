**chrono > month**

# Module: month

## Contents

**Structs**

- [`Months`](#months) - A duration in calendar months
- [`ParseMonthError`](#parsemontherror) - An error resulting from reading `<Month>` value with `FromStr`.

**Enums**

- [`Month`](#month) - The month of the year.

---

## chrono::month::Month

*Enum*

The month of the year.

This enum is just a convenience implementation.
The month in dates created by DateLike objects does not return this enum.

It is possible to convert from a date to a month independently
```
use chrono::prelude::*;
let date = Utc.with_ymd_and_hms(2019, 10, 28, 9, 10, 11).unwrap();
// `2019-10-28T09:10:11Z`
let month = Month::try_from(u8::try_from(date.month()).unwrap()).ok();
assert_eq!(month, Some(Month::October))
```
Or from a Month to an integer usable by dates
```
# use chrono::prelude::*;
let month = Month::January;
let dt = Utc.with_ymd_and_hms(2019, month.number_from_month(), 28, 9, 10, 11).unwrap();
assert_eq!((dt.year(), dt.month(), dt.day()), (2019, 1, 28));
```
Allows mapping from and to month, from 1-January to 12-December.
Can be Serialized/Deserialized with serde

**Variants:**
- `January` - January
- `February` - February
- `March` - March
- `April` - April
- `May` - May
- `June` - June
- `July` - July
- `August` - August
- `September` - September
- `October` - October
- `November` - November
- `December` - December

**Methods:**

- `fn succ(self: &Self) -> Month` - The next month.
- `fn pred(self: &Self) -> Month` - The previous month.
- `fn number_from_month(self: &Self) -> u32` - Returns a month-of-year number starting from January = 1.
- `fn name(self: &Self) -> &'static str` - Get the name of the month
- `fn num_days(self: &Self, year: i32) -> Option<u8>` - Get the length in days of the month

**Traits:** Eq, Copy

**Trait Implementations:**

- **Ord**
  - `fn cmp(self: &Self, other: &Month) -> $crate::cmp::Ordering`
- **Clone**
  - `fn clone(self: &Self) -> Month`
- **Deserialize**
  - `fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>`
- **TryFrom**
  - `fn try_from(value: u8) -> Result<Self, <Self as >::Error>`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Month) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Serialize**
  - `fn serialize<S>(self: &Self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>`
- **FromStr**
  - `fn from_str(s: &str) -> Result<Self, <Self as >::Err>`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Month) -> bool`
- **FromPrimitive**
  - `fn from_u64(n: u64) -> Option<Month>` - Returns an `Option<Month>` from a i64, assuming a 1-index, January = 1.
  - `fn from_i64(n: i64) -> Option<Month>`
  - `fn from_u32(n: u32) -> Option<Month>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## chrono::month::Months

*Struct*

A duration in calendar months

**Tuple Struct**: `()`

**Methods:**

- `fn new(num: u32) -> Self` - Construct a new `Months` from a number of months
- `fn as_u32(self: &Self) -> u32` - Returns the total number of months in the `Months` instance.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Ord**
  - `fn cmp(self: &Self, other: &Months) -> $crate::cmp::Ordering`
- **Clone**
  - `fn clone(self: &Self) -> Months`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Months) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Months) -> bool`



## chrono::month::ParseMonthError

*Struct*

An error resulting from reading `<Month>` value with `FromStr`.

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ParseMonthError`
- **PartialEq**
  - `fn eq(self: &Self, other: &ParseMonthError) -> bool`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



