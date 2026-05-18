**clap_builder > builder > range**

# Module: builder::range

## Contents

**Structs**

- [`ValueRange`](#valuerange) - Values per occurrence for an argument

---

## clap_builder::builder::range::ValueRange

*Struct*

Values per occurrence for an argument

**Methods:**

- `fn new<impl Into<Self>>(range: impl Trait) -> Self` - Create a range
- `fn min_values(self: &Self) -> usize` - Fewest number of values the argument accepts
- `fn max_values(self: &Self) -> usize` - Most number of values the argument accepts
- `fn takes_values(self: &Self) -> bool` - Report whether the argument takes any values (ie is a flag)

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ValueRange`
- **PartialEq**
  - `fn eq(self: &Self, other: &ValueRange) -> bool`
- **From**
  - `fn from(fixed: usize) -> Self`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **From**
  - `fn from(range: std::ops::Range<usize>) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut std::fmt::Formatter) -> std::fmt::Result`
- **From**
  - `fn from(_: std::ops::RangeFull) -> Self`
- **Default**
  - `fn default() -> Self`
- **From**
  - `fn from(range: std::ops::RangeFrom<usize>) -> Self`
- **Display**
  - `fn fmt(self: &Self, f: & mut std::fmt::Formatter) -> std::fmt::Result`
- **From**
  - `fn from(range: std::ops::RangeTo<usize>) -> Self`
- **RangeBounds**
  - `fn start_bound(self: &Self) -> std::ops::Bound<&usize>`
  - `fn end_bound(self: &Self) -> std::ops::Bound<&usize>`
- **From**
  - `fn from(range: std::ops::RangeInclusive<usize>) -> Self`
- **From**
  - `fn from(range: std::ops::RangeToInclusive<usize>) -> Self`



