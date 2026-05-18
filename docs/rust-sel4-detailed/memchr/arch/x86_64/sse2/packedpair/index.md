*[memchr](../../../../index.md) / [arch](../../../index.md) / [x86_64](../../index.md) / [sse2](../index.md) / [packedpair](index.md)*

---

# Module `packedpair`

A 128-bit vector implementation of the "packed pair" SIMD algorithm.

The "packed pair" algorithm is based on the [generic SIMD] algorithm. The main
difference is that it (by default) uses a background distribution of byte
frequencies to heuristically select the pair of bytes to search for.


## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Finder`](#finder) | struct | A "packed pair" finder that uses 128-bit vector operations. |

## Structs

### `Finder`

```rust
struct Finder(packedpair::Finder<core::arch::x86_64::__m128i>);
```

A "packed pair" finder that uses 128-bit vector operations.

This finder picks two bytes that it believes have high predictive power
for indicating an overall match of a needle. Depending on whether
`Finder::find` or `Finder::find_prefilter` is used, it reports offsets
where the needle matches or could match. In the prefilter case, candidates
are reported whenever the [`Pair`](../../../all/packedpair/index.md) of bytes given matches.

#### Implementations

- <span id="finder-new"></span>`fn new(needle: &[u8]) -> Option<Finder>` — [`Finder`](#finder)

  Create a new pair searcher. The searcher returned can either report

  exact matches of `needle` or act as a prefilter and report candidate

  positions of `needle`.

  

  If SSE2 is unavailable in the current environment or if a [`Pair`](../../../all/packedpair/index.md)

  could not be constructed from the needle given, then `None` is

  returned.

- <span id="finder-with-pair"></span>`fn with_pair(needle: &[u8], pair: Pair) -> Option<Finder>` — [`Pair`](../../../all/packedpair/index.md#pair), [`Finder`](#finder)

  Create a new "packed pair" finder using the pair of bytes given.

  

  This constructor permits callers to control precisely which pair of

  bytes is used as a predicate.

  

  If SSE2 is unavailable in the current environment, then `None` is

  returned.

- <span id="finder-with-pair-impl"></span>`unsafe fn with_pair_impl(needle: &[u8], pair: Pair) -> Finder` — [`Pair`](../../../all/packedpair/index.md#pair), [`Finder`](#finder)

  Create a new `Finder` specific to SSE2 vectors and routines.

  

  # Safety

  

  Same as the safety for `packedpair::Finder::new`, and callers must also

  ensure that SSE2 is available.

- <span id="finder-is-available"></span>`fn is_available() -> bool`

  Returns true when this implementation is available in the current

  environment.

  

  When this is true, it is guaranteed that `Finder::with_pair` will

  return a `Some` value. Similarly, when it is false, it is guaranteed

  that `Finder::with_pair` will return a `None` value. Notice that this

  does not guarantee that `Finder::new` will return a `Finder`. Namely,

  even when `Finder::is_available` is true, it is not guaranteed that a

  valid [`Pair`](../../../all/packedpair/index.md) can be found from the needle given.

  

  Note also that for the lifetime of a single program, if this returns

  true then it will always return true.

- <span id="finder-find"></span>`fn find(&self, haystack: &[u8], needle: &[u8]) -> Option<usize>`

  Execute a search using SSE2 vectors and routines.

  

  # Panics

  

  When `haystack.len()` is less than `Finder::min_haystack_len`.

- <span id="finder-find-prefilter"></span>`fn find_prefilter(&self, haystack: &[u8]) -> Option<usize>`

  Run this finder on the given haystack as a prefilter.

  

  If a candidate match is found, then an offset where the needle *could*

  begin in the haystack is returned.

  

  # Panics

  

  When `haystack.len()` is less than `Finder::min_haystack_len`.

- <span id="finder-find-impl"></span>`unsafe fn find_impl(&self, haystack: &[u8], needle: &[u8]) -> Option<usize>`

  Execute a search using SSE2 vectors and routines.

  

  # Panics

  

  When `haystack.len()` is less than `Finder::min_haystack_len`.

  

  # Safety

  

  (The target feature safety obligation is automatically fulfilled by

  virtue of being a method on `Finder`, which can only be constructed

  when it is safe to call `sse2` routines.)

- <span id="finder-find-prefilter-impl"></span>`unsafe fn find_prefilter_impl(&self, haystack: &[u8]) -> Option<usize>`

  Execute a prefilter search using SSE2 vectors and routines.

  

  # Panics

  

  When `haystack.len()` is less than `Finder::min_haystack_len`.

  

  # Safety

  

  (The target feature safety obligation is automatically fulfilled by

  virtue of being a method on `Finder`, which can only be constructed

  when it is safe to call `sse2` routines.)

- <span id="finder-pair"></span>`fn pair(&self) -> &Pair` — [`Pair`](../../../all/packedpair/index.md#pair)

  Returns the pair of offsets (into the needle) used to check as a

  predicate before confirming whether a needle exists at a particular

  position.

- <span id="finder-min-haystack-len"></span>`fn min_haystack_len(&self) -> usize`

  Returns the minimum haystack length that this `Finder` can search.

  

  Using a haystack with length smaller than this in a search will result

  in a panic. The reason for this restriction is that this finder is

  meant to be a low-level component that is part of a larger substring

  strategy. In that sense, it avoids trying to handle all cases and

  instead only handles the cases that it can handle very well.

#### Trait Implementations

##### `impl Clone for Finder`

- <span id="finder-clone"></span>`fn clone(&self) -> Finder` — [`Finder`](#finder)

##### `impl Copy for Finder`

##### `impl Debug for Finder`

- <span id="finder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

