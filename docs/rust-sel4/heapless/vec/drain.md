**heapless > vec > drain**

# Module: vec::drain

## Contents

**Structs**

- [`Drain`](#drain) - A draining iterator for [`Vec`](super::Vec).

---

## heapless::vec::drain::Drain

*Struct*

A draining iterator for [`Vec`](super::Vec).

This `struct` is created by [`Vec::drain`](super::Vec::drain).
See its documentation for more.

**Generic Parameters:**
- 'a
- T
- LenT

**Methods:**

- `fn as_slice(self: &Self) -> &[T]` - Returns the remaining items of this iterator as a slice.

**Traits:** Send, Sync, FusedIterator, ExactSizeIterator

**Trait Implementations:**

- **Drop**
  - `fn drop(self: & mut Self)`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<T>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **DoubleEndedIterator**
  - `fn next_back(self: & mut Self) -> Option<T>`
- **AsRef**
  - `fn as_ref(self: &Self) -> &[T]`



