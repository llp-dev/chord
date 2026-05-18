**memchr > arch > all > twoway**

# Module: arch::all::twoway

## Contents

**Structs**

- [`Finder`](#finder) - A forward substring searcher that uses the Two-Way algorithm.
- [`FinderRev`](#finderrev) - A reverse substring searcher that uses the Two-Way algorithm.

---

## memchr::arch::all::twoway::Finder

*Struct*

A forward substring searcher that uses the Two-Way algorithm.

**Tuple Struct**: `()`

**Methods:**

- `fn new(needle: &[u8]) -> Finder` - Create a searcher that finds occurrences of the given `needle`.
- `fn find(self: &Self, haystack: &[u8], needle: &[u8]) -> Option<usize>` - Returns the first occurrence of `needle` in the given `haystack`, or

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Finder`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## memchr::arch::all::twoway::FinderRev

*Struct*

A reverse substring searcher that uses the Two-Way algorithm.

**Tuple Struct**: `()`

**Methods:**

- `fn new(needle: &[u8]) -> FinderRev` - Create a searcher that finds occurrences of the given `needle`.
- `fn rfind(self: &Self, haystack: &[u8], needle: &[u8]) -> Option<usize>` - Returns the last occurrence of `needle` in the given `haystack`, or

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> FinderRev`



