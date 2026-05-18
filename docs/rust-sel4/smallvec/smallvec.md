**smallvec**

# Module: smallvec

## Contents

**Macros**

- [`smallvec`](#smallvec) - Creates a [`SmallVec`] containing the arguments.

**Structs**

- [`Drain`](#drain) - An iterator that removes the items from a `SmallVec` and yields them by value.
- [`IntoIter`](#intoiter) - An iterator that consumes a `SmallVec` and yields its items by value.
- [`SmallVec`](#smallvec) - A `Vec`-like container that can store a small number of elements inline.

**Enums**

- [`CollectionAllocErr`](#collectionallocerr) - Error type for APIs with fallible heap allocation

**Traits**

- [`Array`](#array) - Types that can be used as the backing store for a [`SmallVec`].
- [`ToSmallVec`](#tosmallvec) - Convenience trait for constructing a `SmallVec`

---

## smallvec::Array

*Trait*

Types that can be used as the backing store for a [`SmallVec`].

**Methods:**

- `Item`: The type of the array's elements.
- `size`: Returns the number of items the array can hold.



## smallvec::CollectionAllocErr

*Enum*

Error type for APIs with fallible heap allocation

**Variants:**
- `CapacityOverflow` - Overflow `usize::MAX` or other error during size computation
- `AllocErr{ layout: alloc::alloc::Layout }` - The allocator return an error

**Trait Implementations:**

- **From**
  - `fn from(_: LayoutErr) -> Self`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## smallvec::Drain

*Struct*

An iterator that removes the items from a `SmallVec` and yields them by value.

Returned from [`SmallVec::drain`][1].

[1]: struct.SmallVec.html#method.drain

**Generic Parameters:**
- 'a
- T

**Traits:** Send, Sync, FusedIterator

**Trait Implementations:**

- **ExactSizeIterator**
  - `fn len(self: &Self) -> usize`
- **DoubleEndedIterator**
  - `fn next_back(self: & mut Self) -> Option<<T as >::Item>`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<T as >::Item>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Drop**
  - `fn drop(self: & mut Self)`



## smallvec::IntoIter

*Struct*

An iterator that consumes a `SmallVec` and yields its items by value.

Returned from [`SmallVec::into_iter`][1].

[1]: struct.SmallVec.html#method.into_iter

**Generic Parameters:**
- A

**Methods:**

- `fn as_slice(self: &Self) -> &[<A as >::Item]` - Returns the remaining items of this iterator as a slice.
- `fn as_mut_slice(self: & mut Self) -> & mut [<A as >::Item]` - Returns the remaining items of this iterator as a mutable slice.

**Traits:** FusedIterator, ExactSizeIterator

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<A as >::Item>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`
- **Drop**
  - `fn drop(self: & mut Self)`
- **Clone**
  - `fn clone(self: &Self) -> IntoIter<A>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **DoubleEndedIterator**
  - `fn next_back(self: & mut Self) -> Option<<A as >::Item>`



## smallvec::SmallVec

*Struct*

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

**Generic Parameters:**
- A

**Methods:**

- `fn resize(self: & mut Self, len: usize, value: <A as >::Item)` - Resizes the vector so that its length is equal to `len`.
- `fn from_elem(elem: <A as >::Item, n: usize) -> Self` - Creates a `SmallVec` with `n` copies of `elem`.
- `fn from_slice(slice: &[<A as >::Item]) -> Self` - Copy the elements from a slice into a new `SmallVec`.
- `fn insert_from_slice(self: & mut Self, index: usize, slice: &[<A as >::Item])` - Copy elements from a slice into the vector at position `index`, shifting any following
- `fn extend_from_slice(self: & mut Self, slice: &[<A as >::Item])` - Copy elements from a slice and append them to the vector.
- `fn new() -> SmallVec<A>` - Construct an empty vector
- `fn with_capacity(n: usize) -> Self` - Construct an empty vector with enough capacity pre-allocated to store at least `n`
- `fn from_vec(vec: Vec<<A as >::Item>) -> SmallVec<A>` - Construct a new `SmallVec` from a `Vec<A::Item>`.
- `fn from_buf(buf: A) -> SmallVec<A>` - Constructs a new `SmallVec` on the stack from an `A` without
- `fn from_buf_and_len(buf: A, len: usize) -> SmallVec<A>` - Constructs a new `SmallVec` on the stack from an `A` without
- `fn from_buf_and_len_unchecked(buf: MaybeUninit<A>, len: usize) -> SmallVec<A>` - Constructs a new `SmallVec` on the stack from an `A` without
- `fn set_len(self: & mut Self, new_len: usize)` - Sets the length of a vector.
- `fn inline_size(self: &Self) -> usize` - The maximum number of elements this vector can hold inline
- `fn len(self: &Self) -> usize` - The number of elements stored in the vector
- `fn is_empty(self: &Self) -> bool` - Returns `true` if the vector is empty
- `fn capacity(self: &Self) -> usize` - The number of items the vector can hold without reallocating
- `fn spilled(self: &Self) -> bool` - Returns `true` if the data has spilled into a separate heap-allocated buffer.
- `fn drain<R>(self: & mut Self, range: R) -> Drain<A>` - Creates a draining iterator that removes the specified range in the vector
- `fn push(self: & mut Self, value: <A as >::Item)` - Append an item to the vector.
- `fn pop(self: & mut Self) -> Option<<A as >::Item>` - Remove an item from the end of the vector and return it, or None if empty.
- `fn append<B>(self: & mut Self, other: & mut SmallVec<B>)` - Moves all the elements of `other` into `self`, leaving `other` empty.
- `fn grow(self: & mut Self, new_cap: usize)` - Re-allocate to set the capacity to `max(new_cap, inline_size())`.
- `fn try_grow(self: & mut Self, new_cap: usize) -> Result<(), CollectionAllocErr>` - Re-allocate to set the capacity to `max(new_cap, inline_size())`.
- `fn reserve(self: & mut Self, additional: usize)` - Reserve capacity for `additional` more elements to be inserted.
- `fn try_reserve(self: & mut Self, additional: usize) -> Result<(), CollectionAllocErr>` - Reserve capacity for `additional` more elements to be inserted.
- `fn reserve_exact(self: & mut Self, additional: usize)` - Reserve the minimum capacity for `additional` more elements to be inserted.
- `fn try_reserve_exact(self: & mut Self, additional: usize) -> Result<(), CollectionAllocErr>` - Reserve the minimum capacity for `additional` more elements to be inserted.
- `fn shrink_to_fit(self: & mut Self)` - Shrink the capacity of the vector as much as possible.
- `fn truncate(self: & mut Self, len: usize)` - Shorten the vector, keeping the first `len` elements and dropping the rest.
- `fn as_slice(self: &Self) -> &[<A as >::Item]` - Extracts a slice containing the entire vector.
- `fn as_mut_slice(self: & mut Self) -> & mut [<A as >::Item]` - Extracts a mutable slice of the entire vector.
- `fn swap_remove(self: & mut Self, index: usize) -> <A as >::Item` - Remove the element at position `index`, replacing it with the last element.
- `fn clear(self: & mut Self)` - Remove all elements from the vector.
- `fn remove(self: & mut Self, index: usize) -> <A as >::Item` - Remove and return the element at position `index`, shifting all elements after it to the
- `fn insert(self: & mut Self, index: usize, element: <A as >::Item)` - Insert an element at position `index`, shifting all elements after it to the right.
- `fn insert_many<I>(self: & mut Self, index: usize, iterable: I)` - Insert multiple elements at position `index`, shifting all following elements toward the
- `fn into_vec(self: Self) -> Vec<<A as >::Item>` - Convert a `SmallVec` to a `Vec`, without reallocating if the `SmallVec` has already spilled onto
- `fn into_boxed_slice(self: Self) -> Box<[<A as >::Item]>` - Converts a `SmallVec` into a `Box<[T]>` without reallocating if the `SmallVec` has already spilled
- `fn into_inner(self: Self) -> Result<A, Self>` - Convert the `SmallVec` into an `A` if possible. Otherwise return `Err(Self)`.
- `fn retain<F>(self: & mut Self, f: F)` - Retains only the elements specified by the predicate.
- `fn retain_mut<F>(self: & mut Self, f: F)` - Retains only the elements specified by the predicate.
- `fn dedup(self: & mut Self)` - Removes consecutive duplicate elements.
- `fn dedup_by<F>(self: & mut Self, same_bucket: F)` - Removes consecutive duplicate elements using the given equality relation.
- `fn dedup_by_key<F, K>(self: & mut Self, key: F)` - Removes consecutive elements that map to the same key.
- `fn resize_with<F>(self: & mut Self, new_len: usize, f: F)` - Resizes the `SmallVec` in-place so that `len` is equal to `new_len`.
- `fn from_raw_parts(ptr: *mut <A as >::Item, length: usize, capacity: usize) -> SmallVec<A>` - Creates a `SmallVec` directly from the raw components of another
- `fn as_ptr(self: &Self) -> *const <A as >::Item` - Returns a raw pointer to the vector's buffer.
- `fn as_mut_ptr(self: & mut Self) -> *mut <A as >::Item` - Returns a raw mutable pointer to the vector's buffer.

**Traits:** Eq, Send

**Trait Implementations:**

- **AsRef**
  - `fn as_ref(self: &Self) -> &[<A as >::Item]`
- **FromIterator**
  - `fn from_iter<I>(iterable: I) -> SmallVec<A>`
- **Ord**
  - `fn cmp(self: &Self, other: &SmallVec<A>) -> cmp::Ordering`
- **DerefMut**
  - `fn deref_mut(self: & mut Self) -> & mut [<A as >::Item]`
- **IndexMut**
  - `fn index_mut(self: & mut Self, index: I) -> & mut <I as >::Output`
- **Deref**
  - `fn deref(self: &Self) -> &[<A as >::Item]`
- **PartialOrd**
  - `fn partial_cmp(self: &Self, other: &SmallVec<A>) -> Option<cmp::Ordering>`
- **Index**
  - `fn index(self: &Self, index: I) -> &<I as >::Output`
- **PartialEq**
  - `fn eq(self: &Self, other: &SmallVec<B>) -> bool`
- **From**
  - `fn from(array: A) -> SmallVec<A>`
- **Clone**
  - `fn clone(self: &Self) -> SmallVec<A>`
  - `fn clone_from(self: & mut Self, source: &Self)`
- **From**
  - `fn from(vec: Vec<<A as >::Item>) -> SmallVec<A>`
- **From**
  - `fn from(slice: &'a [<A as >::Item]) -> SmallVec<A>`
- **Drop**
  - `fn drop(self: & mut Self)`
- **IntoIterator**
  - `fn into_iter(self: Self) -> <Self as >::IntoIter`
- **BorrowMut**
  - `fn borrow_mut(self: & mut Self) -> & mut [<A as >::Item]`
- **Default**
  - `fn default() -> SmallVec<A>`
- **Borrow**
  - `fn borrow(self: &Self) -> &[<A as >::Item]`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **AsMut**
  - `fn as_mut(self: & mut Self) -> & mut [<A as >::Item]`
- **Extend**
  - `fn extend<I>(self: & mut Self, iterable: I)`
- **Hash**
  - `fn hash<H>(self: &Self, state: & mut H)`



## smallvec::ToSmallVec

*Trait*

Convenience trait for constructing a `SmallVec`

**Methods:**

- `to_smallvec`: Construct a new `SmallVec` from a slice.



## smallvec::smallvec

*Declarative Macro*

Creates a [`SmallVec`] containing the arguments.

`smallvec!` allows `SmallVec`s to be defined with the same syntax as array expressions.
There are two forms of this macro:

- Create a [`SmallVec`] containing a given list of elements:

```
# use smallvec::{smallvec, SmallVec};
# fn main() {
let v: SmallVec<[_; 128]> = smallvec![1, 2, 3];
assert_eq!(v[0], 1);
assert_eq!(v[1], 2);
assert_eq!(v[2], 3);
# }
```

- Create a [`SmallVec`] from a given element and size:

```
# use smallvec::{smallvec, SmallVec};
# fn main() {
let v: SmallVec<[_; 0x8000]> = smallvec![1; 3];
assert_eq!(v, SmallVec::from_buf([1, 1, 1]));
# }
```

Note that unlike array expressions this syntax supports all elements
which implement [`Clone`] and the number of elements doesn't have to be
a constant.

This will use `clone` to duplicate an expression, so one should be careful
using this with types having a nonstandard `Clone` implementation. For
example, `smallvec![Rc::new(1); 5]` will create a vector of five references
to the same boxed integer value, not five references pointing to independently
boxed integers.

```rust
macro_rules! smallvec {
    (@one $x:expr) => { ... };
    () => { ... };
    ($elem:expr; $n:expr) => { ... };
    ($($x:expr),+$(,)?) => { ... };
}
```



