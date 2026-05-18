**httparse > iter**

# Module: iter

## Contents

**Structs**

- [`Bytes`](#bytes)

---

## httparse::iter::Bytes

*Struct*

**Generic Parameters:**
- 'a

**Methods:**

- `fn new(slice: &'a [u8]) -> Bytes<'a>`
- `fn pos(self: &Self) -> usize`
- `fn peek(self: &Self) -> Option<u8>`
- `fn peek_ahead(self: &Self, n: usize) -> Option<u8>` - Peek at byte `n` ahead of cursor
- `fn peek_n<'b, U>(self: &'b Self, n: usize) -> Option<U>`
- `fn bump(self: & mut Self)` - Advance by 1, equivalent to calling `advance(1)`.
- `fn advance(self: & mut Self, n: usize)` - Advance cursor by `n`
- `fn len(self: &Self) -> usize`
- `fn is_empty(self: &Self) -> bool`
- `fn slice(self: & mut Self) -> &'a [u8]`
- `fn slice_skip(self: & mut Self, skip: usize) -> &'a [u8]` - Deprecated. Do not use!
- `fn commit(self: & mut Self)`
- `fn advance_and_commit(self: & mut Self, n: usize)` - # Safety
- `fn as_ptr(self: &Self) -> *const u8`
- `fn start(self: &Self) -> *const u8`
- `fn end(self: &Self) -> *const u8`
- `fn set_cursor(self: & mut Self, ptr: *const u8)` - # Safety

**Trait Implementations:**

- **AsRef**
  - `fn as_ref(self: &Self) -> &[u8]`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<u8>`



