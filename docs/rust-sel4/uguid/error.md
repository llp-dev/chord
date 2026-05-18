**uguid > error**

# Module: error

## Contents

**Enums**

- [`GuidFromStrError`](#guidfromstrerror) - Error type for [`Guid::try_parse`] and [`Guid::from_str`].

---

## uguid::error::GuidFromStrError

*Enum*

Error type for [`Guid::try_parse`] and [`Guid::from_str`].

[`Guid::from_str`]: core::str::FromStr::from_str
[`Guid::try_parse`]: crate::Guid::try_parse

**Variants:**
- `Length` - Input has the wrong length, expected 36 bytes.
- `Separator(u8)` - Input is missing a separator (`-`) at this byte index.
- `Hex(u8)` - Input contains invalid ASCII hex at this byte index.

**Traits:** Eq, Error, Copy

**Trait Implementations:**

- **Ord**
  - `fn cmp(self: &Self, other: &GuidFromStrError) -> $crate::cmp::Ordering`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut Formatter) -> fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Default**
  - `fn default() -> Self`
- **PartialEq**
  - `fn eq(self: &Self, other: &GuidFromStrError) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> GuidFromStrError`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &GuidFromStrError) -> $crate::option::Option<$crate::cmp::Ordering>`



