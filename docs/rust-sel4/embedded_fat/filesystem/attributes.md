**embedded_fat > filesystem > attributes**

# Module: filesystem::attributes

## Contents

**Structs**

- [`Attributes`](#attributes) - Indicates whether a directory entry is read-only, a directory, a volume

---

## embedded_fat::filesystem::attributes::Attributes

*Struct*

Indicates whether a directory entry is read-only, a directory, a volume
label, etc.

**Tuple Struct**: `()`

**Methods:**

- `fn is_read_only(self: Self) -> bool` - Does this file has the read-only attribute set?
- `fn is_hidden(self: Self) -> bool` - Does this file has the hidden attribute set?
- `fn is_system(self: Self) -> bool` - Does this file has the system attribute set?
- `fn is_volume(self: Self) -> bool` - Does this file has the volume attribute set?
- `fn is_directory(self: Self) -> bool` - Does this entry point at a directory?
- `fn is_archive(self: Self) -> bool` - Does this need archiving?
- `fn is_lfn(self: Self) -> bool` - Is this a long file name fragment?

**Traits:** Eq, Copy

**Trait Implementations:**

- **Ord**
  - `fn cmp(self: &Self, other: &Attributes) -> $crate::cmp::Ordering`
- **Clone**
  - `fn clone(self: &Self) -> Attributes`
- **PartialEq**
  - `fn eq(self: &Self, other: &Attributes) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Attributes) -> $crate::option::Option<$crate::cmp::Ordering>`



