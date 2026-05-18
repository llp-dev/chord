**allocator_api2 > stable > vec > into_iter**

# Module: stable::vec::into_iter

## Contents

**Structs**

- [`IntoIter`](#intoiter) - An iterator that moves out of a vector.

---

## allocator_api2::stable::vec::into_iter::IntoIter

*Struct*

An iterator that moves out of a vector.

This `struct` is created by the `into_iter` method on [`Vec`](super::Vec)
(provided by the [`IntoIterator`] trait).

# Example

```
let v = vec![0, 1, 2];
let iter: std::vec::IntoIter<_> = v.into_iter();
```

**Generic Parameters:**
- T
- A

**Methods:**

- `fn as_slice(self: &Self) -> &[T]` - Returns the remaining items of this iterator as a slice.
- `fn as_mut_slice(self: & mut Self) -> & mut [T]` - Returns the remaining items of this iterator as a mutable slice.
- `fn allocator(self: &Self) -> &A` - Returns a reference to the underlying allocator.

**Traits:** FusedIterator, Send, ExactSizeIterator, Sync

**Trait Implementations:**

- **Drop**
  - `fn drop(self: & mut Self)`
- **Clone**
  - `fn clone(self: &Self) -> Self`
- **DoubleEndedIterator**
  - `fn next_back(self: & mut Self) -> Option<T>`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<T>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
  - `fn count(self: Self) -> usize`
- **AsRef**
  - `fn as_ref(self: &Self) -> &[T]`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



