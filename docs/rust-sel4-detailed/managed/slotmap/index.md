*[managed](../index.md) / [slotmap](index.md)*

---

# Module `slotmap`

A slotmap, a vector-like container with unique keys instead of indices.

See the documentation of [`SlotMap`](#slotmap) for details.


## Contents

- [Structs](#structs)
  - [`Slot`](#slot)
  - [`SlotMap`](#slotmap)
  - [`Partial`](#partial)
  - [`Key`](#key)
  - [`GenerationOrFreelink`](#generationorfreelink)
  - [`FreeIndex`](#freeindex)
  - [`Generation`](#generation)
  - [`Offset`](#offset)
  - [`IndexComputer`](#indexcomputer)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Slot`](#slot) | struct | Provides links between slots and elements. |
| [`SlotMap`](#slotmap) | struct | Provides a slotmap based on external memory. |
| [`Partial`](#partial) | struct | A backing slice tracking an index how far it is logically initialized. |
| [`Key`](#key) | struct | An index into a slotmap. |
| [`GenerationOrFreelink`](#generationorfreelink) | struct |  |
| [`FreeIndex`](#freeindex) | struct | Newtype wrapper around the index of a free slot. |
| [`Generation`](#generation) | struct | The generation counter. |
| [`Offset`](#offset) | struct | Offset of a freelist entry to the next entry. |
| [`IndexComputer`](#indexcomputer) | struct | Links FreeIndex and Offset. |

## Structs

### `Slot`

```rust
struct Slot {
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

- <span id="slot-clone"></span>`fn clone(&self) -> Slot` — [`Slot`](#slot)

##### `impl Copy for Slot`

##### `impl Debug for Slot`

- <span id="slot-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Slot`

- <span id="slot-default"></span>`fn default() -> Slot` — [`Slot`](#slot)

##### `impl Eq for Slot`

##### `impl Hash for Slot`

- <span id="slot-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Slot`

- <span id="slot-partialeq-eq"></span>`fn eq(&self, other: &Slot) -> bool` — [`Slot`](#slot)

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

- <span id="slotmap-get"></span>`fn get(&self, index: Key) -> Option<&T>` — [`Key`](#key)

  Retrieve a value by index.

- <span id="slotmap-get-mut"></span>`fn get_mut(&mut self, index: Key) -> Option<&mut T>` — [`Key`](#key)

  Retrieve a mutable value by index.

- <span id="slotmap-reserve"></span>`fn reserve(&mut self) -> Option<(Key, &mut T)>` — [`Key`](#key)

  Reserve a new entry.

  

  In case of success, the returned key refers to the entry until it is removed. The entry

  itself is not initialized with any particular value but instead retains the value it had in

  the backing slice. It is only logically placed into the slot map.

- <span id="slotmap-insert"></span>`fn insert(&mut self, value: T) -> Option<Key>` — [`Key`](#key)

  Try to insert a value into the map.

  

  This will fail if there is not enough space. Sugar wrapper around `reserve` for inserting

  values. Note that on success, an old value stored in the backing slice will be overwritten.

  Use `reserve` directly if it is vital that no old value is dropped.

- <span id="slotmap-remove"></span>`fn remove(&mut self, index: Key) -> Option<&mut T>` — [`Key`](#key)

  Remove an element.

  

  If successful, return a mutable reference to the removed element so that the caller can

  swap it with a logically empty value. Returns `None` if the provided index did not refer to

  an element that could be freed.

- <span id="slotmap-next-free-slot"></span>`fn next_free_slot(&mut self) -> Option<FreeIndex>` — [`FreeIndex`](#freeindex)

  Get the next free slot.

### `Partial<'a, T>`

```rust
struct Partial<'a, T> {
    slice: super::ManagedSlice<'a, T>,
    next_idx: usize,
}
```

A backing slice tracking an index how far it is logically initialized.

#### Implementations

- <span id="partial-get"></span>`fn get(&self, idx: usize) -> Option<&T>`

- <span id="partial-get-mut"></span>`fn get_mut(&mut self, idx: usize) -> Option<&mut T>`

- <span id="partial-len"></span>`fn len(&self) -> usize`

- <span id="partial-try-reserve"></span>`fn try_reserve(&mut self) -> Option<&mut T>`

### `Key`

```rust
struct Key {
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

- <span id="key-clone"></span>`fn clone(&self) -> Key` — [`Key`](#key)

##### `impl Copy for Key`

##### `impl Debug for Key`

- <span id="key-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Key`

- <span id="key-default"></span>`fn default() -> Key` — [`Key`](#key)

##### `impl Eq for Key`

##### `impl Hash for Key`

- <span id="key-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Key`

- <span id="key-partialeq-eq"></span>`fn eq(&self, other: &Key) -> bool` — [`Key`](#key)

##### `impl StructuralPartialEq for Key`

### `GenerationOrFreelink`

```rust
struct GenerationOrFreelink(isize);
```

#### Implementations

- <span id="generationorfreelink-free-link"></span>`fn free_link(self) -> Result<Offset, Generation>` — [`Offset`](#offset), [`Generation`](#generation)

- <span id="generationorfreelink-generation"></span>`fn generation(self) -> Result<Generation, Offset>` — [`Generation`](#generation), [`Offset`](#offset)

#### Trait Implementations

##### `impl Clone for GenerationOrFreelink`

- <span id="generationorfreelink-clone"></span>`fn clone(&self) -> GenerationOrFreelink` — [`GenerationOrFreelink`](#generationorfreelink)

##### `impl Copy for GenerationOrFreelink`

##### `impl Debug for GenerationOrFreelink`

- <span id="generationorfreelink-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for GenerationOrFreelink`

- <span id="generationorfreelink-default"></span>`fn default() -> GenerationOrFreelink` — [`GenerationOrFreelink`](#generationorfreelink)

##### `impl Eq for GenerationOrFreelink`

##### `impl Hash for GenerationOrFreelink`

- <span id="generationorfreelink-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for GenerationOrFreelink`

- <span id="generationorfreelink-partialeq-eq"></span>`fn eq(&self, other: &GenerationOrFreelink) -> bool` — [`GenerationOrFreelink`](#generationorfreelink)

##### `impl StructuralPartialEq for GenerationOrFreelink`

### `FreeIndex`

```rust
struct FreeIndex(usize);
```

Newtype wrapper around the index of a free slot.

#### Trait Implementations

##### `impl Clone for FreeIndex`

- <span id="freeindex-clone"></span>`fn clone(&self) -> FreeIndex` — [`FreeIndex`](#freeindex)

##### `impl Copy for FreeIndex`

##### `impl Debug for FreeIndex`

- <span id="freeindex-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for FreeIndex`

- <span id="freeindex-default"></span>`fn default() -> FreeIndex` — [`FreeIndex`](#freeindex)

##### `impl Eq for FreeIndex`

##### `impl Hash for FreeIndex`

- <span id="freeindex-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for FreeIndex`

- <span id="freeindex-partialeq-eq"></span>`fn eq(&self, other: &FreeIndex) -> bool` — [`FreeIndex`](#freeindex)

##### `impl StructuralPartialEq for FreeIndex`

### `Generation`

```rust
struct Generation(isize);
```

The generation counter.

Has a strictly positive value.

#### Implementations

- <span id="generation-advance"></span>`fn advance(&mut self)`

#### Trait Implementations

##### `impl Clone for Generation`

- <span id="generation-clone"></span>`fn clone(&self) -> Generation` — [`Generation`](#generation)

##### `impl Copy for Generation`

##### `impl Debug for Generation`

- <span id="generation-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Generation`

- <span id="generation-default"></span>`fn default() -> Self`

##### `impl Eq for Generation`

##### `impl Hash for Generation`

- <span id="generation-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Generation`

- <span id="generation-partialeq-eq"></span>`fn eq(&self, other: &Generation) -> bool` — [`Generation`](#generation)

##### `impl StructuralPartialEq for Generation`

### `Offset`

```rust
struct Offset(isize);
```

Offset of a freelist entry to the next entry.

Has a negative or zero value. Represents the negative of the offset to the next element in the
free list, wrapping around at the capacity.
The base for the offset is the *next* element for two reasons:
* Offset of `0` points to the natural successor.
* Offset of `len` would point to the element itself and should not occur.

#### Implementations

- <span id="offset-from-int-offset"></span>`fn from_int_offset(offset: usize) -> Self`

- <span id="offset-int-offset"></span>`fn int_offset(self) -> usize`

#### Trait Implementations

##### `impl Clone for Offset`

- <span id="offset-clone"></span>`fn clone(&self) -> Offset` — [`Offset`](#offset)

##### `impl Copy for Offset`

##### `impl Debug for Offset`

- <span id="offset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Offset`

- <span id="offset-default"></span>`fn default() -> Offset` — [`Offset`](#offset)

##### `impl Eq for Offset`

##### `impl Hash for Offset`

- <span id="offset-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for Offset`

- <span id="offset-ord-cmp"></span>`fn cmp(&self, other: &Offset) -> cmp::Ordering` — [`Offset`](#offset)

##### `impl PartialEq for Offset`

- <span id="offset-partialeq-eq"></span>`fn eq(&self, other: &Offset) -> bool` — [`Offset`](#offset)

##### `impl PartialOrd for Offset`

- <span id="offset-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &Offset) -> option::Option<cmp::Ordering>` — [`Offset`](#offset)

##### `impl StructuralPartialEq for Offset`

### `IndexComputer`

```rust
struct IndexComputer(usize);
```

Links FreeIndex and Offset.

#### Implementations

- <span id="indexcomputer-from-capacity"></span>`fn from_capacity(capacity: usize) -> Self`

- <span id="indexcomputer-free-list-next"></span>`fn free_list_next(&self, FreeIndex: FreeIndex, offset: Offset) -> usize` — [`FreeIndex`](#freeindex), [`Offset`](#offset)

  Get the next free list entry.

  This applies the offset to the base index, wrapping around if required.

  

  This is the reverse of `free_list_offset`.

- <span id="indexcomputer-free-list-offset"></span>`fn free_list_offset(&self, FreeIndex: FreeIndex, to: usize) -> Offset` — [`FreeIndex`](#freeindex), [`Offset`](#offset)

  Get the offset difference between the index and the next element.

  Computes a potentially wrapping positive offset where zero is the element following the

  base.

  

  This is the reverse of `free_list_next`.

