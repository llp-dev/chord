**anstream > adapter > wincon**

# Module: adapter::wincon

## Contents

**Structs**

- [`WinconBytes`](#winconbytes) - Incrementally convert to wincon calls for non-contiguous data
- [`WinconBytesIter`](#winconbytesiter) - See [`WinconBytes`]

---

## anstream::adapter::wincon::WinconBytes

*Struct*

Incrementally convert to wincon calls for non-contiguous data

**Methods:**

- `fn new() -> Self` - Initial state
- `fn extract_next<'s>(self: &'s  mut Self, bytes: &'s [u8]) -> WinconBytesIter<'s>` - Strip the next segment of data

**Traits:** Eq

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> WinconBytes`
- **Default**
  - `fn default() -> WinconBytes`
- **PartialEq**
  - `fn eq(self: &Self, other: &WinconBytes) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## anstream::adapter::wincon::WinconBytesIter

*Struct*

See [`WinconBytes`]

**Generic Parameters:**
- 's

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
- **PartialEq**
  - `fn eq(self: &Self, other: &WinconBytesIter<'s>) -> bool`



