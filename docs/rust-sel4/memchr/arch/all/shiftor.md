**memchr > arch > all > shiftor**

# Module: arch::all::shiftor

## Contents

**Structs**

- [`Finder`](#finder) - A forward substring searcher using the Shift-Or algorithm.

---

## memchr::arch::all::shiftor::Finder

*Struct*

A forward substring searcher using the Shift-Or algorithm.

**Methods:**

- `fn new(needle: &[u8]) -> Option<Finder>` - Create a new Shift-Or forward searcher for the given `needle`.
- `fn find(self: &Self, haystack: &[u8]) -> Option<usize>` - Return the first occurrence of the needle given to `Finder::new` in

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



