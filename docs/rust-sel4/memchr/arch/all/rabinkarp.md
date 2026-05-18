**memchr > arch > all > rabinkarp**

# Module: arch::all::rabinkarp

## Contents

**Structs**

- [`Finder`](#finder) - A forward substring searcher using the Rabin-Karp algorithm.
- [`FinderRev`](#finderrev) - A reverse substring searcher using the Rabin-Karp algorithm.

---

## memchr::arch::all::rabinkarp::Finder

*Struct*

A forward substring searcher using the Rabin-Karp algorithm.

Note that, as a lower level API, a `Finder` does not have access to the
needle it was constructed with. For this reason, executing a search
with a `Finder` requires passing both the needle and the haystack,
where the needle is exactly equivalent to the one given to the `Finder`
at construction time. This design was chosen so that callers can have
more precise control over where and how many times a needle is stored.
For example, in cases where Rabin-Karp is just one of several possible
substring search algorithms.

**Methods:**

- `fn new(needle: &[u8]) -> Finder` - Create a new Rabin-Karp forward searcher for the given `needle`.
- `fn find(self: &Self, haystack: &[u8], needle: &[u8]) -> Option<usize>` - Return the first occurrence of the `needle` in the `haystack`
- `fn find_raw(self: &Self, hstart: *const u8, hend: *const u8, nstart: *const u8, nend: *const u8) -> Option<*const u8>` - Like `find`, but accepts and returns raw pointers.

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Finder`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## memchr::arch::all::rabinkarp::FinderRev

*Struct*

A reverse substring searcher using the Rabin-Karp algorithm.

**Tuple Struct**: `()`

**Methods:**

- `fn new(needle: &[u8]) -> FinderRev` - Create a new Rabin-Karp reverse searcher for the given `needle`.
- `fn rfind(self: &Self, haystack: &[u8], needle: &[u8]) -> Option<usize>` - Return the last occurrence of the `needle` in the `haystack`
- `fn rfind_raw(self: &Self, hstart: *const u8, hend: *const u8, nstart: *const u8, nend: *const u8) -> Option<*const u8>` - Like `rfind`, but accepts and returns raw pointers.

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> FinderRev`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



