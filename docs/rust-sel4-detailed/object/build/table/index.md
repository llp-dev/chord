*[object](../../index.md) / [build](../index.md) / [table](index.md)*

---

# Module `table`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`id_private`](#id-private) | mod |  |
| [`Table`](#table) | struct | A table of items. |
| [`TableIter`](#tableiter) | struct | An iterator for non-deleted items in a [`Table`]. |
| [`TableIterMut`](#tableitermut) | struct | An iterator for non-deleted items in a [`Table`]. |
| [`Item`](#item) | trait | An item in a [`Table`]. |
| [`Id`](#id) | trait | An identifier for referring to an item in a [`Table`]. |

## Modules

- [`id_private`](id_private/index.md)

## Structs

### `Table<T>`

```rust
struct Table<T>(alloc::vec::Vec<T>);
```

A table of items.

Each item has a unique identifier.
Items can be deleted without changing the identifiers of other items.

#### Implementations

- <span id="table-new"></span>`fn new() -> Self`

#### Trait Implementations

##### `impl<T: fmt::Debug> Debug for Table<T>`

- <span id="table-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: Item> IntoIterator for &'a Table<T>`

- <span id="a-table-intoiterator-type-item"></span>`type Item = &'a T`

- <span id="a-table-intoiterator-type-intoiter"></span>`type IntoIter = TableIter<'a, T>`

- <span id="a-table-intoiterator-into-iter"></span>`fn into_iter(self) -> TableIter<'a, T>` — [`TableIter`](#tableiter)

### `TableIter<'a, T>`

```rust
struct TableIter<'a, T> {
    iter: core::slice::Iter<'a, T>,
}
```

An iterator for non-deleted items in a [`Table`](#table).

#### Trait Implementations

##### `impl<T: fmt::Debug> Debug for TableIter<'a, T>`

- <span id="tableiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for TableIter<'a, T>`

- <span id="tableiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="tableiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="tableiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T: Item> Iterator for TableIter<'a, T>`

- <span id="tableiter-iterator-type-item"></span>`type Item = &'a T`

- <span id="tableiter-iterator-next"></span>`fn next(&mut self) -> Option<&'a T>`

### `TableIterMut<'a, T>`

```rust
struct TableIterMut<'a, T> {
    iter: core::slice::IterMut<'a, T>,
}
```

An iterator for non-deleted items in a [`Table`](#table).

#### Trait Implementations

##### `impl<T: fmt::Debug> Debug for TableIterMut<'a, T>`

- <span id="tableitermut-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for TableIterMut<'a, T>`

- <span id="tableitermut-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="tableitermut-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="tableitermut-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<T: Item> Iterator for TableIterMut<'a, T>`

- <span id="tableitermut-iterator-type-item"></span>`type Item = &'a mut T`

- <span id="tableitermut-iterator-next"></span>`fn next(&mut self) -> Option<&'a mut T>`

## Traits

### `Item`

```rust
trait Item { ... }
```

An item in a [`Table`](#table).

#### Associated Types

- `type Id: 1`

#### Required Methods

- `fn is_deleted(&self) -> bool`

  Return `True` if the item is deleted.

#### Implementors

- [`Section`](../elf/index.md#section)
- [`Segment`](../elf/index.md#segment)
- [`Symbol`](../elf/index.md#symbol)
- [`VersionFile`](../elf/index.md#versionfile)
- [`Version`](../elf/index.md#version)

### `Id`

```rust
trait Id: IdPrivate { ... }
```

An identifier for referring to an item in a [`Table`](#table).

#### Required Methods

- `fn index(&self) -> usize`

  Return the index of the item in the table.

#### Implementors

- [`SectionId`](../elf/index.md#sectionid)
- [`SegmentId`](../elf/index.md#segmentid)
- [`SymbolId`](../elf/index.md#symbolid)
- [`VersionFileId`](../elf/index.md#versionfileid)
- [`VersionId`](../elf/index.md#versionid)

