**memchr > arch > x86_64 > avx2 > packedpair**

# Module: arch::x86_64::avx2::packedpair

## Contents

**Structs**

- [`Finder`](#finder) - A "packed pair" finder that uses 256-bit vector operations.

---

## memchr::arch::x86_64::avx2::packedpair::Finder

*Struct*

A "packed pair" finder that uses 256-bit vector operations.

This finder picks two bytes that it believes have high predictive power
for indicating an overall match of a needle. Depending on whether
`Finder::find` or `Finder::find_prefilter` is used, it reports offsets
where the needle matches or could match. In the prefilter case, candidates
are reported whenever the [`Pair`] of bytes given matches.

**Methods:**

- `fn new(needle: &[u8]) -> Option<Finder>` - Create a new pair searcher. The searcher returned can either report
- `fn with_pair(needle: &[u8], pair: Pair) -> Option<Finder>` - Create a new "packed pair" finder using the pair of bytes given.
- `fn is_available() -> bool` - Returns true when this implementation is available in the current
- `fn find(self: &Self, haystack: &[u8], needle: &[u8]) -> Option<usize>` - Execute a search using AVX2 vectors and routines.
- `fn find_prefilter(self: &Self, haystack: &[u8]) -> Option<usize>` - Run this finder on the given haystack as a prefilter.
- `fn pair(self: &Self) -> &Pair` - Returns the pair of offsets (into the needle) used to check as a
- `fn min_haystack_len(self: &Self) -> usize` - Returns the minimum haystack length that this `Finder` can search.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Finder`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



