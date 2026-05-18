**object > build > table**

# Module: build::table

## Contents

**Structs**

- [`Table`](#table) - A table of items.
- [`TableIter`](#tableiter) - An iterator for non-deleted items in a [`Table`].
- [`TableIterMut`](#tableitermut) - An iterator for non-deleted items in a [`Table`].

**Traits**

- [`Id`](#id) - An identifier for referring to an item in a [`Table`].
- [`Item`](#item) - An item in a [`Table`].

---

## object::build::table::Id

*Trait*

An identifier for referring to an item in a [`Table`].

**Methods:**

- `index`: Return the index of the item in the table.



## object::build::table::Item

*Trait*

An item in a [`Table`].

**Methods:**

- `Id`: The type of identifier for the item.
- `is_deleted`: Return `True` if the item is deleted.



## object::build::table::Table

*Struct*

A table of items.

Each item has a unique identifier.
Items can be deleted without changing the identifiers of other items.

**Generic Parameters:**
- T

**Tuple Struct**: `()`

**Methods:**

- `fn add(self: & mut Self) -> & mut Section<'data>` - Add a new section to the table.
- `fn copy(self: & mut Self, id: SectionId) -> & mut Section<'data>` - Add a copy of a section to the table.
- `fn count_defined(self: &Self) -> usize` - Number of defined symbols.
- `fn add(self: & mut Self) -> & mut Symbol<'data, DYNAMIC>` - Add a new symbol to the table.
- `fn is_empty(self: &Self) -> bool` - Return `True` if there are no non-deleted items.
- `fn count(self: &Self) -> usize` - Number of non-deleted items.
- `fn get(self: &Self, id: <T as >::Id) -> &T` - Return a reference to an item.
- `fn get_mut(self: & mut Self, id: <T as >::Id) -> & mut T` - Return a mutable reference to a segment.
- `fn iter(self: &Self) -> TableIter<T>` - Return an iterator for the segments.
- `fn iter_mut(self: & mut Self) -> TableIterMut<T>` - Return a mutable iterator for the segments.
- `fn add(self: & mut Self) -> & mut Segment<'data>` - Add a new segment to the table.
- `fn find_load_segment_from_offset(self: &Self, offset: u64) -> Option<&Segment<'data>>` - Find a `PT_LOAD` segment containing the given offset.
- `fn add_load_segment(self: & mut Self, flags: u32, align: u64) -> & mut Segment<'data>` - Add a new `PT_LOAD` segment to the table.
- `fn copy(self: & mut Self, id: SegmentId) -> & mut Segment<'data>` - Add a copy of a segment to the table.
- `fn add(self: & mut Self, name: ByteString<'data>) -> VersionFileId` - Add a new filename to the table.
- `fn add(self: & mut Self, data: VersionData<'data>) -> VersionId` - Add a version.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::build::table::TableIter

*Struct*

An iterator for non-deleted items in a [`Table`].

**Generic Parameters:**
- 'a
- T



## object::build::table::TableIterMut

*Struct*

An iterator for non-deleted items in a [`Table`].

**Generic Parameters:**
- 'a
- T



