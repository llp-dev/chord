# Crate `smallvec`

Small vectors in various sizes. These store a certain number of elements inline, and fall back
to the heap for larger allocations.  This can be a useful optimization for improving cache
locality and reducing allocator traffic for workloads that fit within the inline buffer.

## `no_std` support

By default, `smallvec` does not depend on `std`.  However, the optional
`write` feature implements the `std::io::Write` trait for vectors of `u8`.
When this feature is enabled, `smallvec` depends on `std`.

## Optional features

### `serde`

When this optional dependency is enabled, `SmallVec` implements the `serde::Serialize` and
`serde::Deserialize` traits.

### `write`

When this feature is enabled, `SmallVec<[u8; _]>` implements the `std::io::Write` trait.
This feature is not compatible with `#![no_std]` programs.

### `union`

**This feature requires Rust 1.49.**

When the `union` feature is enabled `smallvec` will track its state (inline or spilled)
without the use of an enum tag, reducing the size of the `smallvec` by one machine word.
This means that there is potentially no space overhead compared to `Vec`.
Note that `smallvec` can still be larger than `Vec` if the inline buffer is larger than two
machine words.

To use this feature add `features = ["union"]` in the `smallvec` section of Cargo.toml.
Note that this feature requires Rust 1.49.

Tracking issue: [rust-lang/rust#55149](https://github.com/rust-lang/rust/issues/55149)

### `const_generics`

**This feature requires Rust 1.51.**

When this feature is enabled, `SmallVec` works with any arrays of any size, not just a fixed
list of sizes.

### `const_new`

**This feature requires Rust 1.51.**

This feature exposes the functions `SmallVec::new_const`, `SmallVec::from_const`, and `smallvec_inline` which enables the `SmallVec` to be initialized from a const context.
For details, see the
[Rust Reference](https://doc.rust-lang.org/reference/const_eval.html#const-functions).

### `drain_filter`

**This feature is unstable.** It may change to match the unstable `drain_filter` method in libstd.

Enables the `drain_filter` method, which produces an iterator that calls a user-provided
closure to determine which elements of the vector to remove and yield from the iterator.

### `drain_keep_rest`

**This feature is unstable.** It may change to match the unstable `drain_keep_rest` method in libstd.

Enables the `DrainFilter::keep_rest` method.

### `specialization`

**This feature is unstable and requires a nightly build of the Rust toolchain.**

When this feature is enabled, `SmallVec::from(slice)` has improved performance for slices
of `Copy` types.  (Without this feature, you can use `SmallVec::from_slice` to get optimal
performance for `Copy` types.)

Tracking issue: [rust-lang/rust#31844](https://github.com/rust-lang/rust/issues/31844)

### `may_dangle`

**This feature is unstable and requires a nightly build of the Rust toolchain.**

This feature makes the Rust compiler less strict about use of vectors that contain borrowed
references. For details, see the
[Rustonomicon](https://doc.rust-lang.org/1.42.0/nomicon/dropck.html#an-escape-hatch).

Tracking issue: [rust-lang/rust#34761](https://github.com/rust-lang/rust/issues/34761)

## Contents

- [Structs](#structs)
  - [`Drain`](#drain)
  - [`SmallVec`](#smallvec)
  - [`IntoIter`](#intoiter)
  - [`SetLenOnDrop`](#setlenondrop)
  - [`ConstNonNull`](#constnonnull)
- [Enums](#enums)
  - [`CollectionAllocErr`](#collectionallocerr)
  - [`SmallVecData`](#smallvecdata)
- [Traits](#traits)
  - [`Array`](#array)
  - [`ToSmallVec`](#tosmallvec)
- [Functions](#functions)
  - [`infallible`](#infallible)
  - [`layout_array`](#layout-array)
  - [`deallocate`](#deallocate)
- [Macros](#macros)
  - [`debug_unreachable!`](#debug-unreachable)
  - [`impl_array!`](#impl-array)
  - [`smallvec!`](#smallvec)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Drain`](#drain) | struct | An iterator that removes the items from a `SmallVec` and yields them by value. |
| [`SmallVec`](#smallvec) | struct | A `Vec`-like container that can store a small number of elements inline. |
| [`IntoIter`](#intoiter) | struct | An iterator that consumes a `SmallVec` and yields its items by value. |
| [`SetLenOnDrop`](#setlenondrop) | struct | Set the length of the vec when the `SetLenOnDrop` value goes out of scope. |
| [`ConstNonNull`](#constnonnull) | struct |  |
| [`CollectionAllocErr`](#collectionallocerr) | enum | Error type for APIs with fallible heap allocation |
| [`SmallVecData`](#smallvecdata) | enum |  |
| [`Array`](#array) | trait | Types that can be used as the backing store for a [`SmallVec`]. |
| [`ToSmallVec`](#tosmallvec) | trait | Convenience trait for constructing a `SmallVec` |
| [`infallible`](#infallible) | fn |  |
| [`layout_array`](#layout-array) | fn | FIXME: use `Layout::array` when we require a Rust version where it’s stable <https://github.com/rust-lang/rust/issues/55724> |
| [`deallocate`](#deallocate) | fn |  |
| [`debug_unreachable!`](#debug-unreachable) | macro | `panic!()` in debug builds, optimization hint in release. |
| [`impl_array!`](#impl-array) | macro |  |
| [`smallvec!`](#smallvec) | macro | Creates a [`SmallVec`] containing the arguments. |

## Structs

### `Drain<'a, T: 'a + Array>`

```rust
struct Drain<'a, T: 'a + Array> {
    tail_start: usize,
    tail_len: usize,
    iter: slice::Iter<'a, <T as >::Item>,
    vec: core::ptr::NonNull<SmallVec<T>>,
}
```

An iterator that removes the items from a `SmallVec` and yields them by value.

Returned from [`SmallVec::drain`][1].


#### Trait Implementations

##### `impl<T: 'a + Array> Debug for Drain<'a, T>`

- <span id="drain-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: 'a + Array> DoubleEndedIterator for Drain<'a, T>`

- <span id="drain-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<T as >::Item>` — [`Array`](#array)

##### `impl<T: 'a + Array> Drop for Drain<'a, T>`

- <span id="drain-drop"></span>`fn drop(&mut self)`

##### `impl<T: Array> ExactSizeIterator for Drain<'a, T>`

- <span id="drain-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl<T: Array> FusedIterator for Drain<'a, T>`

##### `impl IntoIterator for Drain<'a, T>`

- <span id="drain-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="drain-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="drain-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T: 'a + Array> Iterator for Drain<'a, T>`

- <span id="drain-iterator-type-item"></span>`type Item = <T as Array>::Item`

- <span id="drain-iterator-next"></span>`fn next(&mut self) -> Option<<T as >::Item>` — [`Array`](#array)

- <span id="drain-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

##### `impl<T: Send + Array> Send for Drain<'a, T>`

##### `impl<T: Sync + Array> Sync for Drain<'a, T>`

### `SmallVec<A: Array>`

```rust
struct SmallVec<A: Array> {
    capacity: usize,
    data: SmallVecData<A>,
}
```

A `Vec`-like container that can store a small number of elements inline.

`SmallVec` acts like a vector, but can store a limited amount of data inline within the
`SmallVec` struct rather than in a separate allocation.  If the data exceeds this limit, the
`SmallVec` will "spill" its data onto the heap, allocating a new buffer to hold it.

The amount of data that a `SmallVec` can store inline depends on its backing store. The backing
store can be any type that implements the `Array` trait; usually it is a small fixed-sized
array.  For example a `SmallVec<[u64; 8]>` can hold up to eight 64-bit integers inline.

## Example

```rust
use smallvec::SmallVec;
let mut v = SmallVec::<[u8; 4]>::new(); // initialize an empty vector

// The vector can hold up to 4 items without spilling onto the heap.
v.extend(0..4);
assert_eq!(v.len(), 4);
assert!(!v.spilled());

// Pushing another element will force the buffer to spill:
v.push(4);
assert_eq!(v.len(), 5);
assert!(v.spilled());
```

#### Implementations

- <span id="smallvec-new"></span>`fn new() -> SmallVec<A>` — [`SmallVec`](#smallvec)

  Construct an empty vector

- <span id="smallvec-with-capacity"></span>`fn with_capacity(n: usize) -> Self`

  Construct an empty vector with enough capacity pre-allocated to store at least `n`

  elements.

  

  Will create a heap allocation only if `n` is larger than the inline capacity.

  

  ```rust

  use smallvec::SmallVec;

  

  let v: SmallVec<[u8; 3]> = SmallVec::with_capacity(100);

  

  assert!(v.is_empty());

  assert!(v.capacity() >= 100);

  ```

- <span id="smallvec-from-vec"></span>`fn from_vec(vec: Vec<<A as >::Item>) -> SmallVec<A>` — [`Array`](#array), [`SmallVec`](#smallvec)

  Construct a new `SmallVec` from a `Vec<A::Item>`.

  

  Elements will be copied to the inline buffer if `vec.capacity() <= Self::inline_capacity()`.

  

  ```rust

  use smallvec::SmallVec;

  

  let vec = vec![1, 2, 3, 4, 5];

  let small_vec: SmallVec<[_; 3]> = SmallVec::from_vec(vec);

  

  assert_eq!(&*small_vec, &[1, 2, 3, 4, 5]);

  ```

- <span id="smallvec-from-buf"></span>`fn from_buf(buf: A) -> SmallVec<A>` — [`SmallVec`](#smallvec)

  Constructs a new `SmallVec` on the stack from an `A` without

  copying elements.

  

  ```rust

  use smallvec::SmallVec;

  

  let buf = [1, 2, 3, 4, 5];

  let small_vec: SmallVec<_> = SmallVec::from_buf(buf);

  

  assert_eq!(&*small_vec, &[1, 2, 3, 4, 5]);

  ```

- <span id="smallvec-from-buf-and-len"></span>`fn from_buf_and_len(buf: A, len: usize) -> SmallVec<A>` — [`SmallVec`](#smallvec)

  Constructs a new `SmallVec` on the stack from an `A` without

  copying elements. Also sets the length, which must be less or

  equal to the size of `buf`.

  

  ```rust

  use smallvec::SmallVec;

  

  let buf = [1, 2, 3, 4, 5, 0, 0, 0];

  let small_vec: SmallVec<_> = SmallVec::from_buf_and_len(buf, 5);

  

  assert_eq!(&*small_vec, &[1, 2, 3, 4, 5]);

  ```

- <span id="smallvec-from-buf-and-len-unchecked"></span>`unsafe fn from_buf_and_len_unchecked(buf: MaybeUninit<A>, len: usize) -> SmallVec<A>` — [`SmallVec`](#smallvec)

  Constructs a new `SmallVec` on the stack from an `A` without

  copying elements. Also sets the length. The user is responsible

  for ensuring that `len <= A::size()`.

  

  ```rust

  use smallvec::SmallVec;

  use std::mem::MaybeUninit;

  

  let buf = [1, 2, 3, 4, 5, 0, 0, 0];

  let small_vec: SmallVec<_> = unsafe {

      SmallVec::from_buf_and_len_unchecked(MaybeUninit::new(buf), 5)

  };

  

  assert_eq!(&*small_vec, &[1, 2, 3, 4, 5]);

  ```

- <span id="smallvec-set-len"></span>`unsafe fn set_len(&mut self, new_len: usize)`

  Sets the length of a vector.

  

  This will explicitly set the size of the vector, without actually

  modifying its buffers, so it is up to the caller to ensure that the

  vector is actually the specified size.

- <span id="smallvec-inline-capacity"></span>`fn inline_capacity() -> usize`

  The maximum number of elements this vector can hold inline

- <span id="smallvec-inline-size"></span>`fn inline_size(&self) -> usize`

  The maximum number of elements this vector can hold inline

- <span id="smallvec-len"></span>`fn len(&self) -> usize`

  The number of elements stored in the vector

- <span id="smallvec-is-empty"></span>`fn is_empty(&self) -> bool`

  Returns `true` if the vector is empty

- <span id="smallvec-capacity"></span>`fn capacity(&self) -> usize`

  The number of items the vector can hold without reallocating

- <span id="smallvec-triple"></span>`fn triple(&self) -> (ConstNonNull<<A as >::Item>, usize, usize)` — [`ConstNonNull`](#constnonnull), [`Array`](#array)

  Returns a tuple with (data ptr, len, capacity)

  Useful to get all `SmallVec` properties with a single check of the current storage variant.

- <span id="smallvec-triple-mut"></span>`fn triple_mut(&mut self) -> (NonNull<<A as >::Item>, &mut usize, usize)` — [`Array`](#array)

  Returns a tuple with (data ptr, len ptr, capacity)

- <span id="smallvec-spilled"></span>`fn spilled(&self) -> bool`

  Returns `true` if the data has spilled into a separate heap-allocated buffer.

- <span id="smallvec-drain"></span>`fn drain<R>(&mut self, range: R) -> Drain<'_, A>` — [`Drain`](#drain)

  Creates a draining iterator that removes the specified range in the vector

  and yields the removed items.

  

  Note 1: The element range is removed even if the iterator is only

  partially consumed or not consumed at all.

  

  Note 2: It is unspecified how many elements are removed from the vector

  if the `Drain` value is leaked.

  

  # Panics

  

  Panics if the starting point is greater than the end point or if

  the end point is greater than the length of the vector.

- <span id="smallvec-push"></span>`fn push(&mut self, value: <A as >::Item)` — [`Array`](#array)

  Append an item to the vector.

- <span id="smallvec-pop"></span>`fn pop(&mut self) -> Option<<A as >::Item>` — [`Array`](#array)

  Remove an item from the end of the vector and return it, or None if empty.

- <span id="smallvec-append"></span>`fn append<B>(&mut self, other: &mut SmallVec<B>)` — [`SmallVec`](#smallvec)

  Moves all the elements of `other` into `self`, leaving `other` empty.

  

  # Example

  

  ```rust

  use smallvec::{SmallVec, smallvec};

  let mut v0: SmallVec<[u8; 16]> = smallvec![1, 2, 3];

  let mut v1: SmallVec<[u8; 32]> = smallvec![4, 5, 6];

  v0.append(&mut v1);

  assert_eq!(*v0, [1, 2, 3, 4, 5, 6]);

  assert_eq!(*v1, []);

  ```

- <span id="smallvec-grow"></span>`fn grow(&mut self, new_cap: usize)`

  Re-allocate to set the capacity to `max(new_cap, inline_size())`.

  

  Panics if `new_cap` is less than the vector's length

  or if the capacity computation overflows `usize`.

- <span id="smallvec-try-grow"></span>`fn try_grow(&mut self, new_cap: usize) -> Result<(), CollectionAllocErr>` — [`CollectionAllocErr`](#collectionallocerr)

  Re-allocate to set the capacity to `max(new_cap, inline_size())`.

  

  Panics if `new_cap` is less than the vector's length

- <span id="smallvec-reserve"></span>`fn reserve(&mut self, additional: usize)`

  Reserve capacity for `additional` more elements to be inserted.

  

  May reserve more space to avoid frequent reallocations.

  

  Panics if the capacity computation overflows `usize`.

- <span id="smallvec-reserve-one-unchecked"></span>`fn reserve_one_unchecked(&mut self)`

  Internal method used to grow in push() and insert(), where we know already we have to grow.

- <span id="smallvec-try-reserve"></span>`fn try_reserve(&mut self, additional: usize) -> Result<(), CollectionAllocErr>` — [`CollectionAllocErr`](#collectionallocerr)

  Reserve capacity for `additional` more elements to be inserted.

  

  May reserve more space to avoid frequent reallocations.

- <span id="smallvec-reserve-exact"></span>`fn reserve_exact(&mut self, additional: usize)`

  Reserve the minimum capacity for `additional` more elements to be inserted.

  

  Panics if the new capacity overflows `usize`.

- <span id="smallvec-try-reserve-exact"></span>`fn try_reserve_exact(&mut self, additional: usize) -> Result<(), CollectionAllocErr>` — [`CollectionAllocErr`](#collectionallocerr)

  Reserve the minimum capacity for `additional` more elements to be inserted.

- <span id="smallvec-shrink-to-fit"></span>`fn shrink_to_fit(&mut self)`

  Shrink the capacity of the vector as much as possible.

  

  When possible, this will move data from an external heap buffer to the vector's inline

  storage.

- <span id="smallvec-truncate"></span>`fn truncate(&mut self, len: usize)`

  Shorten the vector, keeping the first `len` elements and dropping the rest.

  

  If `len` is greater than or equal to the vector's current length, this has no

  effect.

  

  This does not re-allocate.  If you want the vector's capacity to shrink, call

  `shrink_to_fit` after truncating.

- <span id="smallvec-as-slice"></span>`fn as_slice(&self) -> &[<A as >::Item]` — [`Array`](#array)

  Extracts a slice containing the entire vector.

  

  Equivalent to `&s[..]`.

- <span id="smallvec-as-mut-slice"></span>`fn as_mut_slice(&mut self) -> &mut [<A as >::Item]` — [`Array`](#array)

  Extracts a mutable slice of the entire vector.

  

  Equivalent to `&mut s[..]`.

- <span id="smallvec-swap-remove"></span>`fn swap_remove(&mut self, index: usize) -> <A as >::Item` — [`Array`](#array)

  Remove the element at position `index`, replacing it with the last element.

  

  This does not preserve ordering, but is O(1).

  

  Panics if `index` is out of bounds.

- <span id="smallvec-clear"></span>`fn clear(&mut self)`

  Remove all elements from the vector.

- <span id="smallvec-remove"></span>`fn remove(&mut self, index: usize) -> <A as >::Item` — [`Array`](#array)

  Remove and return the element at position `index`, shifting all elements after it to the

  left.

  

  Panics if `index` is out of bounds.

- <span id="smallvec-insert"></span>`fn insert(&mut self, index: usize, element: <A as >::Item)` — [`Array`](#array)

  Insert an element at position `index`, shifting all elements after it to the right.

  

  Panics if `index > len`.

- <span id="smallvec-insert-many"></span>`fn insert_many<I: IntoIterator<Item = <A as >::Item>>(&mut self, index: usize, iterable: I)`

  Insert multiple elements at position `index`, shifting all following elements toward the

  back.

- <span id="smallvec-into-vec"></span>`fn into_vec(self) -> Vec<<A as >::Item>` — [`Array`](#array)

  Convert a `SmallVec` to a `Vec`, without reallocating if the `SmallVec` has already spilled onto

  the heap.

- <span id="smallvec-into-boxed-slice"></span>`fn into_boxed_slice(self) -> Box<[<A as >::Item]>` — [`Array`](#array)

  Converts a `SmallVec` into a `Box<[T]>` without reallocating if the `SmallVec` has already spilled

  onto the heap.

  

  Note that this will drop any excess capacity.

- <span id="smallvec-into-inner"></span>`fn into_inner(self) -> Result<A, Self>`

  Convert the `SmallVec` into an `A` if possible. Otherwise return `Err(Self)`.

  

  This method returns `Err(Self)` if the `SmallVec` is too short (and the `A` contains uninitialized elements),

  or if the `SmallVec` is too long (and all the elements were spilled to the heap).

- <span id="smallvec-retain"></span>`fn retain<F: FnMut(&mut <A as >::Item) -> bool>(&mut self, f: F)`

  Retains only the elements specified by the predicate.

  

  In other words, remove all elements `e` such that `f(&e)` returns `false`.

  This method operates in place and preserves the order of the retained

  elements.

- <span id="smallvec-retain-mut"></span>`fn retain_mut<F: FnMut(&mut <A as >::Item) -> bool>(&mut self, f: F)`

  Retains only the elements specified by the predicate.

  

  This method is identical in behaviour to `retain`; it is included only

  to maintain api-compatibility with `std::Vec`, where the methods are

  separate for historical reasons.

- <span id="smallvec-dedup"></span>`fn dedup(&mut self)`

  Removes consecutive duplicate elements.

- <span id="smallvec-dedup-by"></span>`fn dedup_by<F>(&mut self, same_bucket: F)`

  Removes consecutive duplicate elements using the given equality relation.

- <span id="smallvec-dedup-by-key"></span>`fn dedup_by_key<F, K>(&mut self, key: F)`

  Removes consecutive elements that map to the same key.

- <span id="smallvec-resize-with"></span>`fn resize_with<F>(&mut self, new_len: usize, f: F)`

  Resizes the `SmallVec` in-place so that `len` is equal to `new_len`.

  

  If `new_len` is greater than `len`, the `SmallVec` is extended by the difference, with each

  additional slot filled with the result of calling the closure `f`. The return values from `f`

  will end up in the `SmallVec` in the order they have been generated.

  

  If `new_len` is less than `len`, the `SmallVec` is simply truncated.

  

  This method uses a closure to create new values on every push. If you'd rather `Clone` a given

  value, use `resize`. If you want to use the `Default` trait to generate values, you can pass

  `Default::default()` as the second argument.

  

  Added for `std::vec::Vec` compatibility (added in Rust 1.33.0)

  

  ```rust

  use smallvec::{smallvec, SmallVec};

  let mut vec : SmallVec<[_; 4]> = smallvec![1, 2, 3];

  vec.resize_with(5, Default::default);

  assert_eq!(&*vec, &[1, 2, 3, 0, 0]);

  

  let mut vec : SmallVec<[_; 4]> = smallvec![];

  let mut p = 1;

  vec.resize_with(4, || { p *= 2; p });

  assert_eq!(&*vec, &[2, 4, 8, 16]);

  ```

- <span id="smallvec-from-raw-parts"></span>`unsafe fn from_raw_parts(ptr: *mut <A as >::Item, length: usize, capacity: usize) -> SmallVec<A>` — [`Array`](#array), [`SmallVec`](#smallvec)

  Creates a `SmallVec` directly from the raw components of another

  `SmallVec`.

  

  # Safety

  

  This is highly unsafe, due to the number of invariants that aren't

  checked:

  

  * `ptr` needs to have been previously allocated via `SmallVec` for its

    spilled storage (at least, it's highly likely to be incorrect if it

    wasn't).

  * `ptr`'s `A::Item` type needs to be the same size and alignment that

    it was allocated with

  * `length` needs to be less than or equal to `capacity`.

  * `capacity` needs to be the capacity that the pointer was allocated

    with.

  

  Violating these may cause problems like corrupting the allocator's

  internal data structures.

  

  Additionally, `capacity` must be greater than the amount of inline

  storage `A` has; that is, the new `SmallVec` must need to spill over

  into heap allocated storage. This condition is asserted against.

  

  The ownership of `ptr` is effectively transferred to the

  `SmallVec` which may then deallocate, reallocate or change the

  contents of memory pointed to by the pointer at will. Ensure

  that nothing else uses the pointer after calling this

  function.

  

  # Examples

  

  ```rust

  use smallvec::{smallvec, SmallVec};

  use std::mem;

  use std::ptr;

  

  fn main() {

      let mut v: SmallVec<[_; 1]> = smallvec![1, 2, 3];

  

      // Pull out the important parts of `v`.

      let p = v.as_mut_ptr();

      let len = v.len();

      let cap = v.capacity();

      let spilled = v.spilled();

  

      unsafe {

          // Forget all about `v`. The heap allocation that stored the

          // three values won't be deallocated.

          mem::forget(v);

  

          // Overwrite memory with [4, 5, 6].

          //

          // This is only safe if `spilled` is true! Otherwise, we are

          // writing into the old `SmallVec`'s inline storage on the

          // stack.

          assert!(spilled);

          for i in 0..len {

              ptr::write(p.add(i), 4 + i);

          }

  

          // Put everything back together into a SmallVec with a different

          // amount of inline storage, but which is still less than `cap`.

          let rebuilt = SmallVec::<[_; 2]>::from_raw_parts(p, len, cap);

          assert_eq!(&*rebuilt, &[4, 5, 6]);

      }

  }

- <span id="smallvec-as-ptr"></span>`fn as_ptr(&self) -> *const <A as >::Item` — [`Array`](#array)

  Returns a raw pointer to the vector's buffer.

- <span id="smallvec-as-mut-ptr"></span>`fn as_mut_ptr(&mut self) -> *mut <A as >::Item` — [`Array`](#array)

  Returns a raw mutable pointer to the vector's buffer.

#### Trait Implementations

##### `impl<A: Array> AsMut for SmallVec<A>`

- <span id="smallvec-asmut-as-mut"></span>`fn as_mut(&mut self) -> &mut [<A as >::Item]` — [`Array`](#array)

##### `impl<A: Array> AsRef for SmallVec<A>`

- <span id="smallvec-asref-as-ref"></span>`fn as_ref(&self) -> &[<A as >::Item]` — [`Array`](#array)

##### `impl<A: Array> Clone for SmallVec<A>`

- <span id="smallvec-clone"></span>`fn clone(&self) -> SmallVec<A>` — [`SmallVec`](#smallvec)

- <span id="smallvec-clone-clone-from"></span>`fn clone_from(&mut self, source: &Self)`

##### `impl<A: Array> Debug for SmallVec<A>`

- <span id="smallvec-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<A: Array> Default for SmallVec<A>`

- <span id="smallvec-default"></span>`fn default() -> SmallVec<A>` — [`SmallVec`](#smallvec)

##### `impl<A: Array> Deref for SmallVec<A>`

- <span id="smallvec-deref-type-target"></span>`type Target = [<A as Array>::Item]`

- <span id="smallvec-deref"></span>`fn deref(&self) -> &[<A as >::Item]` — [`Array`](#array)

##### `impl<A: Array> DerefMut for SmallVec<A>`

- <span id="smallvec-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut [<A as >::Item]` — [`Array`](#array)

##### `impl<A: Array> Drop for SmallVec<A>`

- <span id="smallvec-drop"></span>`fn drop(&mut self)`

##### `impl<A: Array> Eq for SmallVec<A>`

##### `impl<A: Array> Extend for SmallVec<A>`

- <span id="smallvec-extend"></span>`fn extend<I: IntoIterator<Item = <A as >::Item>>(&mut self, iterable: I)`

##### `impl<A: Array> FromIterator for SmallVec<A>`

- <span id="smallvec-fromiterator-from-iter"></span>`fn from_iter<I: IntoIterator<Item = <A as >::Item>>(iterable: I) -> SmallVec<A>` — [`SmallVec`](#smallvec)

##### `impl<A: Array> Hash for SmallVec<A>`

- <span id="smallvec-hash"></span>`fn hash<H: Hasher>(&self, state: &mut H)`

##### `impl<A: Array, I: SliceIndex<[<A as >::Item]>> Index for SmallVec<A>`

- <span id="smallvec-index-type-output"></span>`type Output = <I as SliceIndex>::Output`

- <span id="smallvec-index"></span>`fn index(&self, index: I) -> &<I as >::Output`

##### `impl<A: Array, I: SliceIndex<[<A as >::Item]>> IndexMut for SmallVec<A>`

- <span id="smallvec-indexmut-index-mut"></span>`fn index_mut(&mut self, index: I) -> &mut <I as >::Output`

##### `impl<A: Array> IntoIterator for SmallVec<A>`

- <span id="smallvec-intoiterator-type-intoiter"></span>`type IntoIter = IntoIter<A>`

- <span id="smallvec-intoiterator-type-item"></span>`type Item = <A as Array>::Item`

- <span id="smallvec-intoiterator-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl<A: Array> Ord for SmallVec<A>`

- <span id="smallvec-ord-cmp"></span>`fn cmp(&self, other: &SmallVec<A>) -> cmp::Ordering` — [`SmallVec`](#smallvec)

##### `impl<A: Array, B: Array> PartialEq for SmallVec<A>`

- <span id="smallvec-partialeq-eq"></span>`fn eq(&self, other: &SmallVec<B>) -> bool` — [`SmallVec`](#smallvec)

##### `impl<A: Array> PartialOrd for SmallVec<A>`

- <span id="smallvec-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &SmallVec<A>) -> Option<cmp::Ordering>` — [`SmallVec`](#smallvec)

##### `impl Receiver for SmallVec<A>`

- <span id="smallvec-receiver-type-target"></span>`type Target = T`

##### `impl<A: Array> Send for SmallVec<A>`

### `IntoIter<A: Array>`

```rust
struct IntoIter<A: Array> {
    data: SmallVec<A>,
    current: usize,
    end: usize,
}
```

An iterator that consumes a `SmallVec` and yields its items by value.

Returned from [`SmallVec::into_iter`][1].


#### Implementations

- <span id="intoiter-as-slice"></span>`fn as_slice(&self) -> &[<A as >::Item]` — [`Array`](#array)

  Returns the remaining items of this iterator as a slice.

- <span id="intoiter-as-mut-slice"></span>`fn as_mut_slice(&mut self) -> &mut [<A as >::Item]` — [`Array`](#array)

  Returns the remaining items of this iterator as a mutable slice.

#### Trait Implementations

##### `impl<A: Array + Clone> Clone for IntoIter<A>`

- <span id="intoiter-clone"></span>`fn clone(&self) -> IntoIter<A>` — [`IntoIter`](#intoiter)

##### `impl<A: Array> Debug for IntoIter<A>`

- <span id="intoiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<A: Array> DoubleEndedIterator for IntoIter<A>`

- <span id="intoiter-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<A as >::Item>` — [`Array`](#array)

##### `impl<A: Array> Drop for IntoIter<A>`

- <span id="intoiter-drop"></span>`fn drop(&mut self)`

##### `impl<A: Array> ExactSizeIterator for IntoIter<A>`

##### `impl<A: Array> FusedIterator for IntoIter<A>`

##### `impl IntoIterator for IntoIter<A>`

- <span id="intoiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="intoiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="intoiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<A: Array> Iterator for IntoIter<A>`

- <span id="intoiter-iterator-type-item"></span>`type Item = <A as Array>::Item`

- <span id="intoiter-iterator-next"></span>`fn next(&mut self) -> Option<<A as >::Item>` — [`Array`](#array)

- <span id="intoiter-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `SetLenOnDrop<'a>`

```rust
struct SetLenOnDrop<'a> {
    len: &'a mut usize,
    local_len: usize,
}
```

Set the length of the vec when the `SetLenOnDrop` value goes out of scope.

Copied from <https://github.com/rust-lang/rust/pull/36355>

#### Implementations

- <span id="setlenondrop-new"></span>`fn new(len: &'a mut usize) -> Self`

- <span id="setlenondrop-get"></span>`fn get(&self) -> usize`

- <span id="setlenondrop-increment-len"></span>`fn increment_len(&mut self, increment: usize)`

#### Trait Implementations

##### `impl Drop for SetLenOnDrop<'a>`

- <span id="setlenondrop-drop"></span>`fn drop(&mut self)`

### `ConstNonNull<T>`

```rust
struct ConstNonNull<T>(core::ptr::NonNull<T>);
```

#### Implementations

- <span id="constnonnull-new"></span>`fn new(ptr: *const T) -> Option<Self>`

- <span id="constnonnull-as-ptr"></span>`fn as_ptr(self) -> *const T`

#### Trait Implementations

##### `impl<T> Clone for ConstNonNull<T>`

- <span id="constnonnull-clone"></span>`fn clone(&self) -> Self`

##### `impl<T> Copy for ConstNonNull<T>`

## Enums

### `CollectionAllocErr`

```rust
enum CollectionAllocErr {
    CapacityOverflow,
    AllocErr {
        layout: alloc::alloc::Layout,
    },
}
```

Error type for APIs with fallible heap allocation

#### Variants

- **`CapacityOverflow`**

  Overflow `usize::MAX` or other error during size computation

- **`AllocErr`**

  The allocator return an error

#### Trait Implementations

##### `impl Debug for CollectionAllocErr`

- <span id="collectionallocerr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for CollectionAllocErr`

- <span id="collectionallocerr-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl ToString for CollectionAllocErr`

- <span id="collectionallocerr-tostring-to-string"></span>`fn to_string(&self) -> String`

### `SmallVecData<A: Array>`

```rust
enum SmallVecData<A: Array> {
    Inline(core::mem::MaybeUninit<A>),
    Heap {
        ptr: core::ptr::NonNull<<A as >::Item>,
        len: usize,
    },
}
```

#### Implementations

- <span id="smallvecdata-inline"></span>`unsafe fn inline(&self) -> ConstNonNull<<A as >::Item>` — [`ConstNonNull`](#constnonnull), [`Array`](#array)

- <span id="smallvecdata-inline-mut"></span>`unsafe fn inline_mut(&mut self) -> NonNull<<A as >::Item>` — [`Array`](#array)

- <span id="smallvecdata-from-inline"></span>`fn from_inline(inline: MaybeUninit<A>) -> SmallVecData<A>` — [`SmallVecData`](#smallvecdata)

- <span id="smallvecdata-into-inline"></span>`unsafe fn into_inline(self) -> MaybeUninit<A>`

- <span id="smallvecdata-heap"></span>`unsafe fn heap(&self) -> (ConstNonNull<<A as >::Item>, usize)` — [`ConstNonNull`](#constnonnull), [`Array`](#array)

- <span id="smallvecdata-heap-mut"></span>`unsafe fn heap_mut(&mut self) -> (NonNull<<A as >::Item>, &mut usize)` — [`Array`](#array)

- <span id="smallvecdata-from-heap"></span>`fn from_heap(ptr: NonNull<<A as >::Item>, len: usize) -> SmallVecData<A>` — [`Array`](#array), [`SmallVecData`](#smallvecdata)

#### Trait Implementations

##### `impl<A: Array + Send> Send for SmallVecData<A>`

##### `impl<A: Array + Sync> Sync for SmallVecData<A>`

## Traits

### `Array`

```rust
trait Array { ... }
```

Types that can be used as the backing store for a [`SmallVec`](#smallvec).

#### Associated Types

- `type Item`

#### Required Methods

- `fn size() -> usize`

  Returns the number of items the array can hold.

#### Implementors

- `[T; 0]`
- `[T; 1024]`
- `[T; 1048576]`
- `[T; 10]`
- `[T; 11]`
- `[T; 128]`
- `[T; 12]`
- `[T; 131072]`
- `[T; 13]`
- `[T; 14]`
- `[T; 1536]`
- `[T; 15]`
- `[T; 16384]`
- `[T; 16]`
- `[T; 17]`
- `[T; 18]`
- `[T; 19]`
- `[T; 1]`
- `[T; 2048]`
- `[T; 20]`
- `[T; 21]`
- `[T; 22]`
- `[T; 23]`
- `[T; 24576]`
- `[T; 24]`
- `[T; 256]`
- `[T; 25]`
- `[T; 262144]`
- `[T; 26]`
- `[T; 27]`
- `[T; 28]`
- `[T; 29]`
- `[T; 2]`
- `[T; 30]`
- `[T; 31]`
- `[T; 32768]`
- `[T; 32]`
- `[T; 36]`
- `[T; 393216]`
- `[T; 3]`
- `[T; 4096]`
- `[T; 4]`
- `[T; 512]`
- `[T; 524288]`
- `[T; 5]`
- `[T; 64]`
- `[T; 65536]`
- `[T; 6]`
- `[T; 7]`
- `[T; 8192]`
- `[T; 8]`
- `[T; 96]`
- `[T; 9]`

### `ToSmallVec<A: Array>`

```rust
trait ToSmallVec<A: Array> { ... }
```

Convenience trait for constructing a `SmallVec`

#### Required Methods

- `fn to_smallvec(&self) -> SmallVec<A>`

  Construct a new `SmallVec` from a slice.

#### Implementors

- `[<A as >::Item]`

## Functions

### `infallible`

```rust
fn infallible<T>(result: Result<T, CollectionAllocErr>) -> T
```

### `layout_array`

```rust
fn layout_array<T>(n: usize) -> Result<alloc::alloc::Layout, CollectionAllocErr>
```

FIXME: use `Layout::array` when we require a Rust version where it’s stable
<https://github.com/rust-lang/rust/issues/55724>

### `deallocate`

```rust
unsafe fn deallocate<T>(ptr: core::ptr::NonNull<T>, capacity: usize)
```

## Macros

### `debug_unreachable!`

`panic!()` in debug builds, optimization hint in release.

### `impl_array!`

### `smallvec!`

Creates a [`SmallVec`](#smallvec) containing the arguments.

`smallvec!` allows `SmallVec`s to be defined with the same syntax as array expressions.
There are two forms of this macro:

- Create a [`SmallVec`](#smallvec) containing a given list of elements:

```rust
use smallvec::{smallvec, SmallVec};
fn main() {
let v: SmallVec<[_; 128]> = smallvec![1, 2, 3];
assert_eq!(v[0], 1);
assert_eq!(v[1], 2);
assert_eq!(v[2], 3);
}
```

- Create a [`SmallVec`](#smallvec) from a given element and size:

```rust
use smallvec::{smallvec, SmallVec};
fn main() {
let v: SmallVec<[_; 0x8000]> = smallvec![1; 3];
assert_eq!(v, SmallVec::from_buf([1, 1, 1]));
}
```

Note that unlike array expressions this syntax supports all elements
which implement `Clone` and the number of elements doesn't have to be
a constant.

This will use `clone` to duplicate an expression, so one should be careful
using this with types having a nonstandard `Clone` implementation. For
example, `smallvec![Rc::new(1); 5]` will create a vector of five references
to the same boxed integer value, not five references pointing to independently
boxed integers.

