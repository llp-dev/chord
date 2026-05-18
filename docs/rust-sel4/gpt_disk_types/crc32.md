**gpt_disk_types > crc32**

# Module: crc32

## Contents

**Structs**

- [`Crc32`](#crc32) - 32-bit CRC (cyclic redundancy check).

---

## gpt_disk_types::crc32::Crc32

*Struct*

32-bit CRC (cyclic redundancy check).

**Tuple Struct**: `(crate::U32Le)`

**Methods:**


**Traits:** Eq, Zeroable, Copy, Pod

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Crc32`
- **LowerHex**
  - `fn fmt(self: &Self, f: & mut Formatter) -> fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Crc32) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut Formatter) -> fmt::Result`
- **Ord**
  - `fn cmp(self: &Self, other: &Crc32) -> $crate::cmp::Ordering`
- **PartialEq**
  - `fn eq(self: &Self, other: &Crc32) -> bool`
- **Default**
  - `fn default() -> Crc32`



