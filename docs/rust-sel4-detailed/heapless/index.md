# Crate `heapless`

`static` friendly data structures that don't require dynamic memory allocation

The core principle behind `heapless` is that its data structures are backed by a *static* memory
allocation. For example, you can think of `heapless::Vec` as an alternative version of
`std::Vec` with fixed capacity and that can't be re-allocated on the fly (e.g. via `push`).

All `heapless` data structures store their memory allocation *inline* and specify their capacity
via their type parameter `N`. This means that you can instantiate a `heapless` data structure on
the stack, in a `static` variable, or even in the heap.

```rust
use heapless::Vec; // fixed capacity `std::Vec`

// on the stack
let mut xs: Vec<u8, 8> = Vec::new(); // can hold up to 8 elements
xs.push(42)?;
assert_eq!(xs.pop(), Some(42));

// in a `static` variable
static mut XS: Vec<u8, 8> = Vec::new();

let xs = unsafe { &mut XS };

xs.push(42)?;
assert_eq!(xs.pop(), Some(42));

// in the heap (though kind of pointless because no reallocation)
let mut ys: Box<Vec<u8, 8>> = Box::new(Vec::new());
ys.push(42)?;
assert_eq!(ys.pop(), Some(42));
Ok::<(), u8>(())
```

Because they have fixed capacity `heapless` data structures don't implicitly reallocate. This
means that operations like `heapless::Vec.push` are *truly* constant time rather than amortized
constant time with potentially unbounded (depends on the allocator) worst case execution time
(which is bad/unacceptable for hard real time applications).

`heapless` data structures don't use a memory allocator which means no risk of an uncatchable
Out Of Memory (OOM) condition while performing operations on them. It's certainly possible to
run out of capacity while growing `heapless` data structures, but the API lets you handle this
possibility by returning a `Result` on operations that may exhaust the capacity of the data
structure.

List of currently implemented data structures:
- [`BinaryHeap`](binary_heap/index.md): A priority queue.
- [`Deque`](deque/index.md): A double-ended queue.
- [`HistoryBuf`](history_buf/index.md): A “history buffer”, similar to a write-only ring buffer.
- [`IndexMap`](index_map/index.md): A hash table.
- [`IndexSet`](index_set/index.md): A hash set.
- [`LinearMap`](linear_map/index.md): A linear map.
- [`SortedLinkedList`](sorted_linked_list::SortedLinkedList): A sorted linked list.
- [`String`](string/index.md): A string.
- [`Vec`](vec/index.md): A vector.
- [`mpmc::MpMcQueue`](mpmc): A lock-free multiple-producer, multiple-consumer queue.
- [`spsc::Queue`](spsc): A lock-free single-producer, single-consumer queue.

# Minimum Supported Rust Version (MSRV)

This crate does *not* have a Minimum Supported Rust Version (MSRV) and may make use of language
features and API in the standard library available in the latest stable Rust version.

In other words, changes in the Rust version requirement of this crate are not considered semver
breaking change and may occur in patch version releases.

## Contents

- [Modules](#modules)
  - [`c_string`](#c-string)
  - [`deque`](#deque)
  - [`history_buf`](#history-buf)
  - [`index_map`](#index-map)
  - [`index_set`](#index-set)
  - [`len_type`](#len-type)
  - [`linear_map`](#linear-map)
  - [`slice`](#slice)
  - [`storage`](#storage)
  - [`string`](#string)
  - [`vec`](#vec)
  - [`de`](#de)
  - [`ser`](#ser)
  - [`binary_heap`](#binary-heap)
  - [`mpmc`](#mpmc)
  - [`sorted_linked_list`](#sorted-linked-list)
  - [`spsc`](#spsc)
- [Structs](#structs)
  - [`CString`](#cstring)
  - [`OldestOrdered`](#oldestordered)
  - [`IndexMap`](#indexmap)
  - [`IndexSet`](#indexset)
  - [`CapacityError`](#capacityerror)
- [Traits](#traits)
  - [`LenType`](#lentype)
- [Functions](#functions)
  - [`dead_code_ice_workaround`](#dead-code-ice-workaround)
- [Type Aliases](#type-aliases)
  - [`BinaryHeap`](#binaryheap)
  - [`Deque`](#deque)
  - [`HistoryBuf`](#historybuf)
  - [`LinearMap`](#linearmap)
  - [`String`](#string)
  - [`Vec`](#vec)
  - [`VecView`](#vecview)
- [Macros](#macros)
  - [`format!`](#format)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`c_string`](#c-string) | mod | A fixed capacity [`CString`](https://doc.rust-lang.org/std/ffi/struct.CString.html). |
| [`deque`](#deque) | mod | A fixed capacity double-ended queue. |
| [`history_buf`](#history-buf) | mod | A "history buffer", similar to a write-only ring buffer of fixed length. |
| [`index_map`](#index-map) | mod | A fixed-capacity hash table where the iteration order is independent of the hash of the keys. |
| [`index_set`](#index-set) | mod | A fixed-capacity hash set where the iteration order is independent of the hash values. |
| [`len_type`](#len-type) | mod |  |
| [`linear_map`](#linear-map) | mod | A fixed capacity map/dictionary that performs lookups via linear search. |
| [`slice`](#slice) | mod |  |
| [`storage`](#storage) | mod | `Storage` trait defining how data is stored in a container. |
| [`string`](#string) | mod | A fixed capacity [`String`](https://doc.rust-lang.org/std/string/struct.String.html). |
| [`vec`](#vec) | mod | A fixed capacity [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html). |
| [`de`](#de) | mod |  |
| [`ser`](#ser) | mod |  |
| [`binary_heap`](#binary-heap) | mod | A priority queue implemented with a binary heap. |
| [`mpmc`](#mpmc) | mod | A fixed capacity multiple-producer, multiple-consumer (MPMC) lock-free queue. |
| [`sorted_linked_list`](#sorted-linked-list) | mod | A fixed sorted priority linked list, similar to [`BinaryHeap`] but with different properties on `push`, `pop`, etc. |
| [`spsc`](#spsc) | mod | A fixed capacity single-producer, single-consumer (SPSC) lock-free queue. |
| [`CString`](#cstring) | struct |  |
| [`OldestOrdered`](#oldestordered) | struct |  |
| [`IndexMap`](#indexmap) | struct |  |
| [`IndexSet`](#indexset) | struct |  |
| [`CapacityError`](#capacityerror) | struct | The error type for fallible [`Vec`] and [`String`] methods. |
| [`LenType`](#lentype) | trait |  |
| [`dead_code_ice_workaround`](#dead-code-ice-workaround) | fn |  |
| [`BinaryHeap`](#binaryheap) | type |  |
| [`Deque`](#deque) | type |  |
| [`HistoryBuf`](#historybuf) | type |  |
| [`LinearMap`](#linearmap) | type |  |
| [`String`](#string) | type |  |
| [`Vec`](#vec) | type |  |
| [`VecView`](#vecview) | type |  |
| [`format!`](#format) | macro | Macro that creates a fixed capacity [`String`]. |

## Modules

- [`c_string`](c_string/index.md) — A fixed capacity [`CString`](https://doc.rust-lang.org/std/ffi/struct.CString.html).
- [`deque`](deque/index.md) — A fixed capacity double-ended queue.
- [`history_buf`](history_buf/index.md) — A "history buffer", similar to a write-only ring buffer of fixed length.
- [`index_map`](index_map/index.md) — A fixed-capacity hash table where the iteration order is independent of the hash of the keys.
- [`index_set`](index_set/index.md) — A fixed-capacity hash set where the iteration order is independent of the hash values.
- [`len_type`](len_type/index.md)
- [`linear_map`](linear_map/index.md) — A fixed capacity map/dictionary that performs lookups via linear search.
- [`slice`](slice/index.md)
- [`storage`](storage/index.md) — `Storage` trait defining how data is stored in a container.
- [`string`](string/index.md) — A fixed capacity [`String`](https://doc.rust-lang.org/std/string/struct.String.html).
- [`vec`](vec/index.md) — A fixed capacity [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html).
- [`de`](de/index.md)
- [`ser`](ser/index.md)
- [`binary_heap`](binary_heap/index.md) — A priority queue implemented with a binary heap.
- [`mpmc`](mpmc/index.md) — A fixed capacity multiple-producer, multiple-consumer (MPMC) lock-free queue.
- [`sorted_linked_list`](sorted_linked_list/index.md) — A fixed sorted priority linked list, similar to [`BinaryHeap`] but with different properties
- [`spsc`](spsc/index.md) — A fixed capacity single-producer, single-consumer (SPSC) lock-free queue.

## Structs

### `CString<const N: usize, LenT: LenType>`

```rust
struct CString<const N: usize, LenT: LenType> {
    inner: crate::vec::Vec<u8, N, LenT>,
}
```

A fixed capacity [`CString`](https://doc.rust-lang.org/std/ffi/struct.CString.html).

It stores up to `N - 1` non-nul characters with a trailing nul terminator.

#### Implementations

- <span id="cstring-new"></span>`fn new() -> Self`

  Creates a new C-compatible string with a terminating nul byte.

  

  ```rust

  use heapless::CString;

  

  // A fixed-size `CString` that can store up to 10 characters

  // including the nul terminator.

  let empty = CString::<10>::new();

  

  assert_eq!(empty.as_c_str(), c"");

  assert_eq!(empty.to_str(), Ok(""));

  ```

- <span id="cstring-from-bytes-with-nul-unchecked"></span>`unsafe fn from_bytes_with_nul_unchecked(bytes: &[u8]) -> Result<Self, CapacityError>` — [`CapacityError`](#capacityerror)

  Unsafely creates a [`CString`](c_string/index.md) from a byte slice.

  

  This function will copy the provided `bytes` to a [`CString`](c_string/index.md) without

  performing any sanity checks.

  

  The function will fail if `bytes.len() > N`.

  

  # Safety

  

  The provided slice **must** be nul-terminated and not contain any interior

  nul bytes.

  

  # Examples

  

  ```rust

  use heapless::CString;

  let mut c_string = unsafe { CString::<7>::from_bytes_with_nul_unchecked(b"string\0").unwrap() };

  

  assert_eq!(c_string.to_str(), Ok("string"));

  ```

- <span id="cstring-from-bytes-with-nul"></span>`fn from_bytes_with_nul(bytes: &[u8]) -> Result<Self, ExtendError>` — [`ExtendError`](c_string/index.md#extenderror)

  Instantiates a [`CString`](c_string/index.md) copying from the giving byte slice, assuming it is

  nul-terminated.

  

  Fails if the given byte slice has any interior nul byte, if the slice does not

  end with a nul byte, or if the byte slice can't fit in `N`.

- <span id="cstring-from-raw"></span>`unsafe fn from_raw(ptr: *const c_char) -> Result<Self, ExtendError>` — [`ExtendError`](c_string/index.md#extenderror)

  Builds a [`CString`](c_string/index.md) copying from a raw C string pointer.

  

  # Safety

  

  - The memory pointed to by `ptr` must contain a valid nul terminator at the

    end of the string.

  - `ptr` must be valid for reads of bytes up to and including the nul terminator.

    This means in particular:

      - The entire memory range of this `CStr` must be contained within a single allocated object!

      - `ptr` must be non-nul even for a zero-length `CStr`.

  

  # Example

  

  ```rust

  use core::ffi::{c_char, CStr};

  use heapless::CString;

  

  const HELLO_PTR: *const c_char = {

      const BYTES: &[u8] = b"Hello, world!\0";

      BYTES.as_ptr().cast()

  };

  

  let copied = unsafe { CString::<14>::from_raw(HELLO_PTR) }.unwrap();

  

  assert_eq!(copied.to_str(), Ok("Hello, world!"));

  ```

- <span id="cstring-as-c-str"></span>`fn as_c_str(&self) -> &CStr`

  Converts the [`CString`](c_string/index.md) to a [`CStr`](../serde_core/lib/index.md) slice.

- <span id="cstring-capacity-with-bytes"></span>`fn capacity_with_bytes(&self, bytes: &[u8]) -> Option<usize>`

  Calculates the length of `self.inner` would have if it appended `bytes`.

- <span id="cstring-extend-from-bytes"></span>`fn extend_from_bytes(&mut self, bytes: &[u8]) -> Result<(), ExtendError>` — [`ExtendError`](c_string/index.md#extenderror)

  Extends the [`CString`](c_string/index.md) with the given bytes.

  

  This function fails if the [`CString`](c_string/index.md) would not have enough capacity to append the bytes or

  if the bytes contain an interior nul byte.

  

  # Example

  

  ```rust

  use heapless::CString;

  

  let mut c_string = CString::<10>::new();

  

  c_string.extend_from_bytes(b"hey").unwrap();

  c_string.extend_from_bytes(b" there\0").unwrap();

  

  assert_eq!(c_string.to_str(), Ok("hey there"));

  ```

- <span id="cstring-pop-terminator"></span>`unsafe fn pop_terminator(&mut self)`

  Removes the nul byte terminator from the inner buffer.

  

  # Safety

  

  Callers must ensure to add the nul terminator back after this function is called.

- <span id="cstring-extend-from-bytes-unchecked"></span>`unsafe fn extend_from_bytes_unchecked(&mut self, additional: &[u8]) -> Result<(), CapacityError>` — [`CapacityError`](#capacityerror)

  Removes the existing nul terminator and then extends `self` with the given bytes.

  

  # Safety

  

  If `additional` is not nul-terminated, the [`CString`](c_string/index.md) is left non nul-terminated, which is

  an invalid state. Caller must ensure that either `additional` has a terminating nul byte

  or ensure to append a trailing nul terminator.

- <span id="cstring-as-bytes-with-nul"></span>`fn as_bytes_with_nul(&self) -> &[u8]`

  Returns the underlying byte slice including the trailing nul terminator.

  

  # Example

  

  ```rust

  use heapless::CString;

  

  let mut c_string = CString::<5>::new();

  c_string.extend_from_bytes(b"abc").unwrap();

  

  assert_eq!(c_string.as_bytes_with_nul(), b"abc\0");

  ```

- <span id="cstring-as-bytes"></span>`fn as_bytes(&self) -> &[u8]`

  Returns the underlying byte slice excluding the trailing nul terminator.

  

  # Example

  

  ```rust

  use heapless::CString;

  

  let mut c_string = CString::<5>::new();

  c_string.extend_from_bytes(b"abc").unwrap();

  

  assert_eq!(c_string.as_bytes(), b"abc");

  ```

#### Trait Implementations

##### `impl<LenT: LenType> AsRef for CString<N, LenT>`

- <span id="cstring-asref-as-ref"></span>`fn as_ref(&self) -> &CStr`

##### `impl<LenT: clone::Clone + LenType> Clone for CString<N, LenT>`

- <span id="cstring-clone"></span>`fn clone(&self) -> CString<N, LenT>` — [`CString`](c_string/index.md#cstring)

##### `impl<LenT: LenType> Debug for CString<N, LenT>`

- <span id="cstring-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<LenT: LenType> Default for CString<N, LenT>`

- <span id="cstring-default"></span>`fn default() -> Self`

##### `impl<LenT: LenType> Deref for CString<N, LenT>`

- <span id="cstring-deref-type-target"></span>`type Target = CStr`

- <span id="cstring-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl<LenT: LenType> Eq for CString<N, LenT>`

##### `impl<LenT: hash::Hash + LenType> Hash for CString<N, LenT>`

- <span id="cstring-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<LenT: LenType> Ord for CString<N, LenT>`

- <span id="cstring-ord-cmp"></span>`fn cmp(&self, rhs: &Self) -> Ordering`

##### `impl<LenT1: LenType, LenT2: LenType> PartialEq for CString<N, LenT1>`

- <span id="cstring-partialeq-eq"></span>`fn eq(&self, rhs: &CString<M, LenT2>) -> bool` — [`CString`](c_string/index.md#cstring)

##### `impl<LenT1: LenType, LenT2: LenType> PartialOrd for CString<N, LenT1>`

- <span id="cstring-partialord-partial-cmp"></span>`fn partial_cmp(&self, rhs: &CString<M, LenT2>) -> Option<Ordering>` — [`CString`](c_string/index.md#cstring)

##### `impl Receiver for CString<N, LenT>`

- <span id="cstring-receiver-type-target"></span>`type Target = T`

### `OldestOrdered<'a, T>`

```rust
struct OldestOrdered<'a, T> {
    inner: core::iter::Chain<core::slice::Iter<'a, T>, core::slice::Iter<'a, T>>,
}
```

Double ended iterator on the underlying buffer ordered from the oldest data
to the newest.

#### Trait Implementations

##### `impl<T> Clone for OldestOrdered<'_, T>`

- <span id="oldestordered-clone"></span>`fn clone(&self) -> Self`

##### `impl<T> DoubleEndedIterator for OldestOrdered<'_, T>`

- <span id="oldestordered-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl IntoIterator for OldestOrdered<'a, T>`

- <span id="oldestordered-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="oldestordered-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="oldestordered-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T> Iterator for OldestOrdered<'a, T>`

- <span id="oldestordered-iterator-type-item"></span>`type Item = &'a T`

- <span id="oldestordered-iterator-next"></span>`fn next(&mut self) -> Option<&'a T>`

- <span id="oldestordered-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `IndexMap<K, V, S, const N: usize>`

```rust
struct IndexMap<K, V, S, const N: usize> {
    core: CoreMap<K, V, N>,
    build_hasher: S,
}
```

Fixed capacity [`IndexMap`](https://docs.rs/indexmap/2/indexmap/map/struct.IndexMap.html)

Note that you cannot use `IndexMap` directly, since it is generic around the hashing algorithm
in use. Pick a concrete instantiation like [`FnvIndexMap`](index_map/index.md) instead
or create your own.

Note that the capacity of the `IndexMap` must be a power of 2.

# Examples

Since `IndexMap` cannot be used directly, we're using its `FnvIndexMap` instantiation
for this example.

```rust
use heapless::index_map::FnvIndexMap;

// A hash map with a capacity of 16 key-value pairs allocated on the stack
let mut book_reviews = FnvIndexMap::<_, _, 16>::new();

// review some books.
book_reviews
    .insert("Adventures of Huckleberry Finn", "My favorite book.")
    .unwrap();
book_reviews
    .insert("Grimms' Fairy Tales", "Masterpiece.")
    .unwrap();
book_reviews
    .insert("Pride and Prejudice", "Very enjoyable.")
    .unwrap();
book_reviews
    .insert("The Adventures of Sherlock Holmes", "Eye lyked it alot.")
    .unwrap();

// check for a specific one.
if !book_reviews.contains_key("Les Misérables") {
    println!(
        "We've got {} reviews, but Les Misérables ain't one.",
        book_reviews.len()
    );
}

// oops, this review has a lot of spelling mistakes, let's delete it.
book_reviews.remove("The Adventures of Sherlock Holmes");

// look up the values associated with some keys.
let to_find = ["Pride and Prejudice", "Alice's Adventure in Wonderland"];
for book in &to_find {
    match book_reviews.get(book) {
        Some(review) => println!("{}: {}", book, review),
        None => println!("{} is unreviewed.", book),
    }
}

// iterate over everything.
for (book, review) in &book_reviews {
    println!("{}: \"{}\"", book, review);
}
```

#### Implementations

- <span id="indexmap-new"></span>`const fn new() -> Self`

  Creates an empty `IndexMap`.

#### Trait Implementations

##### `impl<K, V, S> Clone for IndexMap<K, V, S, N>`

- <span id="indexmap-clone"></span>`fn clone(&self) -> Self`

##### `impl<K, V, S> Debug for IndexMap<K, V, S, N>`

- <span id="indexmap-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<K, V, S> Default for IndexMap<K, V, S, N>`

- <span id="indexmap-default"></span>`fn default() -> Self`

##### `impl<K, V, S> Deserialize for crate::IndexMap<K, V, hash32::BuildHasherDefault<S>, N>`

- <span id="crateindexmap-deserialize"></span>`fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>`

##### `impl DeserializeOwned for IndexMap<K, V, S, N>`

##### `impl<K, V, S> Eq for IndexMap<K, V, S, N>`

##### `impl<K, V, S> Extend for IndexMap<K, V, S, N>`

- <span id="indexmap-extend"></span>`fn extend<I>(&mut self, iterable: I)`

##### `impl<K, V, S> FromIterator for IndexMap<K, V, S, N>`

- <span id="indexmap-fromiterator-from-iter"></span>`fn from_iter<I>(iterable: I) -> Self`

##### `impl<K, Q, V, S> Index for IndexMap<K, V, S, N>`

- <span id="indexmap-index-type-output"></span>`type Output = V`

- <span id="indexmap-index"></span>`fn index(&self, key: &Q) -> &V`

##### `impl<K, Q, V, S> IndexMut for IndexMap<K, V, S, N>`

- <span id="indexmap-indexmut-index-mut"></span>`fn index_mut(&mut self, key: &Q) -> &mut V`

##### `impl<K, V, S> IntoIterator for IndexMap<K, V, S, N>`

- <span id="indexmap-intoiterator-type-item"></span>`type Item = (K, V)`

- <span id="indexmap-intoiterator-type-intoiter"></span>`type IntoIter = IntoIter<K, V, N>`

- <span id="indexmap-intoiterator-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl<K, V1, V2, S1, S2> PartialEq for IndexMap<K, V1, S1, N1>`

- <span id="indexmap-partialeq-eq"></span>`fn eq(&self, other: &IndexMap<K, V2, S2, N2>) -> bool` — [`IndexMap`](index_map/index.md#indexmap)

##### `impl<K, V, S> Serialize for crate::IndexMap<K, V, S, N>`

- <span id="crateindexmap-serialize"></span>`fn serialize<SER>(&self, serializer: SER) -> Result<<SER as >::Ok, <SER as >::Error>`

### `IndexSet<T, S, const N: usize>`

```rust
struct IndexSet<T, S, const N: usize> {
    map: crate::index_map::IndexMap<T, (), S, N>,
}
```

Fixed capacity [`IndexSet`](https://docs.rs/indexmap/2/indexmap/set/struct.IndexSet.html).

Note that you cannot use `IndexSet` directly, since it is generic around the hashing algorithm
in use. Pick a concrete instantiation like [`FnvIndexSet`](index_set/index.md) instead
or create your own.

Note that the capacity of the `IndexSet` must be a power of 2.

# Examples
Since `IndexSet` cannot be used directly, we're using its `FnvIndexSet` instantiation
for this example.

```rust
use heapless::index_set::FnvIndexSet;

// A hash set with a capacity of 16 elements allocated on the stack
let mut books = FnvIndexSet::<_, 16>::new();

// Add some books.
books.insert("A Dance With Dragons").unwrap();
books.insert("To Kill a Mockingbird").unwrap();
books.insert("The Odyssey").unwrap();
books.insert("The Great Gatsby").unwrap();

// Check for a specific one.
if !books.contains("The Winds of Winter") {
    println!(
        "We have {} books, but The Winds of Winter ain't one.",
        books.len()
    );
}

// Remove a book.
books.remove("The Odyssey");

// Iterate over everything.
for book in &books {
    println!("{}", book);
}
```

#### Implementations

- <span id="indexset-new"></span>`const fn new() -> Self`

  Creates an empty `IndexSet`

#### Trait Implementations

##### `impl<T, S> Clone for IndexSet<T, S, N>`

- <span id="indexset-clone"></span>`fn clone(&self) -> Self`

##### `impl<T, S> Debug for IndexSet<T, S, N>`

- <span id="indexset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T, S> Default for IndexSet<T, S, N>`

- <span id="indexset-default"></span>`fn default() -> Self`

##### `impl<T, S> Deserialize for crate::IndexSet<T, hash32::BuildHasherDefault<S>, N>`

- <span id="crateindexset-deserialize"></span>`fn deserialize<D>(deserializer: D) -> Result<Self, <D as >::Error>`

##### `impl<T> DeserializeOwned for IndexSet<T, S, N>`

##### `impl<T, S> Extend for IndexSet<T, S, N>`

- <span id="indexset-extend"></span>`fn extend<I>(&mut self, iterable: I)`

##### `impl<T, S> FromIterator for IndexSet<T, S, N>`

- <span id="indexset-fromiterator-from-iter"></span>`fn from_iter<I>(iter: I) -> Self`

##### `impl<T, S> IntoIterator for &'a IndexSet<T, S, N>`

- <span id="a-indexset-intoiterator-type-item"></span>`type Item = &'a T`

- <span id="a-indexset-intoiterator-type-intoiter"></span>`type IntoIter = Iter<'a, T>`

- <span id="a-indexset-intoiterator-into-iter"></span>`fn into_iter(self) -> <Self as >::IntoIter`

##### `impl<T, S1, S2> PartialEq for IndexSet<T, S1, N1>`

- <span id="indexset-partialeq-eq"></span>`fn eq(&self, other: &IndexSet<T, S2, N2>) -> bool` — [`IndexSet`](index_set/index.md#indexset)

##### `impl<T, S> Serialize for crate::IndexSet<T, S, N>`

- <span id="crateindexset-serialize"></span>`fn serialize<SER>(&self, serializer: SER) -> Result<<SER as >::Ok, <SER as >::Error>`

### `CapacityError`

```rust
struct CapacityError;
```

The error type for fallible [`Vec`](vec/index.md) and [`String`](string/index.md) methods.

#### Trait Implementations

##### `impl Debug for CapacityError`

- <span id="capacityerror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for CapacityError`

- <span id="capacityerror-display-fmt"></span>`fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result`

##### `impl Error for CapacityError`

##### `impl ToString for CapacityError`

- <span id="capacityerror-tostring-to-string"></span>`fn to_string(&self) -> String`

## Traits

### `LenType`

```rust
trait LenType: Sealed { ... }
```

A sealed trait representing a valid type to use as a length for a container.

This cannot be implemented in user code, and is restricted to `u8`, `u16`, `u32`, and `usize`.

#### Implementors

- `u16`
- `u32`
- `u8`
- `usize`

## Functions

### `dead_code_ice_workaround`

```rust
fn dead_code_ice_workaround()
```

## Type Aliases

### `BinaryHeap<T, K, const N: usize>`

```rust
type BinaryHeap<T, K, const N: usize> = BinaryHeapInner<T, K, crate::vec::OwnedVecStorage<T, N>>;
```

A priority queue implemented with a binary heap.

This can be either a min-heap or a max-heap.

It is a logic error for an item to be modified in such a way that the item's ordering relative
to any other item, as determined by the `Ord` trait, changes while it is in the heap. This is
normally only possible through `Cell`, `RefCell`, global state, I/O, or unsafe code.

```rust
use heapless::binary_heap::{BinaryHeap, Max};

let mut heap: BinaryHeap<_, Max, 8> = BinaryHeap::new();

// We can use peek to look at the next item in the heap. In this case,
// there's no items in there yet so we get None.
assert_eq!(heap.peek(), None);

// Let's add some scores...
heap.push(1).unwrap();
heap.push(5).unwrap();
heap.push(2).unwrap();

// Now peek shows the most important item in the heap.
assert_eq!(heap.peek(), Some(&5));

// We can check the length of a heap.
assert_eq!(heap.len(), 3);

// We can iterate over the items in the heap, although they are returned in
// a random order.
for x in &heap {
    println!("{}", x);
}

// If we instead pop these scores, they should come back in order.
assert_eq!(heap.pop(), Some(5));
assert_eq!(heap.pop(), Some(2));
assert_eq!(heap.pop(), Some(1));
assert_eq!(heap.pop(), None);

// We can clear the heap of any remaining items.
heap.clear();

// The heap should now be empty.
assert!(heap.is_empty())
```

### `Deque<T, const N: usize>`

```rust
type Deque<T, const N: usize> = DequeInner<T, crate::vec::OwnedVecStorage<T, N>>;
```

A fixed capacity double-ended queue.

# Examples

```rust
use heapless::Deque;

// A deque with a fixed capacity of 8 elements allocated on the stack
let mut deque = Deque::<_, 8>::new();

// You can use it as a good old FIFO queue.
deque.push_back(1);
deque.push_back(2);
assert_eq!(deque.len(), 2);

assert_eq!(deque.pop_front(), Some(1));
assert_eq!(deque.pop_front(), Some(2));
assert_eq!(deque.len(), 0);

// Deque is double-ended, you can push and pop from the front and back.
deque.push_back(1);
deque.push_front(2);
deque.push_back(3);
deque.push_front(4);
assert_eq!(deque.pop_front(), Some(4));
assert_eq!(deque.pop_front(), Some(2));
assert_eq!(deque.pop_front(), Some(1));
assert_eq!(deque.pop_front(), Some(3));

// You can iterate it, yielding all the elements front-to-back.
for x in &deque {
    println!("{}", x);
}
```

### `HistoryBuf<T, const N: usize>`

```rust
type HistoryBuf<T, const N: usize> = HistoryBufInner<T, OwnedHistoryBufStorage<T, N>>;
```

A "history buffer", similar to a write-only ring buffer of fixed length.

This buffer keeps a fixed number of elements.  On write, the oldest element
is overwritten. Thus, the buffer is useful to keep a history of values with
some desired depth, and for example calculate a rolling average.

# Examples
```rust
use heapless::HistoryBuf;

// Initialize a new buffer with 8 elements.
let mut buf = HistoryBuf::<_, 8>::new();

// Starts with no data
assert_eq!(buf.recent(), None);

buf.write(3);
buf.write(5);
buf.extend(&[4, 4]);

// The most recent written element is a four.
assert_eq!(buf.recent(), Some(&4));

// To access all elements in an unspecified order, use `as_slice()`.
for el in buf.as_slice() {
    println!("{:?}", el);
}

// Now we can prepare an average of all values, which comes out to 4.
let avg = buf.as_slice().iter().sum::<usize>() / buf.len();
assert_eq!(avg, 4);
```

### `LinearMap<K, V, const N: usize>`

```rust
type LinearMap<K, V, const N: usize> = LinearMapInner<K, V, OwnedStorage<K, V, N>>;
```

A fixed capacity map/dictionary that performs lookups via linear search.

Note that as this map doesn't use hashing so most operations are *O*(n) instead of *O*(1).

### `String<const N: usize, LenT>`

```rust
type String<const N: usize, LenT> = StringInner<LenT, OwnedStorage<N>>;
```

A fixed capacity [`String`](https://doc.rust-lang.org/std/string/struct.String.html).

### `Vec<T, const N: usize, LenT>`

```rust
type Vec<T, const N: usize, LenT> = VecInner<T, LenT, OwnedVecStorage<T, N>>;
```

A fixed capacity [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html).

# Examples

```rust
use heapless::Vec;

// A vector with a fixed capacity of 8 elements allocated on the stack
let mut vec = Vec::<_, 8>::new();
vec.push(1).unwrap();
vec.push(2).unwrap();

assert_eq!(vec.len(), 2);
assert_eq!(vec[0], 1);

assert_eq!(vec.pop(), Some(2));
assert_eq!(vec.len(), 1);

vec[0] = 7;
assert_eq!(vec[0], 7);

vec.extend([1, 2, 3].iter().cloned());

for x in &vec {
    println!("{}", x);
}
assert_eq!(*vec, [7, 1, 2, 3]);
```

In some cases, the const-generic might be cumbersome. `Vec` can coerce into a [`VecView`](vec/index.md) to remove the need for the const-generic:

```rust
use heapless::{Vec, VecView};

let vec: Vec<u8, 10> = Vec::from_slice(&[1, 2, 3, 4]).unwrap();
let view: &VecView<_, _> = &vec;
```

For uncommmon capacity values, or in generic scenarios, you may have to provide the `LenT` generic yourself.

This should be the smallest unsigned integer type that your capacity fits in, or `usize` if you don't want to consider this.

### `VecView<T, LenT>`

```rust
type VecView<T, LenT> = VecInner<T, LenT, ViewVecStorage<T>>;
```

A [`Vec`](vec/index.md) with dynamic capacity

[`Vec`](vec/index.md) coerces to `VecView`. `VecView` is `!Sized`, meaning it can only ever be used by reference.

Unlike [`Vec`](vec/index.md), `VecView` does not have an `N` const-generic parameter.
This has the ergonomic advantage of making it possible to use functions without needing to know at
compile-time the size of the buffers used, for example for use in `dyn` traits.

`VecView<T>` is to `Vec<T, N>` what `[T]` is to `[T; N]`.

```rust
use heapless::{Vec, VecView};

let mut vec: Vec<u8, 10> = Vec::from_slice(&[1, 2, 3, 4]).unwrap();
let view: &VecView<_, _> = &vec;
assert_eq!(view, &[1, 2, 3, 4]);

let mut_view: &mut VecView<_, _> = &mut vec;
mut_view.push(5);
assert_eq!(vec, [1, 2, 3, 4, 5]);
```

## Macros

### `format!`

Macro that creates a fixed capacity [`String`](string/index.md). Equivalent to [`format!`](https://doc.rust-lang.org/std/macro.format.html).

The macro's arguments work in the same way as the regular macro.

It is possible to explicitly specify the capacity of the returned string as the first argument.
In this case it is necessary to disambiguate by separating the capacity with a semicolon.

# Errors

There are two possible error cases. Both return the unit type [`core::fmt::Error`](../sel4/error/index.md).

- In case the formatting exceeds the string's capacity. This error does not exist in
  the standard library as the string would just grow.
- If a formatting trait implementation returns an error. The standard library panics
  in this case.

# Examples

```rust
fn main() -> Result<(), core::fmt::Error> {
use heapless::{format, String};

// Notice semicolon instead of comma!
format!(4; "test")?;
format!(15; "hello {}", "world!")?;
format!(20; "x = {}, y = {y}", 10, y = 30)?;
let (x, y) = (1, 2);
format!(12; "{x} + {y} = 3")?;

let implicit: String<10> = format!("speed = {}", 7)?;
Ok(())
}
```

