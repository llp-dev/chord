**sel4_one_ref_cell**

# Module: sel4_one_ref_cell

## Contents

**Structs**

- [`OneRefCell`](#onerefcell)

**Enums**

- [`Error`](#error)

---

## sel4_one_ref_cell::Error

*Enum*

**Variants:**
- `AlreadyTaken`

**Traits:** Copy, Eq

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Error) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Ord**
  - `fn cmp(self: &Self, other: &Error) -> $crate::cmp::Ordering`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Error) -> $crate::option::Option<$crate::cmp::Ordering>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Error`



## sel4_one_ref_cell::OneRefCell

*Struct*

**Generic Parameters:**
- T

**Methods:**

- `fn new(value: T) -> Self`
- `fn take(self: &Self) -> Result<& mut T, Error>`

**Traits:** Sync

**Trait Implementations:**

- **From**
  - `fn from(t: T) -> Self`
- **Default**
  - `fn default() -> Self`



