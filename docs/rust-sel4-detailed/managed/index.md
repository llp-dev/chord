# Crate `managed`

A library that provides a way to logically own objects, whether or not
heap allocation is available.

## Contents

- [Modules](#modules)
  - [`object`](#object)
  - [`slice`](#slice)
  - [`slotmap`](#slotmap)
  - [`map`](#map)
- [Structs](#structs)
  - [`SlotKey`](#slotkey)
  - [`SlotIndex`](#slotindex)
  - [`SlotMap`](#slotmap)
- [Enums](#enums)
  - [`Managed`](#managed)
  - [`ManagedSlice`](#managedslice)
  - [`ManagedMap`](#managedmap)
  - [`ManagedMapIter`](#managedmapiter)
  - [`ManagedMapIterMut`](#managedmapitermut)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`object`](#object) | mod |  |
| [`slice`](#slice) | mod |  |
| [`slotmap`](#slotmap) | mod | A slotmap, a vector-like container with unique keys instead of indices. |
| [`map`](#map) | mod |  |
| [`SlotKey`](#slotkey) | struct |  |
| [`SlotIndex`](#slotindex) | struct |  |
| [`SlotMap`](#slotmap) | struct |  |
| [`Managed`](#managed) | enum |  |
| [`ManagedSlice`](#managedslice) | enum |  |
| [`ManagedMap`](#managedmap) | enum |  |
| [`ManagedMapIter`](#managedmapiter) | enum |  |
| [`ManagedMapIterMut`](#managedmapitermut) | enum |  |

## Modules

- [`object`](object/index.md)
- [`slice`](slice/index.md)
- [`slotmap`](slotmap/index.md) — A slotmap, a vector-like container with unique keys instead of indices.
- [`map`](map/index.md)

## Structs

### `SlotKey`

```rust
struct SlotKey {
    idx: usize,
    generation: Generation,
}
```

An index into a slotmap.

The index remains valid until the entry is removed. If accessing the slotmap with the index
again after the entry was removed will fail, even if the index where the element was previously
stored has been reused for another element.

#### Trait Implementations

##### `impl Clone for Key`

- <span id="key-clone"></span>`fn clone(&self) -> Key` — [`Key`](slotmap/index.md#key)

##### `impl Copy for Key`

##### `impl Debug for Key`

- <span id="key-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Key`

- <span id="key-default"></span>`fn default() -> Key` — [`Key`](slotmap/index.md#key)

##### `impl Eq for Key`

##### `impl Hash for Key`

- <span id="key-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Key`

- <span id="key-partialeq-eq"></span>`fn eq(&self, other: &Key) -> bool` — [`Key`](slotmap/index.md#key)

##### `impl StructuralPartialEq for Key`

### `SlotIndex`

```rust
struct SlotIndex {
    generation_id: GenerationOrFreelink,
}
```

Provides links between slots and elements.

The benefit of separating this struct from the elements is that it is unconditionally `Copy`
and `Default`. It also provides better locality for both the indices and the elements which
could help with iteration or very large structs.

#### Fields

- **`generation_id`**: `GenerationOrFreelink`

  The id of this slot.
  
  If the given out index mismatches the `generation_id` then the element was removed already
  and we can return `None` on lookup.
  
  If the slot is currently unused we will instead provide the index to the previous slot in
  the slot-free-list.

#### Trait Implementations

##### `impl Clone for Slot`

- <span id="slot-clone"></span>`fn clone(&self) -> Slot` — [`Slot`](slotmap/index.md#slot)

##### `impl Copy for Slot`

##### `impl Debug for Slot`

- <span id="slot-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Slot`

- <span id="slot-default"></span>`fn default() -> Slot` — [`Slot`](slotmap/index.md#slot)

##### `impl Eq for Slot`

##### `impl Hash for Slot`

- <span id="slot-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Slot`

- <span id="slot-partialeq-eq"></span>`fn eq(&self, other: &Slot) -> bool` — [`Slot`](slotmap/index.md#slot)

##### `impl StructuralPartialEq for Slot`

### `SlotMap<'a, T>`

```rust
struct SlotMap<'a, T> {
    elements: super::ManagedSlice<'a, T>,
    slots: Partial<'a, Slot>,
    generation: Generation,
    free_top: usize,
    indices: IndexComputer,
}
```

Provides a slotmap based on external memory.

A slotmap provides a `Vec`-like interface where each entry is associated with a stable
index-like key. Lookup with the key will detect if an entry has been removed but does not
require a lifetime relation. Compared to other slotmap implementations this does not internally
allocate any memory on its own but only relies on the [`Slice`](#slice) arguments in the constructor.

## Usage

The important aspect is that the slotmap does not create the storage of its own elements, it
merely manages one given to it at construction time.

```rust
use managed::{ManagedSlice, SlotMap, SlotIndex};

let mut elements = [0usize; 1024];
let mut slots = [SlotIndex::default(); 1024];

let mut map = SlotMap::new(
    ManagedSlice::Borrowed(&mut elements[..]),
    ManagedSlice::Borrowed(&mut slots[..]));
let index = map.insert(42).unwrap();
assert_eq!(map.get(index).cloned(), Some(42));
```

#### Fields

- **`elements`**: `super::ManagedSlice<'a, T>`

  The slice where elements are placed.
  All of them are initialized at all times but not all are logically part of the map.

- **`slots`**: `Partial<'a, Slot>`

  The logical list of used slots.
  Note that a slot is never remove from this list but instead used to track the generation_id
  and the link in the free list.

- **`generation`**: `Generation`

  The source of generation ids.
  Generation ids are a positive, non-zero value.

- **`free_top`**: `usize`

  An index to the top element of the free list.
  Refers to the one-past-the-end index of `slots` if there are no elements.

- **`indices`**: `IndexComputer`

  An abstraction around computing wrapped indices in the free list.

#### Implementations

- <span id="slotmap-get"></span>`fn get(&self, index: Key) -> Option<&T>` — [`Key`](slotmap/index.md#key)

  Retrieve a value by index.

- <span id="slotmap-get-mut"></span>`fn get_mut(&mut self, index: Key) -> Option<&mut T>` — [`Key`](slotmap/index.md#key)

  Retrieve a mutable value by index.

- <span id="slotmap-reserve"></span>`fn reserve(&mut self) -> Option<(Key, &mut T)>` — [`Key`](slotmap/index.md#key)

  Reserve a new entry.

  

  In case of success, the returned key refers to the entry until it is removed. The entry

  itself is not initialized with any particular value but instead retains the value it had in

  the backing slice. It is only logically placed into the slot map.

- <span id="slotmap-insert"></span>`fn insert(&mut self, value: T) -> Option<Key>` — [`Key`](slotmap/index.md#key)

  Try to insert a value into the map.

  

  This will fail if there is not enough space. Sugar wrapper around `reserve` for inserting

  values. Note that on success, an old value stored in the backing slice will be overwritten.

  Use `reserve` directly if it is vital that no old value is dropped.

- <span id="slotmap-remove"></span>`fn remove(&mut self, index: Key) -> Option<&mut T>` — [`Key`](slotmap/index.md#key)

  Remove an element.

  

  If successful, return a mutable reference to the removed element so that the caller can

  swap it with a logically empty value. Returns `None` if the provided index did not refer to

  an element that could be freed.

- <span id="slotmap-next-free-slot"></span>`fn next_free_slot(&mut self) -> Option<FreeIndex>` — [`FreeIndex`](slotmap/index.md#freeindex)

  Get the next free slot.

## Enums

### `Managed<'a, T: 'a + ?Sized>`

```rust
enum Managed<'a, T: 'a + ?Sized> {
    Borrowed(&'a mut T),
    Owned(alloc::boxed::Box<T>),
}
```

A managed object.

This enum can be used to represent exclusive access to objects. In Rust, exclusive access
to an object is obtained by either owning the object, or owning a mutable pointer
to the object; hence, "managed".

The purpose of this enum is providing good ergonomics with `std` present while making
it possible to avoid having a heap at all (which of course means that `std` is not present).
To achieve this, the variants other than `Borrow` are only available when the corresponding
feature is opted in.

A function that requires a managed object should be generic over an `Into<Managed<'a, T>>`
argument; then, it will be possible to pass either a `Box<T>`, `Vec<T>`, or a `&'a mut T`
without any conversion at the call site.

Note that a `Vec<T>` converted into an `Into<Managed<'a, [T]>>` gets transformed
into a boxed slice, and can no longer be resized. See also
[ManagedSlice](#managedslice), which does not have this drawback.

#### Variants

- **`Borrowed`**

  Borrowed variant.

- **`Owned`**

  Owned variant, only available with the `std` or `alloc` feature enabled.

#### Trait Implementations

##### `impl<T> Debug for Managed<'a, T>`

- <span id="managed-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: 'a + ?Sized> Deref for Managed<'a, T>`

- <span id="managed-deref-type-target"></span>`type Target = T`

- <span id="managed-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl<T: 'a + ?Sized> DerefMut for Managed<'a, T>`

- <span id="managed-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl<T> Receiver for Managed<'a, T>`

- <span id="managed-receiver-type-target"></span>`type Target = T`

### `ManagedSlice<'a, T: 'a>`

```rust
enum ManagedSlice<'a, T: 'a> {
    Borrowed(&'a mut [T]),
    Owned(alloc::vec::Vec<T>),
}
```

A managed slice.

This enum can be used to represent exclusive access to slices of objects.
In Rust, exclusive access to an object is obtained by either owning the object,
or owning a mutable pointer to the object; hence, "managed".

The purpose of this enum is providing good ergonomics with `std` present while making
it possible to avoid having a heap at all (which of course means that `std` is not present).
To achieve this, the variants other than `Borrowed` are only available when the corresponding
feature is opted in.

A function that requires a managed object should be generic over an `Into<ManagedSlice<'a, T>>`
argument; then, it will be possible to pass either a `Vec<T>`, or a `&'a mut [T]`
without any conversion at the call site.

See also [Managed](#managed).

#### Variants

- **`Borrowed`**

  Borrowed variant.

- **`Owned`**

  Owned variant, only available with the `std` or `alloc` feature enabled.

#### Trait Implementations

##### `impl<T> Debug for ManagedSlice<'a, T>`

- <span id="managedslice-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: 'a> Deref for ManagedSlice<'a, T>`

- <span id="managedslice-deref-type-target"></span>`type Target = [T]`

- <span id="managedslice-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl<T: 'a> DerefMut for ManagedSlice<'a, T>`

- <span id="managedslice-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl<T> Receiver for ManagedSlice<'a, T>`

- <span id="managedslice-receiver-type-target"></span>`type Target = T`

### `ManagedMap<'a, K: 'a, V: 'a>`

```rust
enum ManagedMap<'a, K: 'a, V: 'a> {
    Borrowed(&'a mut [Option<(K, V)>]),
    Owned(alloc::collections::btree_map::BTreeMap<K, V>),
}
```

A managed map.

This enum can be used to represent exclusive access to maps.
In Rust, exclusive access to an object is obtained by either owning the object,
or owning a mutable pointer to the object; hence, "managed".

The purpose of this enum is providing good ergonomics with `std` present while making
it possible to avoid having a heap at all (which of course means that `std` is not present).
To achieve this, the variants other than `Borrow` are only available when the corresponding
feature is opted in.

Unlike [Managed](#managed) and [ManagedSlice](#managedslice),
the managed map is implemented using a B-tree map when allocation is available,
and a sorted slice of key-value pairs when it is not. Thus, algorithmic complexity
of operations on it depends on the kind of map.

A function that requires a managed object should be generic over an `Into<ManagedMap<'a, T>>`
argument; then, it will be possible to pass either a `Vec<T>`, or a `&'a mut [T]`
without any conversion at the call site.

See also [Managed](#managed).

#### Variants

- **`Borrowed`**

  Borrowed variant.

- **`Owned`**

  Owned variant, only available with the `std` or `alloc` feature enabled.

#### Implementations

- <span id="managedmap-clear"></span>`fn clear(&mut self)`

- <span id="managedmap-get"></span>`fn get<Q>(&self, key: &Q) -> Option<&V>`

- <span id="managedmap-get-mut"></span>`fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>`

- <span id="managedmap-range"></span>`fn range<'b, 'c, Q, R>(self: &'b Self, range: R) -> Range<'a, K, V>` — [`Range`](map/index.md#range)

- <span id="managedmap-insert"></span>`fn insert(&mut self, key: K, new_value: V) -> Result<Option<V>, (K, V)>`

- <span id="managedmap-remove"></span>`fn remove<Q>(&mut self, key: &Q) -> Option<V>`

- <span id="managedmap-is-empty"></span>`fn is_empty(&self) -> bool`

  ManagedMap contains no elements?

- <span id="managedmap-len"></span>`fn len(&self) -> usize`

  Returns the number of elements in the ManagedMap.

- <span id="managedmap-iter"></span>`fn iter(&self) -> Iter<'_, K, V>` — [`Iter`](map/index.md#iter)

- <span id="managedmap-iter-mut"></span>`fn iter_mut(&mut self) -> IterMut<'_, K, V>` — [`IterMut`](map/index.md#itermut)

#### Trait Implementations

##### `impl<K, V> Debug for ManagedMap<'a, K, V>`

- <span id="managedmap-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ManagedMapIter<'a, K: 'a, V: 'a>`

```rust
enum ManagedMapIter<'a, K: 'a, V: 'a> {
    Borrowed(slice::Iter<'a, Option<(K, V)>>),
    Owned(alloc::collections::btree_map::Iter<'a, K, V>),
}
```

#### Variants

- **`Borrowed`**

  Borrowed variant.

- **`Owned`**

  Owned variant, only available with the `std` or `alloc` feature enabled.

#### Trait Implementations

##### `impl IntoIterator for Iter<'a, K, V>`

- <span id="iter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="iter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="iter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<K: Ord + 'a, V: 'a> Iterator for Iter<'a, K, V>`

- <span id="iter-iterator-type-item"></span>`type Item = (&'a K, &'a V)`

- <span id="iter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="iter-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

### `ManagedMapIterMut<'a, K: 'a, V: 'a>`

```rust
enum ManagedMapIterMut<'a, K: 'a, V: 'a> {
    Borrowed(slice::IterMut<'a, Option<(K, V)>>),
    Owned(alloc::collections::btree_map::IterMut<'a, K, V>),
}
```

#### Variants

- **`Borrowed`**

  Borrowed variant.

- **`Owned`**

  Owned variant, only available with the `std` or `alloc` feature enabled.

#### Trait Implementations

##### `impl IntoIterator for IterMut<'a, K, V>`

- <span id="itermut-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="itermut-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="itermut-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<K: Ord + 'a, V: 'a> Iterator for IterMut<'a, K, V>`

- <span id="itermut-iterator-type-item"></span>`type Item = (&'a K, &'a mut V)`

- <span id="itermut-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

- <span id="itermut-iterator-size-hint"></span>`fn size_hint(&self) -> (usize, Option<usize>)`

