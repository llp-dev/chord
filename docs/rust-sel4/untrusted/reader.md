**untrusted > reader**

# Module: reader

## Contents

**Structs**

- [`EndOfInput`](#endofinput) - The error type used to indicate the end of the input was reached before the
- [`Reader`](#reader) - A read-only, forward-only cursor into the data in an `Input`.

---

## untrusted::reader::EndOfInput

*Struct*

The error type used to indicate the end of the input was reached before the
operation could be completed.

**Unit Struct**

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> EndOfInput`
- **PartialEq**
  - `fn eq(self: &Self, other: &EndOfInput) -> bool`



## untrusted::reader::Reader

*Struct*

A read-only, forward-only cursor into the data in an `Input`.

Using `Reader` to parse input helps to ensure that no byte of the input
will be accidentally processed more than once. Using `Reader` in
conjunction with `read_all` and `read_all_optional` helps ensure that no
byte of the input is accidentally left unprocessed. The methods of `Reader`
never panic, so `Reader` also assists the writing of panic-free code.

Intentionally avoids implementing `PartialEq` and `Eq` to avoid implicit
non-constant-time comparisons.

**Generic Parameters:**
- 'a

**Fields:**
- `input: no_panic::Slice<'a>`
- `i: usize`

**Methods:**

- `fn new(input: Input<'a>) -> Self` - Construct a new Reader for the given input. Use `read_all` or
- `fn at_end(self: &Self) -> bool` - Returns `true` if the reader is at the end of the input, and `false`
- `fn peek(self: &Self, b: u8) -> bool` - Returns `true` if there is at least one more byte in the input and that
- `fn read_byte(self: & mut Self) -> Result<u8, EndOfInput>` - Reads the next input byte.
- `fn read_bytes(self: & mut Self, num_bytes: usize) -> Result<Input<'a>, EndOfInput>` - Skips `num_bytes` of the input, returning the skipped input as an
- `fn read_bytes_to_end(self: & mut Self) -> Input<'a>` - Skips the reader to the end of the input, returning the skipped input
- `fn read_partial<F, R, E>(self: & mut Self, read: F) -> Result<(Input<'a>, R), E>` - Calls `read()` with the given input as a `Reader`. On success, returns a
- `fn skip(self: & mut Self, num_bytes: usize) -> Result<(), EndOfInput>` - Skips `num_bytes` of the input.
- `fn skip_to_end(self: & mut Self)` - Skips the reader to the end of the input.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> core::fmt::Result`



