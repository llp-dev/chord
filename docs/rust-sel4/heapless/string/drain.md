**heapless > string > drain**

# Module: string::drain

## Contents

**Structs**

- [`Drain`](#drain) - A draining iterator for `String`.

---

## heapless::string::drain::Drain

*Struct*

A draining iterator for `String`.

This struct is created by the [`drain`] method on [`crate::String`]. See its
documentation for more.

[`drain`]: crate::String::drain

**Generic Parameters:**
- 'a
- LenT

**Methods:**

- `fn as_str(self: &Self) -> &str` - Returns the remaining (sub)string of this iterator as a slice.

**Traits:** Send, Sync, FusedIterator

**Trait Implementations:**

- **AsRef**
  - `fn as_ref(self: &Self) -> &str`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<char>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
  - `fn last(self: Self) -> Option<char>`
- **AsRef**
  - `fn as_ref(self: &Self) -> &[u8]`
- **Drop**
  - `fn drop(self: & mut Self)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **DoubleEndedIterator**
  - `fn next_back(self: & mut Self) -> Option<char>`



