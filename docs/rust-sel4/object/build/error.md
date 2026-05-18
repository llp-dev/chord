**object > build > error**

# Module: build::error

## Contents

**Structs**

- [`Error`](#error) - The error type used within the build module.

**Type Aliases**

- [`Result`](#result) - The result type used within the build module.

---

## object::build::error::Error

*Struct*

The error type used within the build module.

**Tuple Struct**: `()`

**Traits:** Eq, Error

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &Error) -> bool`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **From**
  - `fn from(error: write::Error) -> Error`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **From**
  - `fn from(error: read::Error) -> Error`
- **Clone**
  - `fn clone(self: &Self) -> Error`



## object::build::error::Result

*Type Alias*: `result::Result<T, Error>`

The result type used within the build module.



