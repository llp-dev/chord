**rancor > boxed_error**

# Module: boxed_error

## Contents

**Structs**

- [`BoxedError`](#boxederror) - An error type that preserves all detailed error messages. It is optimized

---

## rancor::boxed_error::BoxedError

*Struct*

An error type that preserves all detailed error messages. It is optimized
to fit in a single pointer.

**Trait Implementations:**

- **Error**
  - `fn source(self: &Self) -> Option<&dyn error::Error>`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Source**
  - `fn new<T>(source: T) -> Self`
- **Trace**
  - `fn trace<R>(self: Self, trace: R) -> Self`



