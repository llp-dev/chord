**sel4_pl031_driver**

# Module: sel4_pl031_driver

## Contents

**Structs**

- [`Driver`](#driver)

**Enums**

- [`Error`](#error)

---

## sel4_pl031_driver::Driver

*Struct*

**Methods:**

- `fn new_uninit(ptr: *mut ()) -> Self`
- `fn new(ptr: *mut ()) -> Self`
- `fn init(self: & mut Self)`

**Trait Implementations:**

- **DateTimeAccess**
  - `fn datetime(self: & mut Self) -> Result<NaiveDateTime, <Self as >::Error>`
  - `fn set_datetime(self: & mut Self, _datetime: &NaiveDateTime) -> Result<(), <Self as >::Error>`



## sel4_pl031_driver::Error

*Enum*

**Variants:**
- `UnsupportedOperation`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Error`



