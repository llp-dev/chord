**sel4 > error**

# Module: error

## Contents

**Enums**

- [`Error`](#error) - Corresponds to `seL4_Error`.

**Type Aliases**

- [`Result`](#result) - Alias for `Result<_, Error>`.

---

## sel4::error::Error

*Enum*

Corresponds to `seL4_Error`.

**Variants:**
- `InvalidArgument`
- `InvalidCapability`
- `IllegalOperation`
- `RangeError`
- `AlignmentError`
- `FailedLookup`
- `TruncatedMessage`
- `DeleteFirst`
- `RevokeFirst`
- `NotEnoughMemory`

**Methods:**

- `fn into_sys(self: Self) -> sys::seL4_Error::Type`
- `fn from_sys(err: sys::seL4_Error::Type) -> Option<Self>`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Error) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Error`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## sel4::error::Result

*Type Alias*: `result::Result<T, Error>`

Alias for `Result<_, Error>`.



