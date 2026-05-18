*[memchr](../../../../index.md) / [arch](../../../index.md) / [x86_64](../../index.md) / [avx2](../index.md) / [memchr](index.md)*

---

# Module `memchr`

This module defines 256-bit vector implementations of `memchr` and friends.

The main types in this module are [`One`](#one), [`Two`](#two) and [`Three`](#three). They are for
searching for one, two or three distinct bytes, respectively, in a haystack.
Each type also has corresponding double ended iterators. These searchers are
typically much faster than scalar routines accomplishing the same task.

The `One` searcher also provides a `One::count` routine for efficiently
counting the number of times a single byte occurs in a haystack. This is
useful, for example, for counting the number of lines in a haystack. This
routine exists because it is usually faster, especially with a high match
count, then using `One::find` repeatedly. ([`OneIter`](#oneiter) specializes its
`Iterator::count` implementation to use this routine.)

Only one, two and three bytes are supported because three bytes is about
the point where one sees diminishing returns. Beyond this point and it's
probably (but not necessarily) better to just use a simple `[bool; 256]` array
or similar. However, it depends mightily on the specific work-load and the
expected match frequency.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`One`](#one) | struct | Finds all occurrences of a single byte in a haystack. |
| [`OneIter`](#oneiter) | struct | An iterator over all occurrences of a single byte in a haystack. |
| [`Two`](#two) | struct | Finds all occurrences of two bytes in a haystack. |
| [`TwoIter`](#twoiter) | struct | An iterator over all occurrences of two possible bytes in a haystack. |
| [`Three`](#three) | struct | Finds all occurrences of three bytes in a haystack. |
| [`ThreeIter`](#threeiter) | struct | An iterator over all occurrences of three possible bytes in a haystack. |

## Structs

### `One`

```rust
struct One {
    sse2: generic::One<core::arch::x86_64::__m128i>,
    avx2: generic::One<core::arch::x86_64::__m256i>,
}
```

Finds all occurrences of a single byte in a haystack.

#### Fields

- **`sse2`**: `generic::One<core::arch::x86_64::__m128i>`

  Used for haystacks less than 32 bytes.

- **`avx2`**: `generic::One<core::arch::x86_64::__m256i>`

  Used for haystacks bigger than 32 bytes.

#### Implementations

- <span id="one-new"></span>`fn new(needle: u8) -> Option<One>` — [`One`](#one)

  Create a new searcher that finds occurrences of the needle byte given.

  

  This particular searcher is specialized to use AVX2 vector instructions

  that typically make it quite fast. (SSE2 is used for haystacks that

  are too short to accommodate an AVX2 vector.)

  

  If either SSE2 or AVX2 is unavailable in the current environment, then

  `None` is returned.

- <span id="one-new-unchecked"></span>`unsafe fn new_unchecked(needle: u8) -> One` — [`One`](#one)

  Create a new finder specific to AVX2 vectors and routines without

  checking that either SSE2 or AVX2 is available.

  

  # Safety

  

  Callers must guarantee that it is safe to execute both `sse2` and

  `avx2` instructions in the current environment.

  

  Note that it is a common misconception that if one compiles for an

  `x86_64` target, then they therefore automatically have access to SSE2

  instructions. While this is almost always the case, it isn't true in

  100% of cases.

- <span id="one-is-available"></span>`fn is_available() -> bool`

  Returns true when this implementation is available in the current

  environment.

  

  When this is true, it is guaranteed that `One::new` will return

  a `Some` value. Similarly, when it is false, it is guaranteed that

  `One::new` will return a `None` value.

  

  Note also that for the lifetime of a single program, if this returns

  true then it will always return true.

- <span id="one-find"></span>`fn find(&self, haystack: &[u8]) -> Option<usize>`

  Return the first occurrence of one of the needle bytes in the given

  haystack. If no such occurrence exists, then `None` is returned.

  

  The occurrence is reported as an offset into `haystack`. Its maximum

  value is `haystack.len() - 1`.

- <span id="one-rfind"></span>`fn rfind(&self, haystack: &[u8]) -> Option<usize>`

  Return the last occurrence of one of the needle bytes in the given

  haystack. If no such occurrence exists, then `None` is returned.

  

  The occurrence is reported as an offset into `haystack`. Its maximum

  value is `haystack.len() - 1`.

- <span id="one-count"></span>`fn count(&self, haystack: &[u8]) -> usize`

  Counts all occurrences of this byte in the given haystack.

- <span id="one-find-raw"></span>`unsafe fn find_raw(&self, start: *const u8, end: *const u8) -> Option<*const u8>`

  Like `find`, but accepts and returns raw pointers.

  

  When a match is found, the pointer returned is guaranteed to be

  `>= start` and `< end`.

  

  This routine is useful if you're already using raw pointers and would

  like to avoid converting back to a slice before executing a search.

  

  # Safety

  

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

  

  Note that callers may pass a pair of pointers such that `start >= end`.

  In that case, `None` will always be returned.

- <span id="one-rfind-raw"></span>`unsafe fn rfind_raw(&self, start: *const u8, end: *const u8) -> Option<*const u8>`

  Like `rfind`, but accepts and returns raw pointers.

  

  When a match is found, the pointer returned is guaranteed to be

  `>= start` and `< end`.

  

  This routine is useful if you're already using raw pointers and would

  like to avoid converting back to a slice before executing a search.

  

  # Safety

  

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

  

  Note that callers may pass a pair of pointers such that `start >= end`.

  In that case, `None` will always be returned.

- <span id="one-count-raw"></span>`unsafe fn count_raw(&self, start: *const u8, end: *const u8) -> usize`

  Counts all occurrences of this byte in the given haystack represented

  by raw pointers.

  

  This routine is useful if you're already using raw pointers and would

  like to avoid converting back to a slice before executing a search.

  

  # Safety

  

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

  

  Note that callers may pass a pair of pointers such that `start >= end`.

  In that case, `0` will always be returned.

- <span id="one-find-raw-sse2"></span>`unsafe fn find_raw_sse2(&self, start: *const u8, end: *const u8) -> Option<*const u8>`

  Execute a search using SSE2 vectors and routines.

  

  # Safety

  

  Same as `One::find_raw`, except the distance between `start` and

  `end` must be at least the size of an SSE2 vector (in bytes).

  

  (The target feature safety obligation is automatically fulfilled by

  virtue of being a method on `One`, which can only be constructed

  when it is safe to call `sse2`/`avx2` routines.)

- <span id="one-rfind-raw-sse2"></span>`unsafe fn rfind_raw_sse2(&self, start: *const u8, end: *const u8) -> Option<*const u8>`

  Execute a search using SSE2 vectors and routines.

  

  # Safety

  

  Same as `One::rfind_raw`, except the distance between `start` and

  `end` must be at least the size of an SSE2 vector (in bytes).

  

  (The target feature safety obligation is automatically fulfilled by

  virtue of being a method on `One`, which can only be constructed

  when it is safe to call `sse2`/`avx2` routines.)

- <span id="one-count-raw-sse2"></span>`unsafe fn count_raw_sse2(&self, start: *const u8, end: *const u8) -> usize`

  Execute a count using SSE2 vectors and routines.

  

  # Safety

  

  Same as `One::count_raw`, except the distance between `start` and

  `end` must be at least the size of an SSE2 vector (in bytes).

  

  (The target feature safety obligation is automatically fulfilled by

  virtue of being a method on `One`, which can only be constructed

  when it is safe to call `sse2`/`avx2` routines.)

- <span id="one-find-raw-avx2"></span>`unsafe fn find_raw_avx2(&self, start: *const u8, end: *const u8) -> Option<*const u8>`

  Execute a search using AVX2 vectors and routines.

  

  # Safety

  

  Same as `One::find_raw`, except the distance between `start` and

  `end` must be at least the size of an AVX2 vector (in bytes).

  

  (The target feature safety obligation is automatically fulfilled by

  virtue of being a method on `One`, which can only be constructed

  when it is safe to call `sse2`/`avx2` routines.)

- <span id="one-rfind-raw-avx2"></span>`unsafe fn rfind_raw_avx2(&self, start: *const u8, end: *const u8) -> Option<*const u8>`

  Execute a search using AVX2 vectors and routines.

  

  # Safety

  

  Same as `One::rfind_raw`, except the distance between `start` and

  `end` must be at least the size of an AVX2 vector (in bytes).

  

  (The target feature safety obligation is automatically fulfilled by

  virtue of being a method on `One`, which can only be constructed

  when it is safe to call `sse2`/`avx2` routines.)

- <span id="one-count-raw-avx2"></span>`unsafe fn count_raw_avx2(&self, start: *const u8, end: *const u8) -> usize`

  Execute a count using AVX2 vectors and routines.

  

  # Safety

  

  Same as `One::count_raw`, except the distance between `start` and

  `end` must be at least the size of an AVX2 vector (in bytes).

  

  (The target feature safety obligation is automatically fulfilled by

  virtue of being a method on `One`, which can only be constructed

  when it is safe to call `sse2`/`avx2` routines.)

- <span id="one-iter"></span>`fn iter<'a, 'h>(self: &'a Self, haystack: &'h [u8]) -> OneIter<'a, 'h>` — [`OneIter`](#oneiter)

  Returns an iterator over all occurrences of the needle byte in the

  given haystack.

  

  The iterator returned implements `DoubleEndedIterator`. This means it

  can also be used to find occurrences in reverse order.

#### Trait Implementations

##### `impl Clone for One`

- <span id="one-clone"></span>`fn clone(&self) -> One` — [`One`](#one)

##### `impl Copy for One`

##### `impl Debug for One`

- <span id="one-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `OneIter<'a, 'h>`

```rust
struct OneIter<'a, 'h> {
    searcher: &'a One,
    it: generic::Iter<'h>,
}
```

An iterator over all occurrences of a single byte in a haystack.

This iterator implements `DoubleEndedIterator`, which means it can also be
used to find occurrences in reverse order.

This iterator is created by the `One::iter` method.

The lifetime parameters are as follows:

* `'a` refers to the lifetime of the underlying [`One`](#one) searcher.
* `'h` refers to the lifetime of the haystack being searched.

#### Trait Implementations

##### `impl Clone for OneIter<'a, 'h>`

- <span id="oneiter-clone"></span>`fn clone(&self) -> OneIter<'a, 'h>` — [`OneIter`](#oneiter)

##### `impl Debug for OneIter<'a, 'h>`

- <span id="oneiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DoubleEndedIterator for OneIter<'a, 'h>`

- <span id="oneiter-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<usize>`

##### `impl FusedIterator for OneIter<'a, 'h>`

##### `impl IntoIterator for OneIter<'a, 'h>`

- <span id="oneiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="oneiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="oneiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for OneIter<'a, 'h>`

- <span id="oneiter-iterator-type-item"></span>`type Item = usize`

- <span id="oneiter-iterator-next"></span>`fn next(&mut self) -> Option<usize>`

- <span id="oneiter-iterator-count"></span>`fn count(self) -> usize`

- <span id="oneiter-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `Two`

```rust
struct Two {
    sse2: generic::Two<core::arch::x86_64::__m128i>,
    avx2: generic::Two<core::arch::x86_64::__m256i>,
}
```

Finds all occurrences of two bytes in a haystack.

That is, this reports matches of one of two possible bytes. For example,
searching for `a` or `b` in `afoobar` would report matches at offsets `0`,
`4` and `5`.

#### Fields

- **`sse2`**: `generic::Two<core::arch::x86_64::__m128i>`

  Used for haystacks less than 32 bytes.

- **`avx2`**: `generic::Two<core::arch::x86_64::__m256i>`

  Used for haystacks bigger than 32 bytes.

#### Implementations

- <span id="two-new"></span>`fn new(needle1: u8, needle2: u8) -> Option<Two>` — [`Two`](#two)

  Create a new searcher that finds occurrences of the needle bytes given.

  

  This particular searcher is specialized to use AVX2 vector instructions

  that typically make it quite fast. (SSE2 is used for haystacks that

  are too short to accommodate an AVX2 vector.)

  

  If either SSE2 or AVX2 is unavailable in the current environment, then

  `None` is returned.

- <span id="two-new-unchecked"></span>`unsafe fn new_unchecked(needle1: u8, needle2: u8) -> Two` — [`Two`](#two)

  Create a new finder specific to AVX2 vectors and routines without

  checking that either SSE2 or AVX2 is available.

  

  # Safety

  

  Callers must guarantee that it is safe to execute both `sse2` and

  `avx2` instructions in the current environment.

  

  Note that it is a common misconception that if one compiles for an

  `x86_64` target, then they therefore automatically have access to SSE2

  instructions. While this is almost always the case, it isn't true in

  100% of cases.

- <span id="two-is-available"></span>`fn is_available() -> bool`

  Returns true when this implementation is available in the current

  environment.

  

  When this is true, it is guaranteed that `Two::new` will return

  a `Some` value. Similarly, when it is false, it is guaranteed that

  `Two::new` will return a `None` value.

  

  Note also that for the lifetime of a single program, if this returns

  true then it will always return true.

- <span id="two-find"></span>`fn find(&self, haystack: &[u8]) -> Option<usize>`

  Return the first occurrence of one of the needle bytes in the given

  haystack. If no such occurrence exists, then `None` is returned.

  

  The occurrence is reported as an offset into `haystack`. Its maximum

  value is `haystack.len() - 1`.

- <span id="two-rfind"></span>`fn rfind(&self, haystack: &[u8]) -> Option<usize>`

  Return the last occurrence of one of the needle bytes in the given

  haystack. If no such occurrence exists, then `None` is returned.

  

  The occurrence is reported as an offset into `haystack`. Its maximum

  value is `haystack.len() - 1`.

- <span id="two-find-raw"></span>`unsafe fn find_raw(&self, start: *const u8, end: *const u8) -> Option<*const u8>`

  Like `find`, but accepts and returns raw pointers.

  

  When a match is found, the pointer returned is guaranteed to be

  `>= start` and `< end`.

  

  This routine is useful if you're already using raw pointers and would

  like to avoid converting back to a slice before executing a search.

  

  # Safety

  

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

  

  Note that callers may pass a pair of pointers such that `start >= end`.

  In that case, `None` will always be returned.

- <span id="two-rfind-raw"></span>`unsafe fn rfind_raw(&self, start: *const u8, end: *const u8) -> Option<*const u8>`

  Like `rfind`, but accepts and returns raw pointers.

  

  When a match is found, the pointer returned is guaranteed to be

  `>= start` and `< end`.

  

  This routine is useful if you're already using raw pointers and would

  like to avoid converting back to a slice before executing a search.

  

  # Safety

  

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

  

  Note that callers may pass a pair of pointers such that `start >= end`.

  In that case, `None` will always be returned.

- <span id="two-find-raw-sse2"></span>`unsafe fn find_raw_sse2(&self, start: *const u8, end: *const u8) -> Option<*const u8>`

  Execute a search using SSE2 vectors and routines.

  

  # Safety

  

  Same as `Two::find_raw`, except the distance between `start` and

  `end` must be at least the size of an SSE2 vector (in bytes).

  

  (The target feature safety obligation is automatically fulfilled by

  virtue of being a method on `Two`, which can only be constructed

  when it is safe to call `sse2`/`avx2` routines.)

- <span id="two-rfind-raw-sse2"></span>`unsafe fn rfind_raw_sse2(&self, start: *const u8, end: *const u8) -> Option<*const u8>`

  Execute a search using SSE2 vectors and routines.

  

  # Safety

  

  Same as `Two::rfind_raw`, except the distance between `start` and

  `end` must be at least the size of an SSE2 vector (in bytes).

  

  (The target feature safety obligation is automatically fulfilled by

  virtue of being a method on `Two`, which can only be constructed

  when it is safe to call `sse2`/`avx2` routines.)

- <span id="two-find-raw-avx2"></span>`unsafe fn find_raw_avx2(&self, start: *const u8, end: *const u8) -> Option<*const u8>`

  Execute a search using AVX2 vectors and routines.

  

  # Safety

  

  Same as `Two::find_raw`, except the distance between `start` and

  `end` must be at least the size of an AVX2 vector (in bytes).

  

  (The target feature safety obligation is automatically fulfilled by

  virtue of being a method on `Two`, which can only be constructed

  when it is safe to call `sse2`/`avx2` routines.)

- <span id="two-rfind-raw-avx2"></span>`unsafe fn rfind_raw_avx2(&self, start: *const u8, end: *const u8) -> Option<*const u8>`

  Execute a search using AVX2 vectors and routines.

  

  # Safety

  

  Same as `Two::rfind_raw`, except the distance between `start` and

  `end` must be at least the size of an AVX2 vector (in bytes).

  

  (The target feature safety obligation is automatically fulfilled by

  virtue of being a method on `Two`, which can only be constructed

  when it is safe to call `sse2`/`avx2` routines.)

- <span id="two-iter"></span>`fn iter<'a, 'h>(self: &'a Self, haystack: &'h [u8]) -> TwoIter<'a, 'h>` — [`TwoIter`](#twoiter)

  Returns an iterator over all occurrences of the needle bytes in the

  given haystack.

  

  The iterator returned implements `DoubleEndedIterator`. This means it

  can also be used to find occurrences in reverse order.

#### Trait Implementations

##### `impl Clone for Two`

- <span id="two-clone"></span>`fn clone(&self) -> Two` — [`Two`](#two)

##### `impl Copy for Two`

##### `impl Debug for Two`

- <span id="two-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `TwoIter<'a, 'h>`

```rust
struct TwoIter<'a, 'h> {
    searcher: &'a Two,
    it: generic::Iter<'h>,
}
```

An iterator over all occurrences of two possible bytes in a haystack.

This iterator implements `DoubleEndedIterator`, which means it can also be
used to find occurrences in reverse order.

This iterator is created by the `Two::iter` method.

The lifetime parameters are as follows:

* `'a` refers to the lifetime of the underlying [`Two`](#two) searcher.
* `'h` refers to the lifetime of the haystack being searched.

#### Trait Implementations

##### `impl Clone for TwoIter<'a, 'h>`

- <span id="twoiter-clone"></span>`fn clone(&self) -> TwoIter<'a, 'h>` — [`TwoIter`](#twoiter)

##### `impl Debug for TwoIter<'a, 'h>`

- <span id="twoiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DoubleEndedIterator for TwoIter<'a, 'h>`

- <span id="twoiter-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<usize>`

##### `impl FusedIterator for TwoIter<'a, 'h>`

##### `impl IntoIterator for TwoIter<'a, 'h>`

- <span id="twoiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="twoiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="twoiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for TwoIter<'a, 'h>`

- <span id="twoiter-iterator-type-item"></span>`type Item = usize`

- <span id="twoiter-iterator-next"></span>`fn next(&mut self) -> Option<usize>`

- <span id="twoiter-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `Three`

```rust
struct Three {
    sse2: generic::Three<core::arch::x86_64::__m128i>,
    avx2: generic::Three<core::arch::x86_64::__m256i>,
}
```

Finds all occurrences of three bytes in a haystack.

That is, this reports matches of one of three possible bytes. For example,
searching for `a`, `b` or `o` in `afoobar` would report matches at offsets
`0`, `2`, `3`, `4` and `5`.

#### Fields

- **`sse2`**: `generic::Three<core::arch::x86_64::__m128i>`

  Used for haystacks less than 32 bytes.

- **`avx2`**: `generic::Three<core::arch::x86_64::__m256i>`

  Used for haystacks bigger than 32 bytes.

#### Implementations

- <span id="three-new"></span>`fn new(needle1: u8, needle2: u8, needle3: u8) -> Option<Three>` — [`Three`](#three)

  Create a new searcher that finds occurrences of the needle bytes given.

  

  This particular searcher is specialized to use AVX2 vector instructions

  that typically make it quite fast. (SSE2 is used for haystacks that

  are too short to accommodate an AVX2 vector.)

  

  If either SSE2 or AVX2 is unavailable in the current environment, then

  `None` is returned.

- <span id="three-new-unchecked"></span>`unsafe fn new_unchecked(needle1: u8, needle2: u8, needle3: u8) -> Three` — [`Three`](#three)

  Create a new finder specific to AVX2 vectors and routines without

  checking that either SSE2 or AVX2 is available.

  

  # Safety

  

  Callers must guarantee that it is safe to execute both `sse2` and

  `avx2` instructions in the current environment.

  

  Note that it is a common misconception that if one compiles for an

  `x86_64` target, then they therefore automatically have access to SSE2

  instructions. While this is almost always the case, it isn't true in

  100% of cases.

- <span id="three-is-available"></span>`fn is_available() -> bool`

  Returns true when this implementation is available in the current

  environment.

  

  When this is true, it is guaranteed that `Three::new` will return

  a `Some` value. Similarly, when it is false, it is guaranteed that

  `Three::new` will return a `None` value.

  

  Note also that for the lifetime of a single program, if this returns

  true then it will always return true.

- <span id="three-find"></span>`fn find(&self, haystack: &[u8]) -> Option<usize>`

  Return the first occurrence of one of the needle bytes in the given

  haystack. If no such occurrence exists, then `None` is returned.

  

  The occurrence is reported as an offset into `haystack`. Its maximum

  value is `haystack.len() - 1`.

- <span id="three-rfind"></span>`fn rfind(&self, haystack: &[u8]) -> Option<usize>`

  Return the last occurrence of one of the needle bytes in the given

  haystack. If no such occurrence exists, then `None` is returned.

  

  The occurrence is reported as an offset into `haystack`. Its maximum

  value is `haystack.len() - 1`.

- <span id="three-find-raw"></span>`unsafe fn find_raw(&self, start: *const u8, end: *const u8) -> Option<*const u8>`

  Like `find`, but accepts and returns raw pointers.

  

  When a match is found, the pointer returned is guaranteed to be

  `>= start` and `< end`.

  

  This routine is useful if you're already using raw pointers and would

  like to avoid converting back to a slice before executing a search.

  

  # Safety

  

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

  

  Note that callers may pass a pair of pointers such that `start >= end`.

  In that case, `None` will always be returned.

- <span id="three-rfind-raw"></span>`unsafe fn rfind_raw(&self, start: *const u8, end: *const u8) -> Option<*const u8>`

  Like `rfind`, but accepts and returns raw pointers.

  

  When a match is found, the pointer returned is guaranteed to be

  `>= start` and `< end`.

  

  This routine is useful if you're already using raw pointers and would

  like to avoid converting back to a slice before executing a search.

  

  # Safety

  

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

  

  Note that callers may pass a pair of pointers such that `start >= end`.

  In that case, `None` will always be returned.

- <span id="three-find-raw-sse2"></span>`unsafe fn find_raw_sse2(&self, start: *const u8, end: *const u8) -> Option<*const u8>`

  Execute a search using SSE2 vectors and routines.

  

  # Safety

  

  Same as `Three::find_raw`, except the distance between `start` and

  `end` must be at least the size of an SSE2 vector (in bytes).

  

  (The target feature safety obligation is automatically fulfilled by

  virtue of being a method on `Three`, which can only be constructed

  when it is safe to call `sse2`/`avx2` routines.)

- <span id="three-rfind-raw-sse2"></span>`unsafe fn rfind_raw_sse2(&self, start: *const u8, end: *const u8) -> Option<*const u8>`

  Execute a search using SSE2 vectors and routines.

  

  # Safety

  

  Same as `Three::rfind_raw`, except the distance between `start` and

  `end` must be at least the size of an SSE2 vector (in bytes).

  

  (The target feature safety obligation is automatically fulfilled by

  virtue of being a method on `Three`, which can only be constructed

  when it is safe to call `sse2`/`avx2` routines.)

- <span id="three-find-raw-avx2"></span>`unsafe fn find_raw_avx2(&self, start: *const u8, end: *const u8) -> Option<*const u8>`

  Execute a search using AVX2 vectors and routines.

  

  # Safety

  

  Same as `Three::find_raw`, except the distance between `start` and

  `end` must be at least the size of an AVX2 vector (in bytes).

  

  (The target feature safety obligation is automatically fulfilled by

  virtue of being a method on `Three`, which can only be constructed

  when it is safe to call `sse2`/`avx2` routines.)

- <span id="three-rfind-raw-avx2"></span>`unsafe fn rfind_raw_avx2(&self, start: *const u8, end: *const u8) -> Option<*const u8>`

  Execute a search using AVX2 vectors and routines.

  

  # Safety

  

  Same as `Three::rfind_raw`, except the distance between `start` and

  `end` must be at least the size of an AVX2 vector (in bytes).

  

  (The target feature safety obligation is automatically fulfilled by

  virtue of being a method on `Three`, which can only be constructed

  when it is safe to call `sse2`/`avx2` routines.)

- <span id="three-iter"></span>`fn iter<'a, 'h>(self: &'a Self, haystack: &'h [u8]) -> ThreeIter<'a, 'h>` — [`ThreeIter`](#threeiter)

  Returns an iterator over all occurrences of the needle bytes in the

  given haystack.

  

  The iterator returned implements `DoubleEndedIterator`. This means it

  can also be used to find occurrences in reverse order.

#### Trait Implementations

##### `impl Clone for Three`

- <span id="three-clone"></span>`fn clone(&self) -> Three` — [`Three`](#three)

##### `impl Copy for Three`

##### `impl Debug for Three`

- <span id="three-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ThreeIter<'a, 'h>`

```rust
struct ThreeIter<'a, 'h> {
    searcher: &'a Three,
    it: generic::Iter<'h>,
}
```

An iterator over all occurrences of three possible bytes in a haystack.

This iterator implements `DoubleEndedIterator`, which means it can also be
used to find occurrences in reverse order.

This iterator is created by the `Three::iter` method.

The lifetime parameters are as follows:

* `'a` refers to the lifetime of the underlying [`Three`](#three) searcher.
* `'h` refers to the lifetime of the haystack being searched.

#### Trait Implementations

##### `impl Clone for ThreeIter<'a, 'h>`

- <span id="threeiter-clone"></span>`fn clone(&self) -> ThreeIter<'a, 'h>` — [`ThreeIter`](#threeiter)

##### `impl Debug for ThreeIter<'a, 'h>`

- <span id="threeiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl DoubleEndedIterator for ThreeIter<'a, 'h>`

- <span id="threeiter-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<usize>`

##### `impl FusedIterator for ThreeIter<'a, 'h>`

##### `impl IntoIterator for ThreeIter<'a, 'h>`

- <span id="threeiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="threeiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="threeiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for ThreeIter<'a, 'h>`

- <span id="threeiter-iterator-type-item"></span>`type Item = usize`

- <span id="threeiter-iterator-next"></span>`fn next(&mut self) -> Option<usize>`

- <span id="threeiter-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

