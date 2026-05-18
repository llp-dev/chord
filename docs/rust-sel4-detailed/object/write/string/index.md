*[object](../../index.md) / [write](../index.md) / [string](index.md)*

---

# Module `string`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`StringId`](#stringid) | struct | An identifier for an entry in a string table. |
| [`StringTable`](#stringtable) | struct |  |
| [`sort`](#sort) | fn |  |
| [`byte`](#byte) | fn |  |
| [`IndexSet`](#indexset) | type |  |

## Structs

### `StringId`

```rust
struct StringId(usize);
```

An identifier for an entry in a string table.

#### Trait Implementations

##### `impl Clone for StringId`

- <span id="stringid-clone"></span>`fn clone(&self) -> StringId` — [`StringId`](#stringid)

##### `impl Copy for StringId`

##### `impl Debug for StringId`

- <span id="stringid-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for StringId`

##### `impl<K> Equivalent for StringId`

- <span id="stringid-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl PartialEq for StringId`

- <span id="stringid-partialeq-eq"></span>`fn eq(&self, other: &StringId) -> bool` — [`StringId`](#stringid)

##### `impl StructuralPartialEq for StringId`

### `StringTable<'a>`

```rust
struct StringTable<'a> {
    strings: indexmap::IndexSet<&'a [u8]>,
    offsets: alloc::vec::Vec<usize>,
}
```

#### Implementations

- <span id="stringtable-add"></span>`fn add(&mut self, string: &'a [u8]) -> StringId` — [`StringId`](#stringid)

  Add a string to the string table.

  

  Panics if the string table has already been written, or

  if the string contains a null byte.

- <span id="stringtable-get-id"></span>`fn get_id(&self, string: &[u8]) -> StringId` — [`StringId`](#stringid)

  Return the id of the given string.

  

  Panics if the string is not in the string table.

- <span id="stringtable-get-string"></span>`fn get_string(&self, id: StringId) -> &'a [u8]` — [`StringId`](#stringid)

  Return the string for the given id.

  

  Panics if the string is not in the string table.

- <span id="stringtable-get-offset"></span>`fn get_offset(&self, id: StringId) -> usize` — [`StringId`](#stringid)

  Return the offset of the given string.

  

  Panics if the string table has not been written, or

  if the string is not in the string table.

- <span id="stringtable-write"></span>`fn write(&mut self, base: usize, w: &mut Vec<u8>)`

  Append the string table to the given `Vec`, and

  calculate the list of string offsets.

  

  `base` is the initial string table offset. For example,

  this should be 1 for ELF, to account for the initial

  null byte (which must have been written by the caller).

  

  Panics if the string table has already been written.

- <span id="stringtable-size"></span>`fn size(&self, base: usize) -> usize`

  Calculate the size in bytes of the string table.

  

  `base` is the initial string table offset. For example,

  this should be 1 for ELF, to account for the initial

  null byte.

#### Trait Implementations

##### `impl Debug for StringTable<'a>`

- <span id="stringtable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for StringTable<'a>`

- <span id="stringtable-default"></span>`fn default() -> StringTable<'a>` — [`StringTable`](#stringtable)

## Functions

### `sort`

```rust
fn sort(ids: &mut [usize], pos: usize, strings: &indexmap::IndexSet<&[u8]>)
```

### `byte`

```rust
fn byte(id: usize, pos: usize, strings: &indexmap::IndexSet<&[u8]>) -> u8
```

## Type Aliases

### `IndexSet<K>`

```rust
type IndexSet<K> = indexmap::IndexSet<K>;
```

