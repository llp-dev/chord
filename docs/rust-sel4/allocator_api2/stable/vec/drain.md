**allocator_api2 > stable > vec > drain**

# Module: stable::vec::drain

## Contents

**Structs**

- [`Drain`](#drain) - A draining iterator for `Vec<T>`.

---

## allocator_api2::stable::vec::drain::Drain

*Struct*

A draining iterator for `Vec<T>`.

This `struct` is created by [`Vec::drain`].
See its documentation for more.

# Example

```
let mut v = vec![0, 1, 2];
let iter: std::vec::Drain<_> = v.drain(..);
```

**Generic Parameters:**
- 'a
- T
- A

**Methods:**

- `fn as_slice(self: &Self) -> &[T]` - Returns the remaining items of this iterator as a slice.
- `fn allocator(self: &Self) -> &A` - Returns a reference to the underlying allocator.
- `fn keep_rest(self: Self)` - Keep unyielded elements in the source `Vec`.

**Traits:** Send, ExactSizeIterator, Sync, FusedIterator

**Trait Implementations:**

- **Drop**
  - `fn drop(self: & mut Self)`
- **DoubleEndedIterator**
  - `fn next_back(self: & mut Self) -> Option<T>`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<T>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **AsRef**
  - `fn as_ref(self: &Self) -> &[T]`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



