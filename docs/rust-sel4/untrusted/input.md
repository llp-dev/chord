**untrusted > input**

# Module: input

## Contents

**Structs**

- [`Input`](#input) - A wrapper around `&'a [u8]` that helps in writing panic-free code.

---

## untrusted::input::Input

*Struct*

A wrapper around `&'a [u8]` that helps in writing panic-free code.

No methods of `Input` will ever panic.

Intentionally avoids implementing `PartialEq` and `Eq` to avoid implicit
non-constant-time comparisons.

**Generic Parameters:**
- 'a

**Fields:**
- `value: no_panic::Slice<'a>`

**Methods:**

- `fn from(bytes: &'a [u8]) -> Self` - Construct a new `Input` for the given input `bytes`.
- `fn is_empty(self: &Self) -> bool` - Returns `true` if the input is empty and false otherwise.
- `fn len(self: &Self) -> usize` - Returns the length of the `Input`.
- `fn read_all<F, R, E>(self: &Self, incomplete_read: E, read: F) -> Result<R, E>` - Calls `read` with the given input as a `Reader`, ensuring that `read`
- `fn as_slice_less_safe(self: &Self) -> &'a [u8]` - Access the input as a slice so it can be processed by functions that
- `fn into_value(self: Self) -> no_panic::Slice<'a>`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Input<'a>`
- **From**
  - `fn from(value: no_panic::Slice<'a>) -> Self`
- **From**
  - `fn from(value: &'a [u8]) -> Self`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`



