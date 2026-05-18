**chrono > weekday**

# Module: weekday

## Contents

**Structs**

- [`ParseWeekdayError`](#parseweekdayerror) - An error resulting from reading `Weekday` value with `FromStr`.

**Enums**

- [`Weekday`](#weekday) - The day of week.

---

## chrono::weekday::ParseWeekdayError

*Struct*

An error resulting from reading `Weekday` value with `FromStr`.

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ParseWeekdayError`
- **PartialEq**
  - `fn eq(self: &Self, other: &ParseWeekdayError) -> bool`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## chrono::weekday::Weekday

*Enum*

The day of week.

The order of the days of week depends on the context.
(This is why this type does *not* implement `PartialOrd` or `Ord` traits.)
One should prefer `*_from_monday` or `*_from_sunday` methods to get the correct result.

# Example
```
use chrono::Weekday;

let monday = "Monday".parse::<Weekday>().unwrap();
assert_eq!(monday, Weekday::Mon);

let sunday = Weekday::try_from(6).unwrap();
assert_eq!(sunday, Weekday::Sun);

assert_eq!(sunday.num_days_from_monday(), 6); // starts counting with Monday = 0
assert_eq!(sunday.number_from_monday(), 7); // starts counting with Monday = 1
assert_eq!(sunday.num_days_from_sunday(), 0); // starts counting with Sunday = 0
assert_eq!(sunday.number_from_sunday(), 1); // starts counting with Sunday = 1

assert_eq!(sunday.succ(), monday);
assert_eq!(sunday.pred(), Weekday::Sat);
```

**Variants:**
- `Mon` - Monday.
- `Tue` - Tuesday.
- `Wed` - Wednesday.
- `Thu` - Thursday.
- `Fri` - Friday.
- `Sat` - Saturday.
- `Sun` - Sunday.

**Methods:**

- `fn succ(self: &Self) -> Weekday` - The next day in the week.
- `fn pred(self: &Self) -> Weekday` - The previous day in the week.
- `fn number_from_monday(self: &Self) -> u32` - Returns a day-of-week number starting from Monday = 1. (ISO 8601 weekday number)
- `fn number_from_sunday(self: &Self) -> u32` - Returns a day-of-week number starting from Sunday = 1.
- `fn num_days_from_monday(self: &Self) -> u32` - Returns a day-of-week number starting from Monday = 0.
- `fn num_days_from_sunday(self: &Self) -> u32` - Returns a day-of-week number starting from Sunday = 0.
- `fn days_since(self: &Self, other: Weekday) -> u32` - The number of days since the given day.

**Traits:** Copy, Eq

**Trait Implementations:**

- **FromStr**
  - `fn from_str(s: &str) -> Result<Self, <Self as >::Err>`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Weekday) -> bool`
- **Deserialize**
  - `fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>`
- **TryFrom**
  - `fn try_from(value: u8) -> Result<Self, <Self as >::Error>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **FromPrimitive**
  - `fn from_i64(n: i64) -> Option<Weekday>`
  - `fn from_u64(n: u64) -> Option<Weekday>`
- **Serialize**
  - `fn serialize<S>(self: &Self, serializer: S) -> Result<<S as >::Ok, <S as >::Error>`
- **Clone**
  - `fn clone(self: &Self) -> Weekday`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



