**rkyv > time**

# Module: time

## Contents

**Structs**

- [`ArchivedDuration`](#archivedduration) - An archived [`Duration`](core::time::Duration).

---

## rkyv::time::ArchivedDuration

*Struct*

An archived [`Duration`](core::time::Duration).

**Methods:**

- `fn as_secs(self: &Self) -> u64` - Returns the number of _whole_ seconds contained by this
- `fn subsec_millis(self: &Self) -> u32` - Returns the fractional part of this `ArchivedDuration`, in whole
- `fn subsec_micros(self: &Self) -> u32` - Returns the fractional part of this `ArchivedDuration`, in whole
- `fn subsec_nanos(self: &Self) -> u32` - Returns the fractional part of this `Duration`, in nanoseconds.
- `fn as_millis(self: &Self) -> u128` - Returns the total number of whole milliseconds contained by this
- `fn as_micros(self: &Self) -> u128` - Returns the total number of whole microseconds contained by this
- `fn as_nanos(self: &Self) -> u128` - Returns the total number of nanoseconds contained by this
- `fn as_secs_f64(self: &Self) -> f64` - Returns the number of seconds contained by this `ArchivedDuration` as
- `fn as_secs_f32(self: &Self) -> f32` - Returns the number of seconds contained by this `ArchivedDuration` as
- `fn emplace(secs: u64, nanos: u32, out: *mut ArchivedDuration)` - Constructs an archived duration at the given position.

**Traits:** Copy, Portable, Eq

**Trait Implementations:**

- **Deserialize**
  - `fn deserialize(self: &Self, _: & mut D) -> Result<Duration, <D as >::Error>`
- **CheckBytes**
  - `fn check_bytes(value: *const Self, context: & mut __C) -> ::core::result::Result<(), <__C as ::bytecheck::rancor::Fallible>::Error>`
- **Ord**
  - `fn cmp(self: &Self, other: &ArchivedDuration) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &ArchivedDuration) -> bool`
- **Default**
  - `fn default() -> ArchivedDuration`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Duration) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> ArchivedDuration`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &ArchivedDuration) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Verify**
  - `fn verify(self: &Self, _: & mut C) -> Result<(), <C as >::Error>`



