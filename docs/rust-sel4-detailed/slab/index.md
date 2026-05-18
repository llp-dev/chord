# Crate `slab`

Pre-allocated storage for a uniform data type.

`Slab` provides pre-allocated storage for a single data type. If many values
of a single type are being allocated, it can be more efficient to
pre-allocate the necessary storage. Since the size of the type is uniform,
memory fragmentation can be avoided. Storing, clearing, and lookup
operations become very cheap.

While `Slab` may look like other Rust collections, it is not intended to be
used as a general purpose collection. The primary difference between `Slab`
and `Vec` is that `Slab` returns the key when storing the value.

It is important to note that keys may be reused. In other words, once a
value associated with a given key is removed from a slab, that key may be
returned from future calls to `insert`.

# Performance notes

Methods that remove values and return them, such as `Slab::remove` and
`Slab::try_remove`, might copy the removed values to the stack even if
their return values are unused. For types that don't have drop glue, the
compiler can usually elide these copies.

# Examples

Basic storing and retrieval.

```rust
use slab::*;
let mut slab = Slab::new();

let hello = slab.insert("hello");
let world = slab.insert("world");

assert_eq!(slab[hello], "hello");
assert_eq!(slab[world], "world");

slab[world] = "earth";
assert_eq!(slab[world], "earth");
```

Sometimes it is useful to be able to associate the key with the value being
inserted in the slab. This can be done with the `vacant_entry` API as such:

```rust
use slab::*;
let mut slab = Slab::new();

let hello = {
    let entry = slab.vacant_entry();
    let key = entry.key();

    entry.insert((key, "hello"));
    key
};

assert_eq!(hello, slab[hello].0);
assert_eq!("hello", slab[hello].1);
```

It is generally a good idea to specify the desired capacity of a slab at
creation time. Note that `Slab` will grow the internal capacity when
attempting to insert a new value once the existing capacity has been reached.
To avoid this, add a check.

```rust
use slab::*;
let mut slab = Slab::with_capacity(1024);

// ... use the slab

if slab.len() == slab.capacity() {
    panic!("slab full");
}

slab.insert("the slab is not at capacity yet");
```

# Capacity and reallocation

The capacity of a slab is the amount of space allocated for any future
values that will be inserted in the slab. This is not to be confused with
the *length* of the slab, which specifies the number of actual values
currently being inserted. If a slab's length is equal to its capacity, the
next value inserted into the slab will require growing the slab by
reallocating.

For example, a slab with capacity 10 and length 0 would be an empty slab
with space for 10 more stored values. Storing 10 or fewer elements into the
slab will not change its capacity or cause reallocation to occur. However,
if the slab length is increased to 11 (due to another `insert`), it will
have to reallocate, which can be slow. For this reason, it is recommended to
use `Slab::with_capacity` whenever possible to specify how many values the
slab is expected to store.

# Implementation

`Slab` is backed by a `Vec` of slots. Each slot is either occupied or
vacant. `Slab` maintains a stack of vacant slots using a linked list. To
find a vacant slot, the stack is popped. When a slot is released, it is
pushed onto the stack.

If there are no more available slots in the stack, then `Vec::reserve(1)` is
called and a new slot is created.


## Contents

- [Modules](#modules)
  - [`builder`](#builder)
- [Structs](#structs)
  - [`Slab`](#slab)
  - [`VacantEntry`](#vacantentry)
  - [`IntoIter`](#intoiter)
  - [`Iter`](#iter)
  - [`IterMut`](#itermut)
  - [`Drain`](#drain)
- [Enums](#enums)
  - [`GetDisjointMutError`](#getdisjointmuterror)
  - [`Entry`](#entry)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`builder`](#builder) | mod |  |
| [`Slab`](#slab) | struct | Pre-allocated storage for a uniform data type |
| [`VacantEntry`](#vacantentry) | struct | A handle to a vacant entry in a `Slab`. |
| [`IntoIter`](#intoiter) | struct | A consuming iterator over the values stored in a `Slab` |
| [`Iter`](#iter) | struct | An iterator over the values stored in the `Slab` |
| [`IterMut`](#itermut) | struct | A mutable iterator over the values stored in the `Slab` |
| [`Drain`](#drain) | struct | A draining iterator for `Slab` |
| [`GetDisjointMutError`](#getdisjointmuterror) | enum | The error type returned by [`Slab::get_disjoint_mut`]. |
| [`Entry`](#entry) | enum |  |

## Modules

- [`builder`](builder/index.md)

## Structs

### `Slab<T>`

```rust
struct Slab<T> {
    entries: alloc::vec::Vec<Entry<T>>,
    len: usize,
    next: usize,
}
```

Pre-allocated storage for a uniform data type

See the [module documentation] for more details.


#### Implementations

- <span id="slab-new"></span>`const fn new() -> Self`

  Construct a new, empty `Slab`.

  

  The function does not allocate and the returned slab will have no

  capacity until `insert` is called or capacity is explicitly reserved.

  

  # Examples

  

  ```rust

  use slab::*;

  let slab: Slab<i32> = Slab::new();

  ```

- <span id="slab-with-capacity"></span>`fn with_capacity(capacity: usize) -> Slab<T>` — [`Slab`](#slab)

  Construct a new, empty `Slab` with the specified capacity.

  

  The returned slab will be able to store exactly `capacity` without

  reallocating. If `capacity` is 0, the slab will not allocate.

  

  It is important to note that this function does not specify the *length*

  of the returned slab, but only the capacity. For an explanation of the

  difference between length and capacity, see [Capacity and

  reallocation](index.html#capacity-and-reallocation).

  

  # Examples

  

  ```rust

  use slab::*;

  let mut slab = Slab::with_capacity(10);

  

  // The slab contains no values, even though it has capacity for more

  assert_eq!(slab.len(), 0);

  

  // These are all done without reallocating...

  for i in 0..10 {

      slab.insert(i);

  }

  

  // ...but this may make the slab reallocate

  slab.insert(11);

  ```

- <span id="slab-capacity"></span>`fn capacity(&self) -> usize`

  Return the number of values the slab can store without reallocating.

  

  # Examples

  

  ```rust

  use slab::*;

  let slab: Slab<i32> = Slab::with_capacity(10);

  assert_eq!(slab.capacity(), 10);

  ```

- <span id="slab-reserve"></span>`fn reserve(&mut self, additional: usize)`

  Reserve capacity for at least `additional` more values to be stored

  without allocating.

  

  `reserve` does nothing if the slab already has sufficient capacity for

  `additional` more values. If more capacity is required, a new segment of

  memory will be allocated and all existing values will be copied into it.

  As such, if the slab is already very large, a call to `reserve` can end

  up being expensive.

  

  The slab may reserve more than `additional` extra space in order to

  avoid frequent reallocations. Use `reserve_exact` instead to guarantee

  that only the requested space is allocated.

  

  # Panics

  

  Panics if the new capacity exceeds `isize::MAX` bytes.

  

  # Examples

  

  ```rust

  use slab::*;

  let mut slab = Slab::new();

  slab.insert("hello");

  slab.reserve(10);

  assert!(slab.capacity() >= 11);

  ```

- <span id="slab-reserve-exact"></span>`fn reserve_exact(&mut self, additional: usize)`

  Reserve the minimum capacity required to store exactly `additional`

  more values.

  

  `reserve_exact` does nothing if the slab already has sufficient capacity

  for `additional` more values. If more capacity is required, a new segment

  of memory will be allocated and all existing values will be copied into

  it.  As such, if the slab is already very large, a call to `reserve` can

  end up being expensive.

  

  Note that the allocator may give the slab more space than it requests.

  Therefore capacity can not be relied upon to be precisely minimal.

  Prefer `reserve` if future insertions are expected.

  

  # Panics

  

  Panics if the new capacity exceeds `isize::MAX` bytes.

  

  # Examples

  

  ```rust

  use slab::*;

  let mut slab = Slab::new();

  slab.insert("hello");

  slab.reserve_exact(10);

  assert!(slab.capacity() >= 11);

  ```

- <span id="slab-shrink-to-fit"></span>`fn shrink_to_fit(&mut self)`

  Shrink the capacity of the slab as much as possible without invalidating keys.

  

  Because values cannot be moved to a different index, the slab cannot

  shrink past any stored values.

  It will drop down as close as possible to the length but the allocator may

  still inform the underlying vector that there is space for a few more elements.

  

  This function can take O(n) time even when the capacity cannot be reduced

  or the allocation is shrunk in place. Repeated calls run in O(1) though.

  

  # Examples

  

  ```rust

  use slab::*;

  let mut slab = Slab::with_capacity(10);

  

  for i in 0..3 {

      slab.insert(i);

  }

  

  slab.shrink_to_fit();

  assert!(slab.capacity() >= 3 && slab.capacity() < 10);

  ```

  

  The slab cannot shrink past the last present value even if previous

  values are removed:

  

  ```rust

  use slab::*;

  let mut slab = Slab::with_capacity(10);

  

  for i in 0..4 {

      slab.insert(i);

  }

  

  slab.remove(0);

  slab.remove(3);

  

  slab.shrink_to_fit();

  assert!(slab.capacity() >= 3 && slab.capacity() < 10);

  ```

- <span id="slab-recreate-vacant-list"></span>`fn recreate_vacant_list(&mut self)`

  Iterate through all entries to recreate and repair the vacant list.

  self.len must be correct and is not modified.

- <span id="slab-compact"></span>`fn compact<F>(&mut self, rekey: F)`

  Reduce the capacity as much as possible, changing the key for elements when necessary.

  

  To allow updating references to the elements which must be moved to a new key,

  this function takes a closure which is called before moving each element.

  The second and third parameters to the closure are the current key and

  new key respectively.

  In case changing the key for one element turns out not to be possible,

  the move can be cancelled by returning `false` from the closure.

  In that case no further attempts at relocating elements is made.

  If the closure unwinds, the slab will be left in a consistent state,

  but the value that the closure panicked on might be removed.

  

  # Examples

  

  ```rust

  use slab::*;

  

  let mut slab = Slab::with_capacity(10);

  let a = slab.insert('a');

  slab.insert('b');

  slab.insert('c');

  slab.remove(a);

  slab.compact(|&mut value, from, to| {

      assert_eq!((value, from, to), ('c', 2, 0));

      true

  });

  assert!(slab.capacity() >= 2 && slab.capacity() < 10);

  ```

  

  The value is not moved when the closure returns `Err`:

  

  ```rust

  use slab::*;

  

  let mut slab = Slab::with_capacity(100);

  let a = slab.insert('a');

  let b = slab.insert('b');

  slab.remove(a);

  slab.compact(|&mut value, from, to| false);

  assert_eq!(slab.iter().next(), Some((b, &'b')));

  ```

- <span id="slab-clear"></span>`fn clear(&mut self)`

  Clear the slab of all values.

  

  # Examples

  

  ```rust

  use slab::*;

  let mut slab = Slab::new();

  

  for i in 0..3 {

      slab.insert(i);

  }

  

  slab.clear();

  assert!(slab.is_empty());

  ```

- <span id="slab-len"></span>`fn len(&self) -> usize`

  Return the number of stored values.

  

  # Examples

  

  ```rust

  use slab::*;

  let mut slab = Slab::new();

  

  for i in 0..3 {

      slab.insert(i);

  }

  

  assert_eq!(3, slab.len());

  ```

- <span id="slab-is-empty"></span>`fn is_empty(&self) -> bool`

  Return `true` if there are no values stored in the slab.

  

  # Examples

  

  ```rust

  use slab::*;

  let mut slab = Slab::new();

  assert!(slab.is_empty());

  

  slab.insert(1);

  assert!(!slab.is_empty());

  ```

- <span id="slab-iter"></span>`fn iter(&self) -> Iter<'_, T>` — [`Iter`](#iter)

  Return an iterator over the slab.

  

  This function should generally be **avoided** as it is not efficient.

  Iterators must iterate over every slot in the slab even if it is

  vacant. As such, a slab with a capacity of 1 million but only one

  stored value must still iterate the million slots.

  

  # Examples

  

  ```rust

  use slab::*;

  let mut slab = Slab::new();

  

  for i in 0..3 {

      slab.insert(i);

  }

  

  let mut iterator = slab.iter();

  

  assert_eq!(iterator.next(), Some((0, &0)));

  assert_eq!(iterator.next(), Some((1, &1)));

  assert_eq!(iterator.next(), Some((2, &2)));

  assert_eq!(iterator.next(), None);

  ```

- <span id="slab-iter-mut"></span>`fn iter_mut(&mut self) -> IterMut<'_, T>` — [`IterMut`](#itermut)

  Return an iterator that allows modifying each value.

  

  This function should generally be **avoided** as it is not efficient.

  Iterators must iterate over every slot in the slab even if it is

  vacant. As such, a slab with a capacity of 1 million but only one

  stored value must still iterate the million slots.

  

  # Examples

  

  ```rust

  use slab::*;

  let mut slab = Slab::new();

  

  let key1 = slab.insert(0);

  let key2 = slab.insert(1);

  

  for (key, val) in slab.iter_mut() {

      if key == key1 {

          *val += 2;

      }

  }

  

  assert_eq!(slab[key1], 2);

  assert_eq!(slab[key2], 1);

  ```

- <span id="slab-get"></span>`fn get(&self, key: usize) -> Option<&T>`

  Return a reference to the value associated with the given key.

  

  If the given key is not associated with a value, then `None` is

  returned.

  

  # Examples

  

  ```rust

  use slab::*;

  let mut slab = Slab::new();

  let key = slab.insert("hello");

  

  assert_eq!(slab.get(key), Some(&"hello"));

  assert_eq!(slab.get(123), None);

  ```

- <span id="slab-get-mut"></span>`fn get_mut(&mut self, key: usize) -> Option<&mut T>`

  Return a mutable reference to the value associated with the given key.

  

  If the given key is not associated with a value, then `None` is

  returned.

  

  # Examples

  

  ```rust

  use slab::*;

  let mut slab = Slab::new();

  let key = slab.insert("hello");

  

  *slab.get_mut(key).unwrap() = "world";

  

  assert_eq!(slab[key], "world");

  assert_eq!(slab.get_mut(123), None);

  ```

- <span id="slab-get2-mut"></span>`fn get2_mut(&mut self, key1: usize, key2: usize) -> Option<(&mut T, &mut T)>`

  Return two mutable references to the values associated with the two

  given keys simultaneously.

  

  If any one of the given keys is not associated with a value, then `None`

  is returned.

  

  This function can be used to get two mutable references out of one slab,

  so that you can manipulate both of them at the same time, eg. swap them.

  

  # Panics

  

  This function will panic if `key1` and `key2` are the same.

  

  # Examples

  

  ```rust

  use slab::*;

  use std::mem;

  

  let mut slab = Slab::new();

  let key1 = slab.insert(1);

  let key2 = slab.insert(2);

  let (value1, value2) = slab.get2_mut(key1, key2).unwrap();

  mem::swap(value1, value2);

  assert_eq!(slab[key1], 2);

  assert_eq!(slab[key2], 1);

  ```

- <span id="slab-get-disjoint-mut"></span>`fn get_disjoint_mut<const N: usize>(&mut self, keys: [usize; N]) -> Result<[&mut T; N], GetDisjointMutError>` — [`GetDisjointMutError`](#getdisjointmuterror)

  Returns mutable references to many indices at once.

  

  Returns [`GetDisjointMutError`](#getdisjointmuterror) if the indices are out of bounds,

  overlapping, or vacant.

- <span id="slab-get-unchecked"></span>`unsafe fn get_unchecked(&self, key: usize) -> &T`

  Return a reference to the value associated with the given key without

  performing bounds checking.

  

  For a safe alternative see [`get`](Slab::get).

  

  This function should be used with care.

  

  # Safety

  

  The key must be within bounds.

  

  # Examples

  

  ```rust

  use slab::*;

  let mut slab = Slab::new();

  let key = slab.insert(2);

  

  unsafe {

      assert_eq!(slab.get_unchecked(key), &2);

  }

  ```

- <span id="slab-get-unchecked-mut"></span>`unsafe fn get_unchecked_mut(&mut self, key: usize) -> &mut T`

  Return a mutable reference to the value associated with the given key

  without performing bounds checking.

  

  For a safe alternative see [`get_mut`](Slab::get_mut).

  

  This function should be used with care.

  

  # Safety

  

  The key must be within bounds.

  

  # Examples

  

  ```rust

  use slab::*;

  let mut slab = Slab::new();

  let key = slab.insert(2);

  

  unsafe {

      let val = slab.get_unchecked_mut(key);

      *val = 13;

  }

  

  assert_eq!(slab[key], 13);

  ```

- <span id="slab-get2-unchecked-mut"></span>`unsafe fn get2_unchecked_mut(&mut self, key1: usize, key2: usize) -> (&mut T, &mut T)`

  Return two mutable references to the values associated with the two

  given keys simultaneously without performing bounds checking and safety

  condition checking.

  

  For a safe alternative see [`get2_mut`](Slab::get2_mut).

  

  This function should be used with care.

  

  # Safety

  

  - Both keys must be within bounds.

  - The condition `key1 != key2` must hold.

  

  # Examples

  

  ```rust

  use slab::*;

  use std::mem;

  

  let mut slab = Slab::new();

  let key1 = slab.insert(1);

  let key2 = slab.insert(2);

  let (value1, value2) = unsafe { slab.get2_unchecked_mut(key1, key2) };

  mem::swap(value1, value2);

  assert_eq!(slab[key1], 2);

  assert_eq!(slab[key2], 1);

  ```

- <span id="slab-key-of"></span>`fn key_of(&self, present_element: &T) -> usize`

  Get the key for an element in the slab.

  

  The reference must point to an element owned by the slab.

  Otherwise this function will panic.

  This is a constant-time operation because the key can be calculated

  from the reference with pointer arithmetic.

  

  # Panics

  

  This function will panic if the reference does not point to an element

  of the slab.

  

  # Examples

  

  ```rust

  use slab::*;

  

  let mut slab = Slab::new();

  let key = slab.insert(String::from("foo"));

  let value = &slab[key];

  assert_eq!(slab.key_of(value), key);

  ```

  

  Values are not compared, so passing a reference to a different location

  will result in a panic:

  

  ```should_panic

  use slab::*;

  

  let mut slab = Slab::new();

  let key = slab.insert(0);

  let bad = &0;

  slab.key_of(bad); // this will panic

  unreachable!();

  ```

- <span id="slab-insert"></span>`fn insert(&mut self, val: T) -> usize`

  Insert a value in the slab, returning key assigned to the value.

  

  The returned key can later be used to retrieve or remove the value using indexed

  lookup and `remove`. Additional capacity is allocated if needed. See

  [Capacity and reallocation](index.html#capacity-and-reallocation).

  

  # Panics

  

  Panics if the new storage in the vector exceeds `isize::MAX` bytes.

  

  # Examples

  

  ```rust

  use slab::*;

  let mut slab = Slab::new();

  let key = slab.insert("hello");

  assert_eq!(slab[key], "hello");

  ```

- <span id="slab-vacant-key"></span>`fn vacant_key(&self) -> usize`

  Returns the key of the next vacant entry.

  

  This function returns the key of the vacant entry which  will be used

  for the next insertion. This is equivalent to

  `slab.vacant_entry().key()`, but it doesn't require mutable access.

  

  # Examples

  

  ```rust

  use slab::*;

  let mut slab = Slab::new();

  assert_eq!(slab.vacant_key(), 0);

  

  slab.insert(0);

  assert_eq!(slab.vacant_key(), 1);

  

  slab.insert(1);

  slab.remove(0);

  assert_eq!(slab.vacant_key(), 0);

  ```

- <span id="slab-vacant-entry"></span>`fn vacant_entry(&mut self) -> VacantEntry<'_, T>` — [`VacantEntry`](#vacantentry)

  Return a handle to a vacant entry allowing for further manipulation.

  

  This function is useful when creating values that must contain their

  slab key. The returned `VacantEntry` reserves a slot in the slab and is

  able to query the associated key.

  

  # Examples

  

  ```rust

  use slab::*;

  let mut slab = Slab::new();

  

  let hello = {

      let entry = slab.vacant_entry();

      let key = entry.key();

  

      entry.insert((key, "hello"));

      key

  };

  

  assert_eq!(hello, slab[hello].0);

  assert_eq!("hello", slab[hello].1);

  ```

- <span id="slab-insert-at"></span>`fn insert_at(&mut self, key: usize, val: T)`

- <span id="slab-try-remove"></span>`fn try_remove(&mut self, key: usize) -> Option<T>`

  Tries to remove the value associated with the given key,

  returning the value if the key existed.

  

  The key is then released and may be associated with future stored

  values.

  

  # Examples

  

  ```rust

  use slab::*;

  let mut slab = Slab::new();

  

  let hello = slab.insert("hello");

  

  assert_eq!(slab.try_remove(hello), Some("hello"));

  assert!(!slab.contains(hello));

  ```

- <span id="slab-remove"></span>`fn remove(&mut self, key: usize) -> T`

  Remove and return the value associated with the given key.

  

  The key is then released and may be associated with future stored

  values.

  

  # Panics

  

  Panics if `key` is not associated with a value.

  

  # Examples

  

  ```rust

  use slab::*;

  let mut slab = Slab::new();

  

  let hello = slab.insert("hello");

  

  assert_eq!(slab.remove(hello), "hello");

  assert!(!slab.contains(hello));

  ```

- <span id="slab-contains"></span>`fn contains(&self, key: usize) -> bool`

  Return `true` if a value is associated with the given key.

  

  # Examples

  

  ```rust

  use slab::*;

  let mut slab = Slab::new();

  

  let hello = slab.insert("hello");

  assert!(slab.contains(hello));

  

  slab.remove(hello);

  

  assert!(!slab.contains(hello));

  ```

- <span id="slab-retain"></span>`fn retain<F>(&mut self, f: F)`

  Retain only the elements specified by the predicate.

  

  In other words, remove all elements `e` such that `f(usize, &mut e)`

  returns false. This method operates in place and preserves the key

  associated with the retained values.

  

  # Examples

  

  ```rust

  use slab::*;

  let mut slab = Slab::new();

  

  let k1 = slab.insert(0);

  let k2 = slab.insert(1);

  let k3 = slab.insert(2);

  

  slab.retain(|key, val| key == k1 || *val == 1);

  

  assert!(slab.contains(k1));

  assert!(slab.contains(k2));

  assert!(!slab.contains(k3));

  

  assert_eq!(2, slab.len());

  ```

- <span id="slab-drain"></span>`fn drain(&mut self) -> Drain<'_, T>` — [`Drain`](#drain)

  Return a draining iterator that removes all elements from the slab and

  yields the removed items.

  

  Note: Elements are removed even if the iterator is only partially

  consumed or not consumed at all.

  

  # Examples

  

  ```rust

  use slab::*;

  let mut slab = Slab::new();

  

  let _ = slab.insert(0);

  let _ = slab.insert(1);

  let _ = slab.insert(2);

  

  {

      let mut drain = slab.drain();

  

      assert_eq!(Some(0), drain.next());

      assert_eq!(Some(1), drain.next());

      assert_eq!(Some(2), drain.next());

      assert_eq!(None, drain.next());

  }

  

  assert!(slab.is_empty());

  ```

#### Trait Implementations

##### `impl<T> Clone for Slab<T>`

- <span id="slab-clone"></span>`fn clone(&self) -> Self`

- <span id="slab-clone-clone-from"></span>`fn clone_from(&mut self, source: &Self)`

##### `impl<T> Debug for Slab<T>`

- <span id="slab-debug-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> Default for Slab<T>`

- <span id="slab-default"></span>`fn default() -> Self`

##### `impl<T> FromIterator for Slab<T>`

- <span id="slab-fromiterator-from-iter"></span>`fn from_iter<I>(iterable: I) -> Self`

##### `impl<T> Index for Slab<T>`

- <span id="slab-index-type-output"></span>`type Output = T`

- <span id="slab-index"></span>`fn index(&self, key: usize) -> &T`

##### `impl<T> IndexMut for Slab<T>`

- <span id="slab-indexmut-index-mut"></span>`fn index_mut(&mut self, key: usize) -> &mut T`

##### `impl<T> IntoIterator for Slab<T>`

- <span id="slab-intoiterator-type-item"></span>`type Item = (usize, T)`

- <span id="slab-intoiterator-type-intoiter"></span>`type IntoIter = IntoIter<T>`

- <span id="slab-intoiterator-into-iter"></span>`fn into_iter(self) -> IntoIter<T>` — [`IntoIter`](#intoiter)

### `VacantEntry<'a, T>`

```rust
struct VacantEntry<'a, T> {
    slab: &'a mut Slab<T>,
    key: usize,
}
```

A handle to a vacant entry in a `Slab`.

`VacantEntry` allows constructing values with the key that they will be
assigned to.

# Examples

```rust
use slab::*;
let mut slab = Slab::new();

let hello = {
    let entry = slab.vacant_entry();
    let key = entry.key();

    entry.insert((key, "hello"));
    key
};

assert_eq!(hello, slab[hello].0);
assert_eq!("hello", slab[hello].1);
```

#### Implementations

- <span id="vacantentry-insert"></span>`fn insert(self, val: T) -> &'a mut T`

  Insert a value in the entry, returning a mutable reference to the value.

  

  To get the key associated with the value, use `key` prior to calling

  `insert`.

  

  # Examples

  

  ```rust

  use slab::*;

  let mut slab = Slab::new();

  

  let hello = {

      let entry = slab.vacant_entry();

      let key = entry.key();

  

      entry.insert((key, "hello"));

      key

  };

  

  assert_eq!(hello, slab[hello].0);

  assert_eq!("hello", slab[hello].1);

  ```

- <span id="vacantentry-key"></span>`fn key(&self) -> usize`

  Return the key associated with this entry.

  

  A value stored in this entry will be associated with this key.

  

  # Examples

  

  ```rust

  use slab::*;

  let mut slab = Slab::new();

  

  let hello = {

      let entry = slab.vacant_entry();

      let key = entry.key();

  

      entry.insert((key, "hello"));

      key

  };

  

  assert_eq!(hello, slab[hello].0);

  assert_eq!("hello", slab[hello].1);

  ```

#### Trait Implementations

##### `impl<T: fmt::Debug> Debug for VacantEntry<'a, T>`

- <span id="vacantentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `IntoIter<T>`

```rust
struct IntoIter<T> {
    entries: iter::Enumerate<vec::IntoIter<Entry<T>>>,
    len: usize,
}
```

A consuming iterator over the values stored in a `Slab`

#### Trait Implementations

##### `impl<T> Debug for IntoIter<T>`

- <span id="intoiter-debug-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> DoubleEndedIterator for IntoIter<T>`

- <span id="intoiter-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl<T> ExactSizeIterator for IntoIter<T>`

- <span id="intoiter-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl<T> FusedIterator for IntoIter<T>`

##### `impl IntoIterator for IntoIter<T>`

- <span id="intoiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="intoiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="intoiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T> Iterator for IntoIter<T>`

- <span id="intoiter-iterator-type-item"></span>`type Item = (usize, T)`

- <span id="intoiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="intoiter-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `Iter<'a, T>`

```rust
struct Iter<'a, T> {
    entries: iter::Enumerate<slice::Iter<'a, Entry<T>>>,
    len: usize,
}
```

An iterator over the values stored in the `Slab`

#### Trait Implementations

##### `impl<T> Clone for Iter<'_, T>`

- <span id="iter-clone"></span>`fn clone(&self) -> Self`

##### `impl<T> Debug for Iter<'_, T>`

- <span id="iter-debug-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> DoubleEndedIterator for Iter<'_, T>`

- <span id="iter-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl<T> ExactSizeIterator for Iter<'_, T>`

- <span id="iter-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl<T> FusedIterator for Iter<'_, T>`

##### `impl IntoIterator for Iter<'a, T>`

- <span id="iter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="iter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="iter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T> Iterator for Iter<'a, T>`

- <span id="iter-iterator-type-item"></span>`type Item = (usize, &'a T)`

- <span id="iter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="iter-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `IterMut<'a, T>`

```rust
struct IterMut<'a, T> {
    entries: iter::Enumerate<slice::IterMut<'a, Entry<T>>>,
    len: usize,
}
```

A mutable iterator over the values stored in the `Slab`

#### Trait Implementations

##### `impl<T> Debug for IterMut<'_, T>`

- <span id="itermut-debug-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> DoubleEndedIterator for IterMut<'_, T>`

- <span id="itermut-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl<T> ExactSizeIterator for IterMut<'_, T>`

- <span id="itermut-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl<T> FusedIterator for IterMut<'_, T>`

##### `impl IntoIterator for IterMut<'a, T>`

- <span id="itermut-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="itermut-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="itermut-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T> Iterator for IterMut<'a, T>`

- <span id="itermut-iterator-type-item"></span>`type Item = (usize, &'a mut T)`

- <span id="itermut-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="itermut-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `Drain<'a, T>`

```rust
struct Drain<'a, T> {
    inner: vec::Drain<'a, Entry<T>>,
    len: usize,
}
```

A draining iterator for `Slab`

#### Trait Implementations

##### `impl<T> Debug for Drain<'_, T>`

- <span id="drain-debug-fmt"></span>`fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T> DoubleEndedIterator for Drain<'_, T>`

- <span id="drain-doubleendediterator-next-back"></span>`fn next_back(&mut self) -> Option<<Self as >::Item>`

##### `impl<T> ExactSizeIterator for Drain<'_, T>`

- <span id="drain-exactsizeiterator-len"></span>`fn len(&self) -> usize`

##### `impl<T> FusedIterator for Drain<'_, T>`

##### `impl IntoIterator for Drain<'a, T>`

- <span id="drain-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="drain-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="drain-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T> Iterator for Drain<'_, T>`

- <span id="drain-iterator-type-item"></span>`type Item = T`

- <span id="drain-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="drain-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

## Enums

### `GetDisjointMutError`

```rust
enum GetDisjointMutError {
    IndexVacant,
    IndexOutOfBounds,
    OverlappingIndices,
}
```

The error type returned by `Slab::get_disjoint_mut`.

#### Variants

- **`IndexVacant`**

  An index provided was not associated with a value.

- **`IndexOutOfBounds`**

  An index provided was out-of-bounds for the slab.

- **`OverlappingIndices`**

  Two indices provided were overlapping.

#### Trait Implementations

##### `impl Clone for GetDisjointMutError`

- <span id="getdisjointmuterror-clone"></span>`fn clone(&self) -> GetDisjointMutError` — [`GetDisjointMutError`](#getdisjointmuterror)

##### `impl Debug for GetDisjointMutError`

- <span id="getdisjointmuterror-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for GetDisjointMutError`

- <span id="getdisjointmuterror-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for GetDisjointMutError`

##### `impl PartialEq for GetDisjointMutError`

- <span id="getdisjointmuterror-partialeq-eq"></span>`fn eq(&self, other: &GetDisjointMutError) -> bool` — [`GetDisjointMutError`](#getdisjointmuterror)

##### `impl StructuralPartialEq for GetDisjointMutError`

##### `impl ToString for GetDisjointMutError`

- <span id="getdisjointmuterror-tostring-to-string"></span>`fn to_string(&self) -> String`

### `Entry<T>`

```rust
enum Entry<T> {
    Vacant(usize),
    Occupied(T),
}
```

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for Entry<T>`

- <span id="entry-clone"></span>`fn clone(&self) -> Entry<T>` — [`Entry`](#entry)

