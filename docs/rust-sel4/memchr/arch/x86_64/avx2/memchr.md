**memchr > arch > x86_64 > avx2 > memchr**

# Module: arch::x86_64::avx2::memchr

## Contents

**Structs**

- [`One`](#one) - Finds all occurrences of a single byte in a haystack.
- [`OneIter`](#oneiter) - An iterator over all occurrences of a single byte in a haystack.
- [`Three`](#three) - Finds all occurrences of three bytes in a haystack.
- [`ThreeIter`](#threeiter) - An iterator over all occurrences of three possible bytes in a haystack.
- [`Two`](#two) - Finds all occurrences of two bytes in a haystack.
- [`TwoIter`](#twoiter) - An iterator over all occurrences of two possible bytes in a haystack.

---

## memchr::arch::x86_64::avx2::memchr::One

*Struct*

Finds all occurrences of a single byte in a haystack.

**Methods:**

- `fn new(needle: u8) -> Option<One>` - Create a new searcher that finds occurrences of the needle byte given.
- `fn new_unchecked(needle: u8) -> One` - Create a new finder specific to AVX2 vectors and routines without
- `fn is_available() -> bool` - Returns true when this implementation is available in the current
- `fn find(self: &Self, haystack: &[u8]) -> Option<usize>` - Return the first occurrence of one of the needle bytes in the given
- `fn rfind(self: &Self, haystack: &[u8]) -> Option<usize>` - Return the last occurrence of one of the needle bytes in the given
- `fn count(self: &Self, haystack: &[u8]) -> usize` - Counts all occurrences of this byte in the given haystack.
- `fn find_raw(self: &Self, start: *const u8, end: *const u8) -> Option<*const u8>` - Like `find`, but accepts and returns raw pointers.
- `fn rfind_raw(self: &Self, start: *const u8, end: *const u8) -> Option<*const u8>` - Like `rfind`, but accepts and returns raw pointers.
- `fn count_raw(self: &Self, start: *const u8, end: *const u8) -> usize` - Counts all occurrences of this byte in the given haystack represented
- `fn iter<'a, 'h>(self: &'a Self, haystack: &'h [u8]) -> OneIter<'a, 'h>` - Returns an iterator over all occurrences of the needle byte in the

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> One`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## memchr::arch::x86_64::avx2::memchr::OneIter

*Struct*

An iterator over all occurrences of a single byte in a haystack.

This iterator implements `DoubleEndedIterator`, which means it can also be
used to find occurrences in reverse order.

This iterator is created by the [`One::iter`] method.

The lifetime parameters are as follows:

* `'a` refers to the lifetime of the underlying [`One`] searcher.
* `'h` refers to the lifetime of the haystack being searched.

**Generic Parameters:**
- 'a
- 'h

**Traits:** FusedIterator

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> OneIter<'a, 'h>`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<usize>`
  - `fn count(self: Self) -> usize`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **DoubleEndedIterator**
  - `fn next_back(self: & mut Self) -> Option<usize>`



## memchr::arch::x86_64::avx2::memchr::Three

*Struct*

Finds all occurrences of three bytes in a haystack.

That is, this reports matches of one of three possible bytes. For example,
searching for `a`, `b` or `o` in `afoobar` would report matches at offsets
`0`, `2`, `3`, `4` and `5`.

**Methods:**

- `fn new(needle1: u8, needle2: u8, needle3: u8) -> Option<Three>` - Create a new searcher that finds occurrences of the needle bytes given.
- `fn new_unchecked(needle1: u8, needle2: u8, needle3: u8) -> Three` - Create a new finder specific to AVX2 vectors and routines without
- `fn is_available() -> bool` - Returns true when this implementation is available in the current
- `fn find(self: &Self, haystack: &[u8]) -> Option<usize>` - Return the first occurrence of one of the needle bytes in the given
- `fn rfind(self: &Self, haystack: &[u8]) -> Option<usize>` - Return the last occurrence of one of the needle bytes in the given
- `fn find_raw(self: &Self, start: *const u8, end: *const u8) -> Option<*const u8>` - Like `find`, but accepts and returns raw pointers.
- `fn rfind_raw(self: &Self, start: *const u8, end: *const u8) -> Option<*const u8>` - Like `rfind`, but accepts and returns raw pointers.
- `fn iter<'a, 'h>(self: &'a Self, haystack: &'h [u8]) -> ThreeIter<'a, 'h>` - Returns an iterator over all occurrences of the needle bytes in the

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Three`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## memchr::arch::x86_64::avx2::memchr::ThreeIter

*Struct*

An iterator over all occurrences of three possible bytes in a haystack.

This iterator implements `DoubleEndedIterator`, which means it can also be
used to find occurrences in reverse order.

This iterator is created by the [`Three::iter`] method.

The lifetime parameters are as follows:

* `'a` refers to the lifetime of the underlying [`Three`] searcher.
* `'h` refers to the lifetime of the haystack being searched.

**Generic Parameters:**
- 'a
- 'h

**Traits:** FusedIterator

**Trait Implementations:**

- **DoubleEndedIterator**
  - `fn next_back(self: & mut Self) -> Option<usize>`
- **Clone**
  - `fn clone(self: &Self) -> ThreeIter<'a, 'h>`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<usize>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## memchr::arch::x86_64::avx2::memchr::Two

*Struct*

Finds all occurrences of two bytes in a haystack.

That is, this reports matches of one of two possible bytes. For example,
searching for `a` or `b` in `afoobar` would report matches at offsets `0`,
`4` and `5`.

**Methods:**

- `fn new(needle1: u8, needle2: u8) -> Option<Two>` - Create a new searcher that finds occurrences of the needle bytes given.
- `fn new_unchecked(needle1: u8, needle2: u8) -> Two` - Create a new finder specific to AVX2 vectors and routines without
- `fn is_available() -> bool` - Returns true when this implementation is available in the current
- `fn find(self: &Self, haystack: &[u8]) -> Option<usize>` - Return the first occurrence of one of the needle bytes in the given
- `fn rfind(self: &Self, haystack: &[u8]) -> Option<usize>` - Return the last occurrence of one of the needle bytes in the given
- `fn find_raw(self: &Self, start: *const u8, end: *const u8) -> Option<*const u8>` - Like `find`, but accepts and returns raw pointers.
- `fn rfind_raw(self: &Self, start: *const u8, end: *const u8) -> Option<*const u8>` - Like `rfind`, but accepts and returns raw pointers.
- `fn iter<'a, 'h>(self: &'a Self, haystack: &'h [u8]) -> TwoIter<'a, 'h>` - Returns an iterator over all occurrences of the needle bytes in the

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Two`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## memchr::arch::x86_64::avx2::memchr::TwoIter

*Struct*

An iterator over all occurrences of two possible bytes in a haystack.

This iterator implements `DoubleEndedIterator`, which means it can also be
used to find occurrences in reverse order.

This iterator is created by the [`Two::iter`] method.

The lifetime parameters are as follows:

* `'a` refers to the lifetime of the underlying [`Two`] searcher.
* `'h` refers to the lifetime of the haystack being searched.

**Generic Parameters:**
- 'a
- 'h

**Traits:** FusedIterator

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<usize>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **DoubleEndedIterator**
  - `fn next_back(self: & mut Self) -> Option<usize>`
- **Clone**
  - `fn clone(self: &Self) -> TwoIter<'a, 'h>`



