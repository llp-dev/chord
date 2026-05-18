**embedded_fat > fat > lfn**

# Module: fat::lfn

## Contents

**Structs**

- [`LfnEntry`](#lfnentry)

---

## embedded_fat::fat::lfn::LfnEntry

*Struct*

**Fields:**
- `is_start: bool`
- `sequence: u8`
- `checksum: u8`
- `buffer: [char; 13]`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &LfnEntry) -> $crate::option::Option<$crate::cmp::Ordering>`
- **PartialEq**
  - `fn eq(self: &Self, other: &LfnEntry) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> LfnEntry`
- **Ord**
  - `fn cmp(self: &Self, other: &LfnEntry) -> $crate::cmp::Ordering`



