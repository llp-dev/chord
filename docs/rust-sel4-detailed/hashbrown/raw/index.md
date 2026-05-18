*[hashbrown](../index.md) / [raw](index.md)*

---

# Module `raw`

## Contents

- [Modules](#modules)
  - [`alloc`](#alloc)
- [Structs](#structs)
  - [`ProbeSeq`](#probeseq)
  - [`TableLayout`](#tablelayout)
  - [`Bucket`](#bucket)
  - [`RawTable`](#rawtable)
  - [`RawTableInner`](#rawtableinner)
  - [`RawIterRange`](#rawiterrange)
  - [`RawIter`](#rawiter)
  - [`FullBucketsIndices`](#fullbucketsindices)
  - [`RawIntoIter`](#rawintoiter)
  - [`RawDrain`](#rawdrain)
  - [`RawIterHash`](#rawiterhash)
  - [`RawIterHashIndices`](#rawiterhashindices)
  - [`RawExtractIf`](#rawextractif)
- [Enums](#enums)
  - [`Fallibility`](#fallibility)
- [Traits](#traits)
  - [`SizedTypeProperties`](#sizedtypeproperties)
  - [`RawTableClone`](#rawtableclone)
- [Functions](#functions)
  - [`offset_from`](#offset-from)
  - [`h1`](#h1)
  - [`capacity_to_buckets`](#capacity-to-buckets)
  - [`ensure_bucket_bytes_at_least_ctrl_align`](#ensure-bucket-bytes-at-least-ctrl-align)
  - [`bucket_mask_to_capacity`](#bucket-mask-to-capacity)
  - [`prev_pow2`](#prev-pow2)
  - [`maximum_buckets_in`](#maximum-buckets-in)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`alloc`](#alloc) | mod |  |
| [`ProbeSeq`](#probeseq) | struct | Probe sequence based on triangular numbers, which is guaranteed (since our table size is a power of two) to visit every group of elements exactly once. |
| [`TableLayout`](#tablelayout) | struct | Helper which allows the max calculation for `ctrl_align` to be statically computed for each `T` while keeping the rest of `calculate_layout_for` independent of `T` |
| [`Bucket`](#bucket) | struct | A reference to a hash table bucket containing a `T`. |
| [`RawTable`](#rawtable) | struct | A raw hash table with an unsafe API. |
| [`RawTableInner`](#rawtableinner) | struct | Non-generic part of `RawTable` which allows functions to be instantiated only once regardless of how many different key-value types are used. |
| [`RawIterRange`](#rawiterrange) | struct | Iterator over a sub-range of a table. |
| [`RawIter`](#rawiter) | struct | Iterator which returns a raw pointer to every full bucket in the table. |
| [`FullBucketsIndices`](#fullbucketsindices) | struct | Iterator which returns an index of every full bucket in the table. |
| [`RawIntoIter`](#rawintoiter) | struct | Iterator which consumes a table and returns elements. |
| [`RawDrain`](#rawdrain) | struct | Iterator which consumes elements without freeing the table storage. |
| [`RawIterHash`](#rawiterhash) | struct | Iterator over occupied buckets that could match a given hash. |
| [`RawIterHashIndices`](#rawiterhashindices) | struct |  |
| [`RawExtractIf`](#rawextractif) | struct |  |
| [`Fallibility`](#fallibility) | enum | Whether memory allocation errors should return an error or abort. |
| [`SizedTypeProperties`](#sizedtypeproperties) | trait |  |
| [`RawTableClone`](#rawtableclone) | trait | Specialization of `clone_from` for `Copy` types |
| [`offset_from`](#offset-from) | fn |  |
| [`h1`](#h1) | fn | Primary hash function, used to select the initial bucket to probe from. |
| [`capacity_to_buckets`](#capacity-to-buckets) | fn | Returns the number of buckets needed to hold the given number of items, taking the maximum load factor into account. |
| [`ensure_bucket_bytes_at_least_ctrl_align`](#ensure-bucket-bytes-at-least-ctrl-align) | fn |  |
| [`bucket_mask_to_capacity`](#bucket-mask-to-capacity) | fn | Returns the maximum effective capacity for the given bucket mask, taking the maximum load factor into account. |
| [`prev_pow2`](#prev-pow2) | fn | Find the previous power of 2. |
| [`maximum_buckets_in`](#maximum-buckets-in) | fn | Finds the largest number of buckets that can fit in `allocation_size` provided the given TableLayout. |

## Modules

- [`alloc`](alloc/index.md)

## Structs

### `ProbeSeq`

```rust
struct ProbeSeq {
    pos: usize,
    stride: usize,
}
```

Probe sequence based on triangular numbers, which is guaranteed (since our
table size is a power of two) to visit every group of elements exactly once.

A triangular probe has us jump by 1 more group every time. So first we
jump by 1 group (meaning we just continue our linear scan), then 2 groups
(skipping over 1 group), then 3 groups (skipping over 2 groups), and so on.

Proof that the probe will visit every group in the table:
<https://fgiesen.wordpress.com/2015/02/22/triangular-numbers-mod-2n/>

#### Implementations

- <span id="probeseq-move-next"></span>`fn move_next(&mut self, bucket_mask: usize)`

#### Trait Implementations

##### `impl Clone for ProbeSeq`

- <span id="probeseq-clone"></span>`fn clone(&self) -> ProbeSeq` — [`ProbeSeq`](#probeseq)

### `TableLayout`

```rust
struct TableLayout {
    size: usize,
    ctrl_align: usize,
}
```

Helper which allows the max calculation for `ctrl_align` to be statically computed for each `T`
while keeping the rest of `calculate_layout_for` independent of `T`

#### Implementations

- <span id="tablelayout-new"></span>`const fn new<T>() -> Self`

- <span id="tablelayout-calculate-layout-for"></span>`fn calculate_layout_for(self, buckets: usize) -> Option<(Layout, usize)>`

#### Trait Implementations

##### `impl Clone for TableLayout`

- <span id="tablelayout-clone"></span>`fn clone(&self) -> TableLayout` — [`TableLayout`](#tablelayout)

##### `impl Copy for TableLayout`

### `Bucket<T>`

```rust
struct Bucket<T> {
    ptr: core::ptr::NonNull<T>,
}
```

A reference to a hash table bucket containing a `T`.

This is usually just a pointer to the element itself. However if the element
is a ZST, then we instead track the index of the element in the table so
that `erase` works properly.

#### Implementations

- <span id="bucket-from-base-index"></span>`unsafe fn from_base_index(base: NonNull<T>, index: usize) -> Self`

  Creates a [`Bucket`](#bucket) that contain pointer to the data.

  The pointer calculation is performed by calculating the

  offset from given `base` pointer (convenience for

  `base.as_ptr().sub(index)`).

  

  `index` is in units of `T`; e.g., an `index` of 3 represents a pointer

  offset of `3 * size_of::<T>()` bytes.

  

  If the `T` is a ZST, then we instead track the index of the element

  in the table so that `erase` works properly (return

  `NonNull::new_unchecked((index + 1) as *mut T)`)

  

  # Safety

  

  If `mem::size_of::<T>() != 0`, then the safety rules are directly derived

  from the safety rules for `<*mut T>::sub` method of `*mut T` and the safety

  rules of `NonNull::new_unchecked` function.

  

  Thus, in order to uphold the safety contracts for the `<*mut T>::sub` method

  and `NonNull::new_unchecked` function, as well as for the correct

  logic of the work of this crate, the following rules are necessary and

  sufficient:

  

  * the `base` pointer must not be `dangling` and must points to the

    end of the first `value element` from the `data part` of the table, i.e.

    must be the pointer that returned by `RawTable::data_end` or by

    `RawTableInner::data_end<T>`;

  

  * `index` must not be greater than `RawTableInner.bucket_mask`, i.e.

    `index <= RawTableInner.bucket_mask` or, in other words, `(index + 1)`

    must be no greater than the number returned by the function

    `RawTable::buckets` or `RawTableInner::buckets`.

  

  If `mem::size_of::<T>() == 0`, then the only requirement is that the

  `index` must not be greater than `RawTableInner.bucket_mask`, i.e.

  `index <= RawTableInner.bucket_mask` or, in other words, `(index + 1)`

  must be no greater than the number returned by the function

  `RawTable::buckets` or `RawTableInner::buckets`.

  

  

  

  

  

  

- <span id="bucket-to-base-index"></span>`unsafe fn to_base_index(&self, base: NonNull<T>) -> usize`

  Calculates the index of a [`Bucket`](#bucket) as distance between two pointers

  (convenience for `base.as_ptr().offset_from(self.ptr.as_ptr()) as usize`).

  The returned value is in units of T: the distance in bytes divided by

  `core::mem::size_of::<T>()`.

  

  If the `T` is a ZST, then we return the index of the element in

  the table so that `erase` works properly (return `self.ptr.as_ptr() as usize - 1`).

  

  This function is the inverse of `from_base_index`.

  

  # Safety

  

  If `mem::size_of::<T>() != 0`, then the safety rules are directly derived

  from the safety rules for `<*const T>::offset_from` method of `*const T`.

  

  Thus, in order to uphold the safety contracts for `<*const T>::offset_from`

  method, as well as for the correct logic of the work of this crate, the

  following rules are necessary and sufficient:

  

  * `base` contained pointer must not be `dangling` and must point to the

    end of the first `element` from the `data part` of the table, i.e.

    must be a pointer that returns by `RawTable::data_end` or by

    `RawTableInner::data_end<T>`;

  

  * `self` also must not contain dangling pointer;

  

  * both `self` and `base` must be created from the same [`RawTable`](#rawtable)

    (or [`RawTableInner`](#rawtableinner)).

  

  If `mem::size_of::<T>() == 0`, this function is always safe.

  

  

  

  

  

  

- <span id="bucket-as-ptr"></span>`fn as_ptr(&self) -> *mut T`

  Acquires the underlying raw pointer `*mut T` to `data`.

  

  # Note

  

  If `T` is not [`Copy`](../../gimli/index.md), do not use `*mut T` methods that can cause calling the

  destructor of `T` (for example the `<*mut T>::drop_in_place` method), because

  for properly dropping the data we also need to clear `data` control bytes. If we

  drop data, but do not clear `data control byte` it leads to double drop when

  [`RawTable`](#rawtable) goes out of scope.

  

  If you modify an already initialized `value`, so `Hash` and `Eq` on the new

  `T` value and its borrowed form *must* match those for the old `T` value, as the map

  will not re-evaluate where the new value should go, meaning the value may become

  "lost" if their location does not reflect their state.

  

  

  

- <span id="bucket-as-non-null"></span>`fn as_non_null(&self) -> NonNull<T>`

  Acquires the underlying non-null pointer `*mut T` to `data`.

- <span id="bucket-next-n"></span>`unsafe fn next_n(&self, offset: usize) -> Self`

  Create a new [`Bucket`](#bucket) that is offset from the `self` by the given

  `offset`. The pointer calculation is performed by calculating the

  offset from `self` pointer (convenience for `self.ptr.as_ptr().sub(offset)`).

  This function is used for iterators.

  

  `offset` is in units of `T`; e.g., a `offset` of 3 represents a pointer

  offset of `3 * size_of::<T>()` bytes.

  

  # Safety

  

  If `mem::size_of::<T>() != 0`, then the safety rules are directly derived

  from the safety rules for `<*mut T>::sub` method of `*mut T` and safety

  rules of `NonNull::new_unchecked` function.

  

  Thus, in order to uphold the safety contracts for `<*mut T>::sub` method

  and `NonNull::new_unchecked` function, as well as for the correct

  logic of the work of this crate, the following rules are necessary and

  sufficient:

  

  * `self` contained pointer must not be `dangling`;

  

  * `self.to_base_index() + offset` must not be greater than `RawTableInner.bucket_mask`,

    i.e. `(self.to_base_index() + offset) <= RawTableInner.bucket_mask` or, in other

    words, `self.to_base_index() + offset + 1` must be no greater than the number returned

    by the function `RawTable::buckets` or `RawTableInner::buckets`.

  

  If `mem::size_of::<T>() == 0`, then the only requirement is that the

  `self.to_base_index() + offset` must not be greater than `RawTableInner.bucket_mask`,

  i.e. `(self.to_base_index() + offset) <= RawTableInner.bucket_mask` or, in other words,

  `self.to_base_index() + offset + 1` must be no greater than the number returned by the

  function `RawTable::buckets` or `RawTableInner::buckets`.

  

  

  

  

- <span id="bucket-drop"></span>`unsafe fn drop(&self)`

  Executes the destructor (if any) of the pointed-to `data`.

  

  # Safety

  

  See `ptr::drop_in_place` for safety concerns.

  

  You should use `RawTable::erase` instead of this function,

  or be careful with calling this function directly, because for

  properly dropping the data we need also clear `data` control bytes.

  If we drop data, but do not erase `data control byte` it leads to

  double drop when [`RawTable`](#rawtable) goes out of scope.

  

  

- <span id="bucket-read"></span>`unsafe fn read(&self) -> T`

  Reads the `value` from `self` without moving it. This leaves the

  memory in `self` unchanged.

  

  # Safety

  

  See `ptr::read` for safety concerns.

  

  You should use `RawTable::remove` instead of this function,

  or be careful with calling this function directly, because compiler

  calls its destructor when the read `value` goes out of scope. It

  can cause double dropping when [`RawTable`](#rawtable) goes out of scope,

  because of not erased `data control byte`.

  

  

- <span id="bucket-write"></span>`unsafe fn write(&self, val: T)`

  Overwrites a memory location with the given `value` without reading

  or dropping the old value (like `ptr::write` function).

  

  # Safety

  

  See `ptr::write` for safety concerns.

  

  # Note

  

  `Hash` and `Eq` on the new `T` value and its borrowed form *must* match

  those for the old `T` value, as the map will not re-evaluate where the new

  value should go, meaning the value may become "lost" if their location

  does not reflect their state.

  

  

- <span id="bucket-as-ref"></span>`unsafe fn as_ref<'a>(&self) -> &'a T`

  Returns a shared immutable reference to the `value`.

  

  # Safety

  

  See `NonNull::as_ref` for safety concerns.

- <span id="bucket-as-mut"></span>`unsafe fn as_mut<'a>(&self) -> &'a mut T`

  Returns a unique mutable reference to the `value`.

  

  # Safety

  

  See `NonNull::as_mut` for safety concerns.

  

  # Note

  

  `Hash` and `Eq` on the new `T` value and its borrowed form *must* match

  those for the old `T` value, as the map will not re-evaluate where the new

  value should go, meaning the value may become "lost" if their location

  does not reflect their state.

  

  

#### Trait Implementations

##### `impl<T> Clone for Bucket<T>`

- <span id="bucket-clone"></span>`fn clone(&self) -> Self`

##### `impl<T> Send for Bucket<T>`

### `RawTable<T, A: Allocator>`

```rust
struct RawTable<T, A: Allocator> {
    table: RawTableInner,
    alloc: A,
    marker: core::marker::PhantomData<T>,
}
```

A raw hash table with an unsafe API.

#### Implementations

- <span id="rawtable-new"></span>`const fn new() -> Self`

  Creates a new empty hash table without allocating any memory.

  

  In effect this returns a table with exactly 1 bucket. However we can

  leave the data pointer dangling since that bucket is never written to

  due to our load factor forcing us to always have at least 1 free bucket.

- <span id="rawtable-with-capacity"></span>`fn with_capacity(capacity: usize) -> Self`

  Allocates a new hash table with at least enough capacity for inserting

  the given number of elements without reallocating.

#### Trait Implementations

##### `impl<T: Clone, A: Allocator + Clone> Clone for RawTable<T, A>`

- <span id="rawtable-clone"></span>`fn clone(&self) -> Self`

- <span id="rawtable-clone-clone-from"></span>`fn clone_from(&mut self, source: &Self)`

##### `impl<T, A: Allocator + Default> Default for RawTable<T, A>`

- <span id="rawtable-default"></span>`fn default() -> Self`

##### `impl<T, A: Allocator> Drop for RawTable<T, A>`

- <span id="rawtable-drop"></span>`fn drop(&mut self)`

##### `impl<T, A: Allocator> IntoIterator for RawTable<T, A>`

- <span id="rawtable-intoiterator-type-item"></span>`type Item = T`

- <span id="rawtable-intoiterator-type-intoiter"></span>`type IntoIter = RawIntoIter<T, A>`

- <span id="rawtable-intoiterator-into-iter"></span>`fn into_iter(self) -> RawIntoIter<T, A>` — [`RawIntoIter`](#rawintoiter)

##### `impl<T: Clone, A: Allocator + Clone> RawTableClone for RawTable<T, A>`

- <span id="rawtable-rawtableclone-clone-from-spec"></span>`unsafe fn clone_from_spec(&mut self, source: &Self)`

##### `impl<T, A> Send for RawTable<T, A>`

##### `impl<T, A> Sync for RawTable<T, A>`

### `RawTableInner`

```rust
struct RawTableInner {
    bucket_mask: usize,
    ctrl: core::ptr::NonNull<u8>,
    growth_left: usize,
    items: usize,
}
```

Non-generic part of `RawTable` which allows functions to be instantiated only once regardless
of how many different key-value types are used.

#### Implementations

- <span id="rawtableinner-const-new"></span>`const NEW: Self`

- <span id="rawtableinner-new"></span>`const fn new() -> Self`

  Creates a new empty hash table without allocating any memory.

  

  In effect this returns a table with exactly 1 bucket. However we can

  leave the data pointer dangling since that bucket is never accessed

  due to our load factor forcing us to always have at least 1 free bucket.

### `RawIterRange<T>`

```rust
struct RawIterRange<T> {
    current_group: self::bitmask::BitMaskIter,
    data: Bucket<T>,
    next_ctrl: *const u8,
    end: *const u8,
}
```

Iterator over a sub-range of a table. Unlike `RawIter` this iterator does
not track an item count.

#### Implementations

- <span id="rawiterrange-new"></span>`unsafe fn new(ctrl: *const u8, data: Bucket<T>, len: usize) -> Self` — [`Bucket`](#bucket)

  Returns a `RawIterRange` covering a subset of a table.

  

  # Safety

  

  If any of the following conditions are violated, the result is

  `undefined behavior`:

  

  * `ctrl` must be [`valid`](../../thiserror_impl/valid/index.md) for reads, i.e. table outlives the `RawIterRange`;

  

  * `ctrl` must be properly aligned to the group size (`Group::WIDTH`);

  

  * `ctrl` must point to the array of properly initialized control bytes;

  

  * `data` must be the [`Bucket`](#bucket) at the `ctrl` index in the table;

  

  * the value of `len` must be less than or equal to the number of table buckets,

    and the returned value of `ctrl.as_ptr().add(len).offset_from(ctrl.as_ptr())`

    must be positive.

  

  * The `ctrl.add(len)` pointer must be either in bounds or one

    byte past the end of the same [allocated table].

  

  * The `len` must be a power of two.

  

- <span id="rawiterrange-next-impl"></span>`unsafe fn next_impl<const DO_CHECK_PTR_RANGE: bool>(&mut self) -> Option<Bucket<T>>` — [`Bucket`](#bucket)

  # Safety

  If `DO_CHECK_PTR_RANGE` is false, caller must ensure that we never try to iterate

  after yielding all elements.

- <span id="rawiterrange-fold-impl"></span>`unsafe fn fold_impl<F, B>(self, n: usize, acc: B, f: F) -> B`

  Folds every element into an accumulator by applying an operation,

  returning the final result.

  

  `fold_impl()` takes three arguments: the number of items remaining in

  the iterator, an initial value, and a closure with two arguments: an

  'accumulator', and an element. The closure returns the value that the

  accumulator should have for the next iteration.

  

  The initial value is the value the accumulator will have on the first call.

  

  After applying this closure to every element of the iterator, `fold_impl()`

  returns the accumulator.

  

  # Safety

  

  If any of the following conditions are violated, the result is

  `Undefined Behavior`:

  

  * The [`RawTableInner`](#rawtableinner) / [`RawTable`](#rawtable) must be alive and not moved,

    i.e. table outlives the `RawIterRange`;

  

  * The provided `n` value must match the actual number of items

    in the table.

#### Trait Implementations

##### `impl<T> Clone for RawIterRange<T>`

- <span id="rawiterrange-clone"></span>`fn clone(&self) -> Self`

##### `impl<T> FusedIterator for RawIterRange<T>`

##### `impl IntoIterator for RawIterRange<T>`

- <span id="rawiterrange-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="rawiterrange-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="rawiterrange-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T> Iterator for RawIterRange<T>`

- <span id="rawiterrange-iterator-type-item"></span>`type Item = Bucket<T>`

- <span id="rawiterrange-iterator-next"></span>`fn next(&mut self) -> Option<Bucket<T>>` — [`Bucket`](#bucket)

- <span id="rawiterrange-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<T> Send for RawIterRange<T>`

##### `impl<T> Sync for RawIterRange<T>`

### `RawIter<T>`

```rust
struct RawIter<T> {
    iter: RawIterRange<T>,
    items: usize,
}
```

Iterator which returns a raw pointer to every full bucket in the table.

For maximum flexibility this iterator is not bound by a lifetime, but you
must observe several rules when using it:
- You must not free the hash table while iterating (including via growing/shrinking).
- It is fine to erase a bucket that has been yielded by the iterator.
- Erasing a bucket that has not yet been yielded by the iterator may still
  result in the iterator yielding that bucket (unless `reflect_remove` is called).
- It is unspecified whether an element inserted after the iterator was
  created will be yielded by that iterator (unless `reflect_insert` is called).
- The order in which the iterator yields bucket is unspecified and may
  change in the future.

#### Implementations

- <span id="rawiter-drop-elements"></span>`unsafe fn drop_elements(&mut self)`

#### Trait Implementations

##### `impl<T> Clone for RawIter<T>`

- <span id="rawiter-clone"></span>`fn clone(&self) -> Self`

##### `impl<T> Default for RawIter<T>`

- <span id="rawiter-default"></span>`fn default() -> Self`

##### `impl<T> ExactSizeIterator for RawIter<T>`

##### `impl<T> FusedIterator for RawIter<T>`

##### `impl IntoIterator for RawIter<T>`

- <span id="rawiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="rawiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="rawiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T> Iterator for RawIter<T>`

- <span id="rawiter-iterator-type-item"></span>`type Item = Bucket<T>`

- <span id="rawiter-iterator-next"></span>`fn next(&mut self) -> Option<Bucket<T>>` — [`Bucket`](#bucket)

- <span id="rawiter-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

- <span id="rawiter-iterator-fold"></span>`fn fold<B, F>(self, init: B, f: F) -> B`

### `FullBucketsIndices`

```rust
struct FullBucketsIndices {
    current_group: self::bitmask::BitMaskIter,
    group_first_index: usize,
    ctrl: core::ptr::NonNull<u8>,
    items: usize,
}
```

Iterator which returns an index of every full bucket in the table.

For maximum flexibility this iterator is not bound by a lifetime, but you
must observe several rules when using it:
- You must not free the hash table while iterating (including via growing/shrinking).
- It is fine to erase a bucket that has been yielded by the iterator.
- Erasing a bucket that has not yet been yielded by the iterator may still
  result in the iterator yielding index of that bucket.
- It is unspecified whether an element inserted after the iterator was
  created will be yielded by that iterator.
- The order in which the iterator yields indices of the buckets is unspecified
  and may change in the future.

#### Implementations

- <span id="fullbucketsindices-next-impl"></span>`unsafe fn next_impl(&mut self) -> Option<usize>`

  Advances the iterator and returns the next value.

  

  # Safety

  

  If any of the following conditions are violated, the result is

  `Undefined Behavior`:

  

  * The [`RawTableInner`](#rawtableinner) / [`RawTable`](#rawtable) must be alive and not moved,

    i.e. table outlives the `FullBucketsIndices`;

  

  * It never tries to iterate after getting all elements.

#### Trait Implementations

##### `impl Clone for FullBucketsIndices`

- <span id="fullbucketsindices-clone"></span>`fn clone(&self) -> FullBucketsIndices` — [`FullBucketsIndices`](#fullbucketsindices)

##### `impl Default for FullBucketsIndices`

- <span id="fullbucketsindices-default"></span>`fn default() -> Self`

##### `impl ExactSizeIterator for FullBucketsIndices`

##### `impl FusedIterator for FullBucketsIndices`

##### `impl IntoIterator for FullBucketsIndices`

- <span id="fullbucketsindices-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="fullbucketsindices-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="fullbucketsindices-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for FullBucketsIndices`

- <span id="fullbucketsindices-iterator-type-item"></span>`type Item = usize`

- <span id="fullbucketsindices-iterator-next"></span>`fn next(&mut self) -> Option<usize>`

  Advances the iterator and returns the next value. It is up to

  the caller to ensure that the `RawTable` outlives the `FullBucketsIndices`,

  because we cannot make the `next` method unsafe.

- <span id="fullbucketsindices-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `RawIntoIter<T, A: Allocator>`

```rust
struct RawIntoIter<T, A: Allocator> {
    iter: RawIter<T>,
    allocation: Option<(core::ptr::NonNull<u8>, crate::alloc::alloc::Layout, A)>,
    marker: core::marker::PhantomData<T>,
}
```

Iterator which consumes a table and returns elements.

#### Implementations

- <span id="rawintoiter-iter"></span>`fn iter(&self) -> RawIter<T>` — [`RawIter`](#rawiter)

#### Trait Implementations

##### `impl<T, A: Allocator> Default for RawIntoIter<T, A>`

- <span id="rawintoiter-default"></span>`fn default() -> Self`

##### `impl<T, A: Allocator> Drop for RawIntoIter<T, A>`

- <span id="rawintoiter-drop"></span>`fn drop(&mut self)`

##### `impl<T, A: Allocator> ExactSizeIterator for RawIntoIter<T, A>`

##### `impl<T, A: Allocator> FusedIterator for RawIntoIter<T, A>`

##### `impl IntoIterator for RawIntoIter<T, A>`

- <span id="rawintoiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="rawintoiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="rawintoiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T, A: Allocator> Iterator for RawIntoIter<T, A>`

- <span id="rawintoiter-iterator-type-item"></span>`type Item = T`

- <span id="rawintoiter-iterator-next"></span>`fn next(&mut self) -> Option<T>`

- <span id="rawintoiter-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<T, A> Send for RawIntoIter<T, A>`

##### `impl<T, A> Sync for RawIntoIter<T, A>`

### `RawDrain<'a, T, A: Allocator>`

```rust
struct RawDrain<'a, T, A: Allocator> {
    iter: RawIter<T>,
    table: RawTableInner,
    orig_table: core::ptr::NonNull<RawTableInner>,
    marker: core::marker::PhantomData<&'a RawTable<T, A>>,
}
```

Iterator which consumes elements without freeing the table storage.

#### Implementations

- <span id="rawdrain-iter"></span>`fn iter(&self) -> RawIter<T>` — [`RawIter`](#rawiter)

#### Trait Implementations

##### `impl<T, A: Allocator> Drop for RawDrain<'_, T, A>`

- <span id="rawdrain-drop"></span>`fn drop(&mut self)`

##### `impl<T, A: Allocator> ExactSizeIterator for RawDrain<'_, T, A>`

##### `impl<T, A: Allocator> FusedIterator for RawDrain<'_, T, A>`

##### `impl IntoIterator for RawDrain<'a, T, A>`

- <span id="rawdrain-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="rawdrain-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="rawdrain-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T, A: Allocator> Iterator for RawDrain<'_, T, A>`

- <span id="rawdrain-iterator-type-item"></span>`type Item = T`

- <span id="rawdrain-iterator-next"></span>`fn next(&mut self) -> Option<T>`

- <span id="rawdrain-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<T, A> Send for RawDrain<'_, T, A>`

##### `impl<T, A> Sync for RawDrain<'_, T, A>`

### `RawIterHash<T>`

```rust
struct RawIterHash<T> {
    inner: RawIterHashIndices,
    _marker: core::marker::PhantomData<T>,
}
```

Iterator over occupied buckets that could match a given hash.

`RawTable` only stores 7 bits of the hash value, so this iterator may return
items that have a hash value different than the one provided. You should
always validate the returned values before using them.

For maximum flexibility this iterator is not bound by a lifetime, but you
must observe several rules when using it:
- You must not free the hash table while iterating (including via growing/shrinking).
- It is fine to erase a bucket that has been yielded by the iterator.
- Erasing a bucket that has not yet been yielded by the iterator may still
  result in the iterator yielding that bucket.
- It is unspecified whether an element inserted after the iterator was
  created will be yielded by that iterator.
- The order in which the iterator yields buckets is unspecified and may
  change in the future.

#### Implementations

- <span id="rawiterhash-new"></span>`unsafe fn new<A: Allocator>(table: &RawTable<T, A>, hash: u64) -> Self` — [`RawTable`](#rawtable)

#### Trait Implementations

##### `impl<T> Clone for RawIterHash<T>`

- <span id="rawiterhash-clone"></span>`fn clone(&self) -> Self`

##### `impl<T> Default for RawIterHash<T>`

- <span id="rawiterhash-default"></span>`fn default() -> Self`

##### `impl IntoIterator for RawIterHash<T>`

- <span id="rawiterhash-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="rawiterhash-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="rawiterhash-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T> Iterator for RawIterHash<T>`

- <span id="rawiterhash-iterator-type-item"></span>`type Item = Bucket<T>`

- <span id="rawiterhash-iterator-next"></span>`fn next(&mut self) -> Option<Bucket<T>>` — [`Bucket`](#bucket)

### `RawIterHashIndices`

```rust
struct RawIterHashIndices {
    bucket_mask: usize,
    ctrl: core::ptr::NonNull<u8>,
    tag_hash: self::tag::Tag,
    probe_seq: ProbeSeq,
    group: self::imp::Group,
    bitmask: self::bitmask::BitMaskIter,
}
```

#### Implementations

- <span id="rawiterhashindices-new"></span>`unsafe fn new(table: &RawTableInner, hash: u64) -> Self` — [`RawTableInner`](#rawtableinner)

#### Trait Implementations

##### `impl Clone for RawIterHashIndices`

- <span id="rawiterhashindices-clone"></span>`fn clone(&self) -> RawIterHashIndices` — [`RawIterHashIndices`](#rawiterhashindices)

##### `impl Default for RawIterHashIndices`

- <span id="rawiterhashindices-default"></span>`fn default() -> Self`

##### `impl IntoIterator for RawIterHashIndices`

- <span id="rawiterhashindices-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="rawiterhashindices-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="rawiterhashindices-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for RawIterHashIndices`

- <span id="rawiterhashindices-iterator-type-item"></span>`type Item = usize`

- <span id="rawiterhashindices-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `RawExtractIf<'a, T, A: Allocator>`

```rust
struct RawExtractIf<'a, T, A: Allocator> {
    pub iter: RawIter<T>,
    pub table: &'a mut RawTable<T, A>,
}
```

#### Implementations

- <span id="rawextractif-next"></span>`fn next<F>(&mut self, f: F) -> Option<T>`

## Enums

### `Fallibility`

```rust
enum Fallibility {
    Fallible,
    Infallible,
}
```

Whether memory allocation errors should return an error or abort.

#### Implementations

- <span id="fallibility-capacity-overflow"></span>`fn capacity_overflow(self) -> TryReserveError` — [`TryReserveError`](../index.md#tryreserveerror)

  Error to return on capacity overflow.

- <span id="fallibility-alloc-err"></span>`fn alloc_err(self, layout: Layout) -> TryReserveError` — [`TryReserveError`](../index.md#tryreserveerror)

  Error to return on allocation error.

#### Trait Implementations

##### `impl Clone for Fallibility`

- <span id="fallibility-clone"></span>`fn clone(&self) -> Fallibility` — [`Fallibility`](#fallibility)

##### `impl Copy for Fallibility`

## Traits

### `SizedTypeProperties`

```rust
trait SizedTypeProperties: Sized { ... }
```

#### Associated Constants

- `const IS_ZERO_SIZED: bool`

- `const NEEDS_DROP: bool`

#### Implementors

- `T`

### `RawTableClone`

```rust
trait RawTableClone { ... }
```

Specialization of `clone_from` for `Copy` types

#### Required Methods

- `fn clone_from_spec(&mut self, source: &Self)`

#### Implementors

- [`RawTable`](#rawtable)

## Functions

### `offset_from`

```rust
unsafe fn offset_from<T>(to: *const T, from: *const T) -> usize
```

### `h1`

```rust
fn h1(hash: u64) -> usize
```

Primary hash function, used to select the initial bucket to probe from.

### `capacity_to_buckets`

```rust
fn capacity_to_buckets(cap: usize, table_layout: TableLayout) -> Option<usize>
```

Returns the number of buckets needed to hold the given number of items,
taking the maximum load factor into account.

Returns `None` if an overflow occurs.

This ensures that `buckets * table_layout.size >= table_layout.ctrl_align`.

### `ensure_bucket_bytes_at_least_ctrl_align`

```rust
fn ensure_bucket_bytes_at_least_ctrl_align(table_layout: TableLayout, buckets: usize)
```

### `bucket_mask_to_capacity`

```rust
fn bucket_mask_to_capacity(bucket_mask: usize) -> usize
```

Returns the maximum effective capacity for the given bucket mask, taking
the maximum load factor into account.

### `prev_pow2`

```rust
fn prev_pow2(z: usize) -> usize
```

Find the previous power of 2. If it's already a power of 2, it's unchanged.
Passing zero is undefined behavior.

### `maximum_buckets_in`

```rust
fn maximum_buckets_in(allocation_size: usize, table_layout: TableLayout, group_width: usize) -> usize
```

Finds the largest number of buckets that can fit in `allocation_size`
provided the given TableLayout.

This relies on some invariants of `capacity_to_buckets`, so only feed in
an `allocation_size` calculated from `capacity_to_buckets`.

