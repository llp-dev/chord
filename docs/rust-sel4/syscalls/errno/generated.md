**syscalls > errno > generated**

# Module: errno::generated

## Contents

**Structs**

- [`Errno`](#errno)

---

## syscalls::errno::generated::Errno

*Struct*

**Tuple Struct**: `(i32)`

**Methods:**

- `fn new(num: i32) -> Self` - Creates a new `Errno`.
- `fn into_raw(self: Self) -> i32` - Converts the `Errno` into a raw `i32`.
- `fn is_valid(self: &Self) -> bool` - Returns true if the error code is valid (i.e., less than 4096).
- `fn from_ret(value: usize) -> Result<usize, Errno>` - Converts a raw syscall return value to a result.
- `fn name(self: &Self) -> Option<&'static str>` - Returns the name of the error. If the internal error code is unknown or
- `fn description(self: &Self) -> Option<&'static str>` - Returns the error description. If the internal error code is unknown or
- `fn name_and_description(self: &Self) -> Option<(&'static str, &'static str)>` - Returns a pair containing the name of the error and a string

**Traits:** Eq, Copy

**Trait Implementations:**

- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &Errno) -> $crate::option::Option<$crate::cmp::Ordering>`
- **PartialEq**
  - `fn eq(self: &Self, other: &Errno) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Errno`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Ord**
  - `fn cmp(self: &Self, other: &Errno) -> $crate::cmp::Ordering`



