**managed > slotmap**

# Module: slotmap

## Contents

**Structs**

- [`Key`](#key) - An index into a slotmap.
- [`Slot`](#slot) - Provides links between slots and elements.
- [`SlotMap`](#slotmap) - Provides a slotmap based on external memory.

---

## managed::slotmap::Key

*Struct*

An index into a slotmap.

The index remains valid until the entry is removed. If accessing the slotmap with the index
again after the entry was removed will fail, even if the index where the element was previously
stored has been reused for another element.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Key) -> bool`
- **Default**
  - `fn default() -> Key`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Key`



## managed::slotmap::Slot

*Struct*

Provides links between slots and elements.

The benefit of separating this struct from the elements is that it is unconditionally `Copy`
and `Default`. It also provides better locality for both the indices and the elements which
could help with iteration or very large structs.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Default**
  - `fn default() -> Slot`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Slot`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **PartialEq**
  - `fn eq(self: &Self, other: &Slot) -> bool`



## managed::slotmap::SlotMap

*Struct*

Provides a slotmap based on external memory.

A slotmap provides a `Vec`-like interface where each entry is associated with a stable
index-like key. Lookup with the key will detect if an entry has been removed but does not
require a lifetime relation. Compared to other slotmap implementations this does not internally
allocate any memory on its own but only relies on the [`Slice`] arguments in the constructor.

[`Slice`]: ../enum.Slice.html

## Usage

The important aspect is that the slotmap does not create the storage of its own elements, it
merely manages one given to it at construction time.

```
# use managed::{ManagedSlice, SlotMap, SlotIndex};

let mut elements = [0usize; 1024];
let mut slots = [SlotIndex::default(); 1024];

let mut map = SlotMap::new(
    ManagedSlice::Borrowed(&mut elements[..]),
    ManagedSlice::Borrowed(&mut slots[..]));
let index = map.insert(42).unwrap();
assert_eq!(map.get(index).cloned(), Some(42));
```

**Generic Parameters:**
- 'a
- T

**Methods:**

- `fn new(elements: Slice<'a, T>, slots: Slice<'a, Slot>) -> Self` - Create a slot map.
- `fn get(self: &Self, index: Key) -> Option<&T>` - Retrieve a value by index.
- `fn get_mut(self: & mut Self, index: Key) -> Option<& mut T>` - Retrieve a mutable value by index.
- `fn reserve(self: & mut Self) -> Option<(Key, & mut T)>` - Reserve a new entry.
- `fn insert(self: & mut Self, value: T) -> Option<Key>` - Try to insert a value into the map.
- `fn remove(self: & mut Self, index: Key) -> Option<& mut T>` - Remove an element.



