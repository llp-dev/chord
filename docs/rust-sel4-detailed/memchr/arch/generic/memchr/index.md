*[memchr](../../../index.md) / [arch](../../index.md) / [generic](../index.md) / [memchr](index.md)*

---

# Module `memchr`

Generic crate-internal routines for the `memchr` family of functions.

## Contents

- [Structs](#structs)
  - [`One`](#one)
  - [`Two`](#two)
  - [`Three`](#three)
  - [`Iter`](#iter)
- [Functions](#functions)
  - [`search_slice_with_raw`](#search-slice-with-raw)
  - [`fwd_byte_by_byte`](#fwd-byte-by-byte)
  - [`rev_byte_by_byte`](#rev-byte-by-byte)
  - [`count_byte_by_byte`](#count-byte-by-byte)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`One`](#one) | struct | Finds all occurrences of a single byte in a haystack. |
| [`Two`](#two) | struct | Finds all occurrences of two bytes in a haystack. |
| [`Three`](#three) | struct | Finds all occurrences of two bytes in a haystack. |
| [`Iter`](#iter) | struct | An iterator over all occurrences of a set of bytes in a haystack. |
| [`search_slice_with_raw`](#search-slice-with-raw) | fn | Search a slice using a function that operates on raw pointers. |
| [`fwd_byte_by_byte`](#fwd-byte-by-byte) | fn | Performs a forward byte-at-a-time loop until either `ptr >= end_ptr` or until `confirm(*ptr)` returns `true`. |
| [`rev_byte_by_byte`](#rev-byte-by-byte) | fn | Performs a reverse byte-at-a-time loop until either `ptr < start_ptr` or until `confirm(*ptr)` returns `true`. |
| [`count_byte_by_byte`](#count-byte-by-byte) | fn | Performs a forward byte-at-a-time loop until `ptr >= end_ptr` and returns the number of times `confirm(*ptr)` returns `true`. |

## Structs

### `One<V>`

```rust
struct One<V> {
    s1: u8,
    v1: V,
}
```

Finds all occurrences of a single byte in a haystack.

#### Implementations

- <span id="one-const-loop-size"></span>`const LOOP_SIZE: usize`

- <span id="one-new"></span>`unsafe fn new(needle: u8) -> One<V>` — [`One`](#one)

  Create a new searcher that finds occurrences of the byte given.

- <span id="one-needle1"></span>`fn needle1(&self) -> u8`

  Returns the needle given to `One::new`.

- <span id="one-find-raw"></span>`unsafe fn find_raw(&self, start: *const u8, end: *const u8) -> Option<*const u8>`

  Return a pointer to the first occurrence of the needle in the given

  haystack. If no such occurrence exists, then `None` is returned.

  

  When a match is found, the pointer returned is guaranteed to be

  `>= start` and `< end`.

  

  # Safety

  

  * It must be the case that `start < end` and that the distance between

  them is at least equal to `V::BYTES`. That is, it must always be valid

  to do at least an unaligned load of `V` at `start`.

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

- <span id="one-rfind-raw"></span>`unsafe fn rfind_raw(&self, start: *const u8, end: *const u8) -> Option<*const u8>`

  Return a pointer to the last occurrence of the needle in the given

  haystack. If no such occurrence exists, then `None` is returned.

  

  When a match is found, the pointer returned is guaranteed to be

  `>= start` and `< end`.

  

  # Safety

  

  * It must be the case that `start < end` and that the distance between

  them is at least equal to `V::BYTES`. That is, it must always be valid

  to do at least an unaligned load of `V` at `start`.

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

- <span id="one-count-raw"></span>`unsafe fn count_raw(&self, start: *const u8, end: *const u8) -> usize`

  Return a count of all matching bytes in the given haystack.

  

  # Safety

  

  * It must be the case that `start < end` and that the distance between

  them is at least equal to `V::BYTES`. That is, it must always be valid

  to do at least an unaligned load of `V` at `start`.

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

- <span id="one-search-chunk"></span>`unsafe fn search_chunk(&self, cur: *const u8, mask_to_offset: impl Fn(<V as >::Mask) -> usize) -> Option<*const u8>` — [`Vector`](../../../vector/index.md#vector)

  Search `V::BYTES` starting at `cur` via an unaligned load.

  

  `mask_to_offset` should be a function that converts a `movemask` to

  an offset such that `cur.add(offset)` corresponds to a pointer to the

  match location if one is found. Generally it is expected to use either

  `mask_to_first_offset` or `mask_to_last_offset`, depending on whether

  one is implementing a forward or reverse search, respectively.

  

  # Safety

  

  `cur` must be a valid pointer and it must be valid to do an unaligned

  load of size `V::BYTES` at `cur`.

#### Trait Implementations

##### `impl<V: clone::Clone> Clone for One<V>`

- <span id="one-clone"></span>`fn clone(&self) -> One<V>` — [`One`](#one)

##### `impl<V: marker::Copy> Copy for One<V>`

##### `impl<V: fmt::Debug> Debug for One<V>`

- <span id="one-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Two<V>`

```rust
struct Two<V> {
    s1: u8,
    s2: u8,
    v1: V,
    v2: V,
}
```

Finds all occurrences of two bytes in a haystack.

That is, this reports matches of one of two possible bytes. For example,
searching for `a` or `b` in `afoobar` would report matches at offsets `0`,
`4` and `5`.

#### Implementations

- <span id="two-const-loop-size"></span>`const LOOP_SIZE: usize`

- <span id="two-new"></span>`unsafe fn new(needle1: u8, needle2: u8) -> Two<V>` — [`Two`](#two)

  Create a new searcher that finds occurrences of the byte given.

- <span id="two-needle1"></span>`fn needle1(&self) -> u8`

  Returns the first needle given to `Two::new`.

- <span id="two-needle2"></span>`fn needle2(&self) -> u8`

  Returns the second needle given to `Two::new`.

- <span id="two-find-raw"></span>`unsafe fn find_raw(&self, start: *const u8, end: *const u8) -> Option<*const u8>`

  Return a pointer to the first occurrence of one of the needles in the

  given haystack. If no such occurrence exists, then `None` is returned.

  

  When a match is found, the pointer returned is guaranteed to be

  `>= start` and `< end`.

  

  # Safety

  

  * It must be the case that `start < end` and that the distance between

  them is at least equal to `V::BYTES`. That is, it must always be valid

  to do at least an unaligned load of `V` at `start`.

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

- <span id="two-rfind-raw"></span>`unsafe fn rfind_raw(&self, start: *const u8, end: *const u8) -> Option<*const u8>`

  Return a pointer to the last occurrence of the needle in the given

  haystack. If no such occurrence exists, then `None` is returned.

  

  When a match is found, the pointer returned is guaranteed to be

  `>= start` and `< end`.

  

  # Safety

  

  * It must be the case that `start < end` and that the distance between

  them is at least equal to `V::BYTES`. That is, it must always be valid

  to do at least an unaligned load of `V` at `start`.

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

- <span id="two-search-chunk"></span>`unsafe fn search_chunk(&self, cur: *const u8, mask_to_offset: impl Fn(<V as >::Mask) -> usize) -> Option<*const u8>` — [`Vector`](../../../vector/index.md#vector)

  Search `V::BYTES` starting at `cur` via an unaligned load.

  

  `mask_to_offset` should be a function that converts a `movemask` to

  an offset such that `cur.add(offset)` corresponds to a pointer to the

  match location if one is found. Generally it is expected to use either

  `mask_to_first_offset` or `mask_to_last_offset`, depending on whether

  one is implementing a forward or reverse search, respectively.

  

  # Safety

  

  `cur` must be a valid pointer and it must be valid to do an unaligned

  load of size `V::BYTES` at `cur`.

#### Trait Implementations

##### `impl<V: clone::Clone> Clone for Two<V>`

- <span id="two-clone"></span>`fn clone(&self) -> Two<V>` — [`Two`](#two)

##### `impl<V: marker::Copy> Copy for Two<V>`

##### `impl<V: fmt::Debug> Debug for Two<V>`

- <span id="two-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Three<V>`

```rust
struct Three<V> {
    s1: u8,
    s2: u8,
    s3: u8,
    v1: V,
    v2: V,
    v3: V,
}
```

Finds all occurrences of two bytes in a haystack.

That is, this reports matches of one of two possible bytes. For example,
searching for `a` or `b` in `afoobar` would report matches at offsets `0`,
`4` and `5`.

#### Implementations

- <span id="three-const-loop-size"></span>`const LOOP_SIZE: usize`

- <span id="three-new"></span>`unsafe fn new(needle1: u8, needle2: u8, needle3: u8) -> Three<V>` — [`Three`](#three)

  Create a new searcher that finds occurrences of the byte given.

- <span id="three-needle1"></span>`fn needle1(&self) -> u8`

  Returns the first needle given to `Three::new`.

- <span id="three-needle2"></span>`fn needle2(&self) -> u8`

  Returns the second needle given to `Three::new`.

- <span id="three-needle3"></span>`fn needle3(&self) -> u8`

  Returns the third needle given to `Three::new`.

- <span id="three-find-raw"></span>`unsafe fn find_raw(&self, start: *const u8, end: *const u8) -> Option<*const u8>`

  Return a pointer to the first occurrence of one of the needles in the

  given haystack. If no such occurrence exists, then `None` is returned.

  

  When a match is found, the pointer returned is guaranteed to be

  `>= start` and `< end`.

  

  # Safety

  

  * It must be the case that `start < end` and that the distance between

  them is at least equal to `V::BYTES`. That is, it must always be valid

  to do at least an unaligned load of `V` at `start`.

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

- <span id="three-rfind-raw"></span>`unsafe fn rfind_raw(&self, start: *const u8, end: *const u8) -> Option<*const u8>`

  Return a pointer to the last occurrence of the needle in the given

  haystack. If no such occurrence exists, then `None` is returned.

  

  When a match is found, the pointer returned is guaranteed to be

  `>= start` and `< end`.

  

  # Safety

  

  * It must be the case that `start < end` and that the distance between

  them is at least equal to `V::BYTES`. That is, it must always be valid

  to do at least an unaligned load of `V` at `start`.

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

- <span id="three-search-chunk"></span>`unsafe fn search_chunk(&self, cur: *const u8, mask_to_offset: impl Fn(<V as >::Mask) -> usize) -> Option<*const u8>` — [`Vector`](../../../vector/index.md#vector)

  Search `V::BYTES` starting at `cur` via an unaligned load.

  

  `mask_to_offset` should be a function that converts a `movemask` to

  an offset such that `cur.add(offset)` corresponds to a pointer to the

  match location if one is found. Generally it is expected to use either

  `mask_to_first_offset` or `mask_to_last_offset`, depending on whether

  one is implementing a forward or reverse search, respectively.

  

  # Safety

  

  `cur` must be a valid pointer and it must be valid to do an unaligned

  load of size `V::BYTES` at `cur`.

#### Trait Implementations

##### `impl<V: clone::Clone> Clone for Three<V>`

- <span id="three-clone"></span>`fn clone(&self) -> Three<V>` — [`Three`](#three)

##### `impl<V: marker::Copy> Copy for Three<V>`

##### `impl<V: fmt::Debug> Debug for Three<V>`

- <span id="three-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Iter<'h>`

```rust
struct Iter<'h> {
    original_start: *const u8,
    start: *const u8,
    end: *const u8,
    haystack: core::marker::PhantomData<&'h [u8]>,
}
```

An iterator over all occurrences of a set of bytes in a haystack.

This iterator implements the routines necessary to provide a
`DoubleEndedIterator` impl, which means it can also be used to find
occurrences in reverse order.

The lifetime parameters are as follows:

* `'h` refers to the lifetime of the haystack being searched.

This type is intended to be used to implement all iterators for the
`memchr` family of functions. It handles a tiny bit of marginally tricky
raw pointer math, but otherwise expects the caller to provide `find_raw`
and `rfind_raw` routines for each call of `next` and `next_back`,
respectively.

#### Fields

- **`original_start`**: `*const u8`

  The original starting point into the haystack. We use this to convert
  pointers to offsets.

- **`start`**: `*const u8`

  The current starting point into the haystack. That is, where the next
  search will begin.

- **`end`**: `*const u8`

  The current ending point into the haystack. That is, where the next
  reverse search will begin.

- **`haystack`**: `core::marker::PhantomData<&'h [u8]>`

  A marker for tracking the lifetime of the start/cur_start/cur_end
  pointers above, which all point into the haystack.

#### Implementations

- <span id="iter-new"></span>`fn new(haystack: &'h [u8]) -> Iter<'h>` — [`Iter`](#iter)

  Create a new generic memchr iterator.

- <span id="iter-next"></span>`unsafe fn next(&mut self, find_raw: impl FnMut(*const u8, *const u8) -> Option<*const u8>) -> Option<usize>`

  Returns the next occurrence in the forward direction.

  

  # Safety

  

  Callers must ensure that if a pointer is returned from the closure

  provided, then it must be greater than or equal to the start pointer

  and less than the end pointer.

- <span id="iter-count"></span>`fn count(self, count_raw: impl FnMut(*const u8, *const u8) -> usize) -> usize`

  Returns the number of remaining elements in this iterator.

- <span id="iter-next-back"></span>`unsafe fn next_back(&mut self, rfind_raw: impl FnMut(*const u8, *const u8) -> Option<*const u8>) -> Option<usize>`

  Returns the next occurrence in reverse.

  

  # Safety

  

  Callers must ensure that if a pointer is returned from the closure

  provided, then it must be greater than or equal to the start pointer

  and less than the end pointer.

- <span id="iter-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

  Provides an implementation of `Iterator::size_hint`.

#### Trait Implementations

##### `impl Clone for Iter<'h>`

- <span id="iter-clone"></span>`fn clone(&self) -> Iter<'h>` — [`Iter`](#iter)

##### `impl Debug for Iter<'h>`

- <span id="iter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Send for Iter<'h>`

##### `impl Sync for Iter<'h>`

## Functions

### `search_slice_with_raw`

```rust
unsafe fn search_slice_with_raw(haystack: &[u8], find_raw: impl FnMut(*const u8, *const u8) -> Option<*const u8>) -> Option<usize>
```

Search a slice using a function that operates on raw pointers.

Given a function to search a contiguous sequence of memory for the location
of a non-empty set of bytes, this will execute that search on a slice of
bytes. The pointer returned by the given function will be converted to an
offset relative to the starting point of the given slice. That is, if a
match is found, the offset returned by this routine is guaranteed to be a
valid index into `haystack`.

Callers may use this for a forward or reverse search.

# Safety

Callers must ensure that if a pointer is returned by `find_raw`, then the
pointer must be greater than or equal to the starting pointer and less than
the end pointer.

### `fwd_byte_by_byte`

```rust
unsafe fn fwd_byte_by_byte<F: Fn(u8) -> bool>(start: *const u8, end: *const u8, confirm: F) -> Option<*const u8>
```

Performs a forward byte-at-a-time loop until either `ptr >= end_ptr` or
until `confirm(*ptr)` returns `true`. If the former occurs, then `None` is
returned. If the latter occurs, then the pointer at which `confirm` returns
`true` is returned.

# Safety

Callers must provide valid pointers and they must satisfy `start_ptr <=
ptr` and `ptr <= end_ptr`.

### `rev_byte_by_byte`

```rust
unsafe fn rev_byte_by_byte<F: Fn(u8) -> bool>(start: *const u8, end: *const u8, confirm: F) -> Option<*const u8>
```

Performs a reverse byte-at-a-time loop until either `ptr < start_ptr` or
until `confirm(*ptr)` returns `true`. If the former occurs, then `None` is
returned. If the latter occurs, then the pointer at which `confirm` returns
`true` is returned.

# Safety

Callers must provide valid pointers and they must satisfy `start_ptr <=
ptr` and `ptr <= end_ptr`.

### `count_byte_by_byte`

```rust
unsafe fn count_byte_by_byte<F: Fn(u8) -> bool>(start: *const u8, end: *const u8, confirm: F) -> usize
```

Performs a forward byte-at-a-time loop until `ptr >= end_ptr` and returns
the number of times `confirm(*ptr)` returns `true`.

# Safety

Callers must provide valid pointers and they must satisfy `start_ptr <=
ptr` and `ptr <= end_ptr`.

