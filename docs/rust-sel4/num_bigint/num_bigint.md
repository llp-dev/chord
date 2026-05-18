**num_bigint**

# Module: num_bigint

## Contents

**Structs**

- [`ParseBigIntError`](#parsebiginterror)
- [`TryFromBigIntError`](#tryfrombiginterror) - The error type returned when a checked conversion regarding big integer fails.

---

## num_bigint::ParseBigIntError

*Struct*

**Traits:** Eq

**Trait Implementations:**

- **Error**
  - `fn description(self: &Self) -> &str`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ParseBigIntError`
- **PartialEq**
  - `fn eq(self: &Self, other: &ParseBigIntError) -> bool`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## num_bigint::TryFromBigIntError

*Struct*

The error type returned when a checked conversion regarding big integer fails.

**Generic Parameters:**
- T

**Methods:**

- `fn into_original(self: Self) -> T` - Extract the original value, if available. The value will be available

**Traits:** Copy, Eq

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> TryFromBigIntError<T>`
- **PartialEq**
  - `fn eq(self: &Self, other: &TryFromBigIntError<T>) -> bool`
- **Error**
  - `fn description(self: &Self) -> &str`



