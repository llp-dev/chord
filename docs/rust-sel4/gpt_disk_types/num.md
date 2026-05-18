**gpt_disk_types > num**

# Module: num

## Contents

**Structs**

- [`U16Le`](#u16le) - 16-bit unsigned integer stored as a little-endian.
- [`U32Le`](#u32le) - 32-bit unsigned integer stored as a little-endian.
- [`U64Le`](#u64le) - 64-bit unsigned integer stored as a little-endian.

---

## gpt_disk_types::num::U16Le

*Struct*

16-bit unsigned integer stored as a little-endian.

**Tuple Struct**: `([u8; 2])`

**Methods:**

- `fn to_u16(self: Self) -> u16` - Convert to [`u16`] with the host's endianness.
- `fn from_u16(v: u16) -> Self` - Create a `U16Le` from a [`u16`] with the host's endianness.
- `fn set(self: & mut Self, v: u16)` - Update the value to a [`u16`] with the host's endianness.

**Traits:** Copy, Pod, Eq, Zeroable

**Trait Implementations:**

- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &U16Le) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> U16Le`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut Formatter) -> fmt::Result`
- **Ord**
  - `fn cmp(self: &Self, other: &U16Le) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &U16Le) -> bool`
- **Default**
  - `fn default() -> U16Le`
- **Display**
  - `fn fmt(self: &Self, f: & mut Formatter) -> fmt::Result`



## gpt_disk_types::num::U32Le

*Struct*

32-bit unsigned integer stored as a little-endian.

**Tuple Struct**: `([u8; 4])`

**Methods:**

- `fn to_u32(self: Self) -> u32` - Convert to [`u32`] with the host's endianness.
- `fn from_u32(v: u32) -> Self` - Create a `U32Le` from a [`u32`] with the host's endianness.
- `fn set(self: & mut Self, v: u32)` - Update the value to a [`u32`] with the host's endianness.

**Traits:** Copy, Pod, Eq, Zeroable

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut Formatter) -> fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &U32Le) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> U32Le`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut Formatter) -> fmt::Result`
- **Ord**
  - `fn cmp(self: &Self, other: &U32Le) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &U32Le) -> bool`
- **Default**
  - `fn default() -> U32Le`



## gpt_disk_types::num::U64Le

*Struct*

64-bit unsigned integer stored as a little-endian.

**Tuple Struct**: `([u8; 8])`

**Methods:**

- `fn to_u64(self: Self) -> u64` - Convert to [`u64`] with the host's endianness.
- `fn from_u64(v: u64) -> Self` - Create a `U64Le` from a [`u64`] with the host's endianness.
- `fn set(self: & mut Self, v: u64)` - Update the value to a [`u64`] with the host's endianness.

**Traits:** Zeroable, Copy, Pod, Eq

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut Formatter) -> fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &U64Le) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> U64Le`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut Formatter) -> fmt::Result`
- **Ord**
  - `fn cmp(self: &Self, other: &U64Le) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &U64Le) -> bool`
- **Default**
  - `fn default() -> U64Le`



