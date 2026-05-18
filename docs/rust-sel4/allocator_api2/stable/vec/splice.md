**allocator_api2 > stable > vec > splice**

# Module: stable::vec::splice

## Contents

**Structs**

- [`Splice`](#splice) - A splicing iterator for `Vec`.

---

## allocator_api2::stable::vec::splice::Splice

*Struct*

A splicing iterator for `Vec`.

This struct is created by [`Vec::splice()`].
See its documentation for more.

# Example

```
let mut v = vec![0, 1, 2];
let new = [7, 8];
let iter: std::vec::Splice<_> = v.splice(1.., new);
```

**Generic Parameters:**
- 'a
- I
- A

**Traits:** ExactSizeIterator

**Trait Implementations:**

- **DoubleEndedIterator**
  - `fn next_back(self: & mut Self) -> Option<<Self as >::Item>`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Drop**
  - `fn drop(self: & mut Self)`



