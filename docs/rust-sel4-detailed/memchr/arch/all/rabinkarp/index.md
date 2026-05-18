*[memchr](../../../index.md) / [arch](../../index.md) / [all](../index.md) / [rabinkarp](index.md)*

---

# Module `rabinkarp`

An implementation of the [Rabin-Karp substring search algorithm][rabinkarp](#rabinkarp).

Rabin-Karp works by creating a hash of the needle provided and then computing
a rolling hash for each needle sized window in the haystack. When the rolling
hash matches the hash of the needle, a byte-wise comparison is done to check
if a match exists. The worst case time complexity of Rabin-Karp is `O(m *
n)` where `m ~ len(needle)` and `n ~ len(haystack)`. Its worst case space
complexity is constant.

The main utility of Rabin-Karp is that the searcher can be constructed very
quickly with very little memory. This makes it especially useful when searching
for small needles in small haystacks, as it might finish its search before a
beefier algorithm (like Two-Way) even starts.


## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Finder`](#finder) | struct | A forward substring searcher using the Rabin-Karp algorithm. |
| [`FinderRev`](#finderrev) | struct | A reverse substring searcher using the Rabin-Karp algorithm. |
| [`Hash`](#hash) | struct | A Rabin-Karp hash. |
| [`is_fast`](#is-fast) | fn | Whether RK is believed to be very fast for the given needle/haystack. |
| [`is_equal_raw`](#is-equal-raw) | fn | Returns true when `x[i] == y[i]` for all `0 <= i < n`. |

## Structs

### `Finder`

```rust
struct Finder {
    hash: Hash,
    hash_2pow: u32,
}
```

A forward substring searcher using the Rabin-Karp algorithm.

Note that, as a lower level API, a `Finder` does not have access to the
needle it was constructed with. For this reason, executing a search
with a `Finder` requires passing both the needle and the haystack,
where the needle is exactly equivalent to the one given to the `Finder`
at construction time. This design was chosen so that callers can have
more precise control over where and how many times a needle is stored.
For example, in cases where Rabin-Karp is just one of several possible
substring search algorithms.

#### Fields

- **`hash`**: `Hash`

  The actual hash.

- **`hash_2pow`**: `u32`

  The factor needed to multiply a byte by in order to subtract it from
  the hash. It is defined to be 2^(n-1) (using wrapping exponentiation),
  where n is the length of the needle. This is how we "remove" a byte
  from the hash once the hash window rolls past it.

#### Implementations

- <span id="finder-new"></span>`fn new(needle: &[u8]) -> Finder` — [`Finder`](#finder)

  Create a new Rabin-Karp forward searcher for the given `needle`.

  

  The needle may be empty. The empty needle matches at every byte offset.

  

  Note that callers must pass the same needle to all search calls using

  this `Finder`.

- <span id="finder-find"></span>`fn find(&self, haystack: &[u8], needle: &[u8]) -> Option<usize>`

  Return the first occurrence of the `needle` in the `haystack`

  given. If no such occurrence exists, then `None` is returned.

  

  The `needle` provided must match the needle given to this finder at

  construction time.

  

  The maximum value this can return is `haystack.len()`, which can only

  occur when the needle and haystack both have length zero. Otherwise,

  for non-empty haystacks, the maximum value is `haystack.len() - 1`.

- <span id="finder-find-raw"></span>`unsafe fn find_raw(&self, hstart: *const u8, hend: *const u8, nstart: *const u8, nend: *const u8) -> Option<*const u8>`

  Like `find`, but accepts and returns raw pointers.

  

  When a match is found, the pointer returned is guaranteed to be

  `>= start` and `<= end`. The pointer returned is only ever equivalent

  to `end` when both the needle and haystack are empty. (That is, the

  empty string matches the empty string.)

  

  This routine is useful if you're already using raw pointers and would

  like to avoid converting back to a slice before executing a search.

  

  # Safety

  

  Note that `start` and `end` below refer to both pairs of pointers given

  to this routine. That is, the conditions apply to both `hstart`/`hend`

  and `nstart`/`nend`.

  

  * Both `start` and `end` must be valid for reads.

  * Both `start` and `end` must point to an initialized value.

  * Both `start` and `end` must point to the same allocated object and

  must either be in bounds or at most one byte past the end of the

  allocated object.

  * Both `start` and `end` must be _derived from_ a pointer to the same

  object.

  * The distance between `start` and `end` must not overflow `isize`.

  * The distance being in bounds must not rely on "wrapping around" the

  address space.

  * It must be the case that `start <= end`.

#### Trait Implementations

##### `impl Clone for Finder`

- <span id="finder-clone"></span>`fn clone(&self) -> Finder` — [`Finder`](#finder)

##### `impl Debug for Finder`

- <span id="finder-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `FinderRev`

```rust
struct FinderRev(Finder);
```

A reverse substring searcher using the Rabin-Karp algorithm.

#### Implementations

- <span id="finderrev-new"></span>`fn new(needle: &[u8]) -> FinderRev` — [`FinderRev`](#finderrev)

  Create a new Rabin-Karp reverse searcher for the given `needle`.

- <span id="finderrev-rfind"></span>`fn rfind(&self, haystack: &[u8], needle: &[u8]) -> Option<usize>`

  Return the last occurrence of the `needle` in the `haystack`

  given. If no such occurrence exists, then `None` is returned.

  

  The `needle` provided must match the needle given to this finder at

  construction time.

  

  The maximum value this can return is `haystack.len()`, which can only

  occur when the needle and haystack both have length zero. Otherwise,

  for non-empty haystacks, the maximum value is `haystack.len() - 1`.

- <span id="finderrev-rfind-raw"></span>`unsafe fn rfind_raw(&self, hstart: *const u8, hend: *const u8, nstart: *const u8, nend: *const u8) -> Option<*const u8>`

  Like `rfind`, but accepts and returns raw pointers.

  

  When a match is found, the pointer returned is guaranteed to be

  `>= start` and `<= end`. The pointer returned is only ever equivalent

  to `end` when both the needle and haystack are empty. (That is, the

  empty string matches the empty string.)

  

  This routine is useful if you're already using raw pointers and would

  like to avoid converting back to a slice before executing a search.

  

  # Safety

  

  Note that `start` and `end` below refer to both pairs of pointers given

  to this routine. That is, the conditions apply to both `hstart`/`hend`

  and `nstart`/`nend`.

  

  * Both `start` and `end` must be valid for reads.

  * Both `start` and `end` must point to an initialized value.

  * Both `start` and `end` must point to the same allocated object and

  must either be in bounds or at most one byte past the end of the

  allocated object.

  * Both `start` and `end` must be _derived from_ a pointer to the same

  object.

  * The distance between `start` and `end` must not overflow `isize`.

  * The distance being in bounds must not rely on "wrapping around" the

  address space.

  * It must be the case that `start <= end`.

#### Trait Implementations

##### `impl Clone for FinderRev`

- <span id="finderrev-clone"></span>`fn clone(&self) -> FinderRev` — [`FinderRev`](#finderrev)

##### `impl Debug for FinderRev`

- <span id="finderrev-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Hash`

```rust
struct Hash(u32);
```

A Rabin-Karp hash. This might represent the hash of a needle, or the hash
of a rolling window in the haystack.

#### Implementations

- <span id="hash-new"></span>`fn new() -> Hash` — [`Hash`](#hash)

  Create a new hash that represents the empty string.

- <span id="hash-forward"></span>`unsafe fn forward(start: *const u8, end: *const u8) -> Hash` — [`Hash`](#hash)

  Create a new hash from the bytes given for use in forward searches.

  

  # Safety

  

  The given pointers must be valid to read from within their range.

- <span id="hash-reverse"></span>`unsafe fn reverse(start: *const u8, end: *const u8) -> Hash` — [`Hash`](#hash)

  Create a new hash from the bytes given for use in reverse searches.

  

  # Safety

  

  The given pointers must be valid to read from within their range.

- <span id="hash-roll"></span>`fn roll(&mut self, finder: &Finder, old: u8, new: u8)` — [`Finder`](#finder)

  Add 'new' and remove 'old' from this hash. The given needle hash should

  correspond to the hash computed for the needle being searched for.

  

  This is meant to be used when the rolling window of the haystack is

  advanced.

- <span id="hash-add"></span>`fn add(&mut self, byte: u8)`

  Add a byte to this hash.

- <span id="hash-del"></span>`fn del(&mut self, finder: &Finder, byte: u8)` — [`Finder`](#finder)

  Remove a byte from this hash. The given needle hash should correspond

  to the hash computed for the needle being searched for.

#### Trait Implementations

##### `impl Clone for Hash`

- <span id="hash-clone"></span>`fn clone(&self) -> Hash` — [`Hash`](#hash)

##### `impl Copy for Hash`

##### `impl Debug for Hash`

- <span id="hash-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Hash`

- <span id="hash-default"></span>`fn default() -> Hash` — [`Hash`](#hash)

##### `impl Eq for Hash`

##### `impl PartialEq for Hash`

- <span id="hash-partialeq-eq"></span>`fn eq(&self, other: &Hash) -> bool` — [`Hash`](#hash)

##### `impl StructuralPartialEq for Hash`

## Functions

### `is_fast`

```rust
fn is_fast(haystack: &[u8], _needle: &[u8]) -> bool
```

Whether RK is believed to be very fast for the given needle/haystack.

### `is_equal_raw`

```rust
unsafe fn is_equal_raw(x: *const u8, y: *const u8, n: usize) -> bool
```

Returns true when `x[i] == y[i]` for all `0 <= i < n`.

We forcefully don't inline this to hint at the compiler that it is unlikely
to be called. This causes the inner rabinkarp loop above to be a bit
tighter and leads to some performance improvement. See the
memmem/krate/prebuilt/sliceslice-words/words benchmark.

# Safety

Same as `crate::arch::all::is_equal_raw`.

