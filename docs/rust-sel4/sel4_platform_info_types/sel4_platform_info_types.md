**sel4_platform_info_types**

# Module: sel4_platform_info_types

## Contents

**Structs**

- [`PlatformInfo`](#platforminfo)

---

## sel4_platform_info_types::PlatformInfo

*Struct*

**Generic Parameters:**
- 'a
- T

**Fields:**
- `memory: &'a [core::ops::Range<T>]`
- `devices: &'a [core::ops::Range<T>]`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> PlatformInfo<'a, T>`



