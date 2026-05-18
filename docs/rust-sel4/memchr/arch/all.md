**memchr > arch > all**

# Module: arch::all

## Contents

**Modules**

- [`memchr`](#memchr) - Provides architecture independent implementations of `memchr` and friends.
- [`packedpair`](#packedpair) - Provides an architecture independent implementation of the "packed pair"
- [`rabinkarp`](#rabinkarp) - An implementation of the [Rabin-Karp substring search algorithm][rabinkarp].
- [`shiftor`](#shiftor) - An implementation of the [Shift-Or substring search algorithm][shiftor].
- [`twoway`](#twoway) - An implementation of the [Two-Way substring search algorithm][two-way].

**Functions**

- [`is_equal`](#is_equal) - Compare corresponding bytes in `x` and `y` for equality.
- [`is_equal_raw`](#is_equal_raw) - Compare `n` bytes at the given pointers for equality.
- [`is_prefix`](#is_prefix) - Returns true if and only if `needle` is a prefix of `haystack`.
- [`is_suffix`](#is_suffix) - Returns true if and only if `needle` is a suffix of `haystack`.

---

## memchr::arch::all::is_equal

*Function*

Compare corresponding bytes in `x` and `y` for equality.

That is, this returns true if and only if `x.len() == y.len()` and
`x[i] == y[i]` for all `0 <= i < x.len()`.

# Inlining

This routine is marked `inline(always)`. If you want to call this function
in a way that is not always inlined, you'll need to wrap a call to it in
another function that is marked as `inline(never)` or just `inline`.

# Motivation

Why not use slice equality instead? Well, slice equality usually results in
a call out to the current platform's `libc` which might not be inlineable
or have other overhead. This routine isn't guaranteed to be a win, but it
might be in some cases.

```rust
fn is_equal(x: &[u8], y: &[u8]) -> bool
```



## memchr::arch::all::is_equal_raw

*Function*

Compare `n` bytes at the given pointers for equality.

This returns true if and only if `*x.add(i) == *y.add(i)` for all
`0 <= i < n`.

# Inlining

This routine is marked `inline(always)`. If you want to call this function
in a way that is not always inlined, you'll need to wrap a call to it in
another function that is marked as `inline(never)` or just `inline`.

# Motivation

Why not use slice equality instead? Well, slice equality usually results in
a call out to the current platform's `libc` which might not be inlineable
or have other overhead. This routine isn't guaranteed to be a win, but it
might be in some cases.

# Safety

* Both `x` and `y` must be valid for reads of up to `n` bytes.
* Both `x` and `y` must point to an initialized value.
* Both `x` and `y` must each point to an allocated object and
must either be in bounds or at most one byte past the end of the
allocated object. `x` and `y` do not need to point to the same allocated
object, but they may.
* Both `x` and `y` must be _derived from_ a pointer to their respective
allocated objects.
* The distance between `x` and `x+n` must not overflow `isize`. Similarly
for `y` and `y+n`.
* The distance being in bounds must not rely on "wrapping around" the
address space.

```rust
fn is_equal_raw(x: *const u8, y: *const u8, n: usize) -> bool
```



## memchr::arch::all::is_prefix

*Function*

Returns true if and only if `needle` is a prefix of `haystack`.

This uses a latency optimized variant of `memcmp` internally which *might*
make this faster for very short strings.

# Inlining

This routine is marked `inline(always)`. If you want to call this function
in a way that is not always inlined, you'll need to wrap a call to it in
another function that is marked as `inline(never)` or just `inline`.

```rust
fn is_prefix(haystack: &[u8], needle: &[u8]) -> bool
```



## memchr::arch::all::is_suffix

*Function*

Returns true if and only if `needle` is a suffix of `haystack`.

This uses a latency optimized variant of `memcmp` internally which *might*
make this faster for very short strings.

# Inlining

This routine is marked `inline(always)`. If you want to call this function
in a way that is not always inlined, you'll need to wrap a call to it in
another function that is marked as `inline(never)` or just `inline`.

```rust
fn is_suffix(haystack: &[u8], needle: &[u8]) -> bool
```



## Module: memchr

Provides architecture independent implementations of `memchr` and friends.

The main types in this module are [`One`], [`Two`] and [`Three`]. They are for
searching for one, two or three distinct bytes, respectively, in a haystack.
Each type also has corresponding double ended iterators. These searchers
are typically slower than hand-coded vector routines accomplishing the same
task, but are also typically faster than naive scalar code. These routines
effectively work by treating a `usize` as a vector of 8-bit lanes, and thus
achieves some level of data parallelism even without explicit vector support.

The `One` searcher also provides a [`One::count`] routine for efficiently
counting the number of times a single byte occurs in a haystack. This is
useful, for example, for counting the number of lines in a haystack. This
routine exists because it is usually faster, especially with a high match
count, than using [`One::find`] repeatedly. ([`OneIter`] specializes its
`Iterator::count` implementation to use this routine.)

Only one, two and three bytes are supported because three bytes is about
the point where one sees diminishing returns. Beyond this point and it's
probably (but not necessarily) better to just use a simple `[bool; 256]` array
or similar. However, it depends mightily on the specific work-load and the
expected match frequency.



## Module: packedpair

Provides an architecture independent implementation of the "packed pair"
algorithm.

The "packed pair" algorithm is based on the [generic SIMD] algorithm. The main
difference is that it (by default) uses a background distribution of byte
frequencies to heuristically select the pair of bytes to search for. Note that
this module provides an architecture independent version that doesn't do as
good of a job keeping the search for candidates inside a SIMD hot path. It
however can be good enough in many circumstances.

[generic SIMD]: http://0x80.pl/articles/simd-strfind.html#first-and-last



## Module: rabinkarp

An implementation of the [Rabin-Karp substring search algorithm][rabinkarp].

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

[rabinkarp]: https://en.wikipedia.org/wiki/Rabin%E2%80%93Karp_algorithm



## Module: shiftor

An implementation of the [Shift-Or substring search algorithm][shiftor].

[shiftor]: https://en.wikipedia.org/wiki/Bitap_algorithm



## Module: twoway

An implementation of the [Two-Way substring search algorithm][two-way].

[`Finder`] can be built for forward searches, while [`FinderRev`] can be built
for reverse searches.

Two-Way makes for a nice general purpose substring search algorithm because of
its time and space complexity properties. It also performs well in practice.
Namely, with `m = len(needle)` and `n = len(haystack)`, Two-Way takes `O(m)`
time to create a finder, `O(1)` space and `O(n)` search time. In other words,
the preprocessing step is quick, doesn't require any heap memory and the worst
case search time is guaranteed to be linear in the haystack regardless of the
size of the needle.

While vector algorithms will usually beat Two-Way handedly, vector algorithms
also usually have pathological or edge cases that are better handled by Two-Way.
Moreover, not all targets support vector algorithms or implementations for them
simply may not exist yet.

Two-Way can be found in the `memmem` implementations in at least [GNU libc] and
[musl].

[two-way]: https://en.wikipedia.org/wiki/Two-way_string-matching_algorithm
[GNU libc]: https://www.gnu.org/software/libc/
[musl]: https://www.musl-libc.org/



