**sel4 > invocations**

# Module: invocations

## Contents

**Structs**

- [`TcbFlagsBuilder`](#tcbflagsbuilder)

---

## sel4::invocations::TcbFlagsBuilder

*Struct*

**Tuple Struct**: `()`

**Methods:**

- `fn new() -> Self`
- `fn build(self: Self) -> Word`
- `fn fpu_disabled(self: Self, val: bool) -> Self`

**Trait Implementations:**

- **PartialEq**
  - `fn eq(self: &Self, other: &TcbFlagsBuilder) -> bool`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> Self`
- **Clone**
  - `fn clone(self: &Self) -> TcbFlagsBuilder`



