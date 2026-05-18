*[gimli](../../index.md) / [read](../index.md) / [names](index.md)*

---

# Module `names`

Functions for parsing DWARF 5 `.debug_names` sections.

The `.debug_names` section provides an accelerated access table for debugging
information entries (DIEs) organized by name. This section is defined in
DWARF 5 Section 6.1.1 and enables efficient lookup of symbols without
scanning the entire `.debug_info` section.

# DWARF 5 Name Index

A name index in the `.debug_names` section contains:
- **Header**: Format, version, and table counts
- **CU/TU Lists**: Lists of compilation and type units
- **Hash Table**: Bucket-based hash table for name lookup
- **Name Table**: String and entry offsets for each name
- **Abbreviation Table**: Describes entry structure and attributes
- **Entry Pool**: Series of entries with abbreviation codes and attributes

Per DWARF 5 Section 6.1.1.3, a `.debug_names` section can contain multiple
name indexes. There are two strategies:
- **Per-module index**: Single index covering all compilation units (most common)
- **Per-CU indexes**: Separate indexes for individual compilation units

The choice depends on the compiler/linker. When looking up names, all indexes
must be searched since a name could appear in any index.


## Contents

- [Structs](#structs)
  - [`DebugNames`](#debugnames)
  - [`NameIndexHeaderIter`](#nameindexheaderiter)
  - [`NameIndexHeader`](#nameindexheader)
  - [`NameTableIndex`](#nametableindex)
  - [`NameIndex`](#nameindex)
  - [`NameTableIter`](#nametableiter)
  - [`NameBucketIter`](#namebucketiter)
  - [`NameHashIter`](#namehashiter)
  - [`NameEntryIter`](#nameentryiter)
  - [`NameEntryOffset`](#nameentryoffset)
  - [`NameEntry`](#nameentry)
  - [`NameAttribute`](#nameattribute)
  - [`NameAbbreviations`](#nameabbreviations)
  - [`NameAbbreviation`](#nameabbreviation)
  - [`NameAbbreviationAttribute`](#nameabbreviationattribute)
- [Enums](#enums)
  - [`NameTypeUnit`](#nametypeunit)
  - [`NameAttributeValue`](#nameattributevalue)
- [Functions](#functions)
  - [`read_debug_names_form_value`](#read-debug-names-form-value)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`DebugNames`](#debugnames) | struct | The `DebugNames` struct represents the DWARF 5 name index information found in the `.debug_names` section. |
| [`NameIndexHeaderIter`](#nameindexheaderiter) | struct | An iterator over the name index headers in the `.debug_names` section. |
| [`NameIndexHeader`](#nameindexheader) | struct | The header of a name index in the `.debug_names` section. |
| [`NameTableIndex`](#nametableindex) | struct | An index into the name table of a `NameIndex`. |
| [`NameIndex`](#nameindex) | struct | A single name index from the `.debug_names` section. |
| [`NameTableIter`](#nametableiter) | struct | An iterator over the indexes of all names in a name index. |
| [`NameBucketIter`](#namebucketiter) | struct | An iterator over the hash entries for a bucket in a name index hash table. |
| [`NameHashIter`](#namehashiter) | struct | An iterator over the indexes of the names in a name index hash table that match a hash value. |
| [`NameEntryIter`](#nameentryiter) | struct | An iterator for a series of name entries in a name index entry pool. |
| [`NameEntryOffset`](#nameentryoffset) | struct | An offset into the entry pool of a name index. |
| [`NameEntry`](#nameentry) | struct | A parsed entry from the `.debug_names` section. |
| [`NameAttribute`](#nameattribute) | struct | A parsed attribute for a [`NameEntry`]. |
| [`NameAbbreviations`](#nameabbreviations) | struct | A table of name entry abbreviations. |
| [`NameAbbreviation`](#nameabbreviation) | struct | A name abbreviation entry defines how name entries are encoded. |
| [`NameAbbreviationAttribute`](#nameabbreviationattribute) | struct | An attribute specification in a name abbreviation. |
| [`NameTypeUnit`](#nametypeunit) | enum | A reference to a type unit. |
| [`NameAttributeValue`](#nameattributevalue) | enum | A parsed attribute value for a [`NameEntry`]. |
| [`read_debug_names_form_value`](#read-debug-names-form-value) | fn | Read an attribute value. |

## Structs

### `DebugNames<R>`

```rust
struct DebugNames<R> {
    section: R,
}
```

The `DebugNames` struct represents the DWARF 5 name index information
found in the `.debug_names` section.

The `.debug_names` section provides an index for efficiently finding
debugging information entries (DIEs) by name. It contains hash tables
that map names to DIE offsets, allowing debuggers to quickly locate
functions, variables, types, and other named entities.

#### Implementations

- <span id="debugnames-new"></span>`fn new(debug_names_section: &'input [u8], endian: Endian) -> Self`

  Construct a new `DebugNames` instance from the data in the `.debug_names`

  section.

  

  It is the caller's responsibility to read the `.debug_names` section and

  present it as a `&[u8]` slice. That means using some ELF loader on

  Linux, a Mach-O loader on macOS, etc.

  

  ```rust

  use gimli::{DebugNames, LittleEndian};

  

  let buf = [0x00, 0x01, 0x02, 0x03];

  let read_debug_names_section_somehow = || &buf;

  let debug_names =

      DebugNames::new(read_debug_names_section_somehow(), LittleEndian);

  ```

#### Trait Implementations

##### `impl<R: clone::Clone> Clone for DebugNames<R>`

- <span id="debugnames-clone"></span>`fn clone(&self) -> DebugNames<R>` — [`DebugNames`](../index.md#debugnames)

##### `impl<R: marker::Copy> Copy for DebugNames<R>`

##### `impl<R: fmt::Debug> Debug for DebugNames<R>`

- <span id="debugnames-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: default::Default> Default for DebugNames<R>`

- <span id="debugnames-default"></span>`fn default() -> DebugNames<R>` — [`DebugNames`](../index.md#debugnames)

##### `impl<R> Section for DebugNames<R>`

- <span id="debugnames-section-id"></span>`fn id() -> SectionId` — [`SectionId`](../../index.md#sectionid)

- <span id="debugnames-section-reader"></span>`fn reader(&self) -> &R`

### `NameIndexHeaderIter<R: Reader>`

```rust
struct NameIndexHeaderIter<R: Reader> {
    input: R,
    end_offset: <R as >::Offset,
}
```

An iterator over the name index headers in the `.debug_names` section.

#### Implementations

- <span id="nameindexheaderiter-next"></span>`fn next(&mut self) -> Result<Option<NameIndexHeader<R>>>` — [`Result`](../../index.md#result), [`NameIndexHeader`](../index.md#nameindexheader)

  Advance the iterator and return the next name index header.

  

  Returns `Ok(None)` when iteration is complete.

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for NameIndexHeaderIter<R>`

- <span id="nameindexheaderiter-clone"></span>`fn clone(&self) -> NameIndexHeaderIter<R>` — [`NameIndexHeaderIter`](../index.md#nameindexheaderiter)

##### `impl<R: fmt::Debug + Reader> Debug for NameIndexHeaderIter<R>`

- <span id="nameindexheaderiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for NameIndexHeaderIter<R>`

- <span id="nameindexheaderiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="nameindexheaderiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="nameindexheaderiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: Reader> Iterator for NameIndexHeaderIter<R>`

- <span id="nameindexheaderiter-iterator-type-item"></span>`type Item = Result<NameIndexHeader<R>, Error>`

- <span id="nameindexheaderiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `NameIndexHeader<R: Reader>`

```rust
struct NameIndexHeader<R: Reader> {
    offset: crate::common::DebugNamesOffset<<R as >::Offset>,
    length: <R as >::Offset,
    format: crate::common::Format,
    version: u16,
    compile_unit_count: u32,
    local_type_unit_count: u32,
    foreign_type_unit_count: u32,
    bucket_count: u32,
    name_count: u32,
    abbrev_table_size: u32,
    augmentation_string: Option<R>,
    content: R,
}
```

The header of a name index in the `.debug_names` section.

#### Fields

- **`offset`**: `crate::common::DebugNamesOffset<<R as >::Offset>`

  The section offset of the header.

- **`length`**: `<R as >::Offset`

  The length of this name index.

- **`format`**: `crate::common::Format`

  The format of the unit.

- **`version`**: `u16`

  Version of the name index format (should be 5 for DWARF 5).

- **`compile_unit_count`**: `u32`

  Number of compilation units in the CU list.

- **`local_type_unit_count`**: `u32`

  Number of type units in the local TU list.

- **`foreign_type_unit_count`**: `u32`

  Number of type units in the foreign TU list.

- **`bucket_count`**: `u32`

  Number of buckets in the hash table.

- **`name_count`**: `u32`

  Number of unique name entries.

- **`abbrev_table_size`**: `u32`

  Size of the abbreviations table in bytes.

- **`augmentation_string`**: `Option<R>`

  The augmentation string.

- **`content`**: `R`

  The remaining unparsed contents of the index.

#### Implementations

- <span id="nameindexheader-index"></span>`fn index(self) -> Result<NameIndex<R>>` — [`Result`](../../index.md#result), [`NameIndex`](../index.md#nameindex)

  Convert the header into a `NameIndex`.

- <span id="nameindexheader-offset"></span>`fn offset(&self) -> DebugNamesOffset<<R as >::Offset>` — [`DebugNamesOffset`](../../index.md#debugnamesoffset), [`Reader`](../index.md#reader)

  Return the section offset of this name index.

- <span id="nameindexheader-length"></span>`fn length(&self) -> <R as >::Offset` — [`Reader`](../index.md#reader)

  Return the index length.

- <span id="nameindexheader-format"></span>`fn format(&self) -> Format` — [`Format`](../../index.md#format)

  Return the format (DWARF32 or DWARF64).

- <span id="nameindexheader-version"></span>`fn version(&self) -> u16`

  Return the version of this name index.

- <span id="nameindexheader-compile-unit-count"></span>`fn compile_unit_count(&self) -> u32`

  Return the number of compilation units in this index.

- <span id="nameindexheader-local-type-unit-count"></span>`fn local_type_unit_count(&self) -> u32`

  Return the number of local type units in this index.

- <span id="nameindexheader-foreign-type-unit-count"></span>`fn foreign_type_unit_count(&self) -> u32`

  Return the number of foreign type units in this index.

- <span id="nameindexheader-bucket-count"></span>`fn bucket_count(&self) -> u32`

  Return the number of buckets in the hash table.

- <span id="nameindexheader-name-count"></span>`fn name_count(&self) -> u32`

  Return the number of unique name entries.

- <span id="nameindexheader-abbrev-table-size"></span>`fn abbrev_table_size(&self) -> u32`

  Return the size of the abbreviations table in bytes.

- <span id="nameindexheader-augmentation-string"></span>`fn augmentation_string(&self) -> Option<&R>`

  Return the augmentation string.

- <span id="nameindexheader-parse"></span>`fn parse(input: &mut R, offset: DebugNamesOffset<<R as >::Offset>) -> Result<Self>` — [`DebugNamesOffset`](../../index.md#debugnamesoffset), [`Reader`](../index.md#reader), [`Result`](../../index.md#result)

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for NameIndexHeader<R>`

- <span id="nameindexheader-clone"></span>`fn clone(&self) -> NameIndexHeader<R>` — [`NameIndexHeader`](../index.md#nameindexheader)

##### `impl<R: fmt::Debug + Reader> Debug for NameIndexHeader<R>`

- <span id="nameindexheader-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `NameTableIndex`

```rust
struct NameTableIndex(u32);
```

An index into the name table of a `NameIndex`.

This is used as an index into the list of string offsets, the list of entry
offsets, and the list of hashes.

Note that while the DWARF standard specifies that indexes in the DWARF data
start at 1, we use a zero based index here. Functions that read an index from
the data will automatically adjust the index to start at 0.

#### Trait Implementations

##### `impl Clone for NameTableIndex`

- <span id="nametableindex-clone"></span>`fn clone(&self) -> NameTableIndex` — [`NameTableIndex`](../index.md#nametableindex)

##### `impl Copy for NameTableIndex`

##### `impl Debug for NameTableIndex`

- <span id="nametableindex-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for NameTableIndex`

##### `impl PartialEq for NameTableIndex`

- <span id="nametableindex-partialeq-eq"></span>`fn eq(&self, other: &NameTableIndex) -> bool` — [`NameTableIndex`](../index.md#nametableindex)

##### `impl StructuralPartialEq for NameTableIndex`

### `NameIndex<R: Reader>`

```rust
struct NameIndex<R: Reader> {
    format: crate::common::Format,
    compile_unit_count: u32,
    local_type_unit_count: u32,
    foreign_type_unit_count: u32,
    bucket_count: u32,
    name_count: u32,
    compile_unit_list: R,
    local_type_unit_list: R,
    foreign_type_unit_list: R,
    bucket_data: R,
    hash_table_data: R,
    name_table_data: R,
    entry_offset_data: R,
    entry_pool: R,
    abbreviations: NameAbbreviations,
}
```

A single name index from the `.debug_names` section.

It provides access to the compilation unit table, type unit tables, hash table, name
table, and entry pool that make up the accelerated lookup structure.

#### Implementations

- <span id="nameindex-new"></span>`fn new(header: NameIndexHeader<R>) -> Result<Self>` — [`NameIndexHeader`](../index.md#nameindexheader), [`Result`](../../index.md#result)

  Create a new name index from a header.

- <span id="nameindex-compile-unit-count"></span>`fn compile_unit_count(&self) -> u32`

  Return the number of compilation units in this index.

- <span id="nameindex-compile-unit"></span>`fn compile_unit(&self, index: u32) -> Result<DebugInfoOffset<<R as >::Offset>>` — [`Result`](../../index.md#result), [`DebugInfoOffset`](../../index.md#debuginfooffset), [`Reader`](../index.md#reader)

  Get the `.debug_info` offset of a compilation unit.

  

  `index` must be less than `Self::compile_unit_count`.

  

  Returns an error if `index` is invalid.

- <span id="nameindex-default-compile-unit"></span>`fn default_compile_unit(&self) -> Result<Option<DebugInfoOffset<<R as >::Offset>>>` — [`Result`](../../index.md#result), [`DebugInfoOffset`](../../index.md#debuginfooffset), [`Reader`](../index.md#reader)

  Return the `.debug_info` offset of the default compilation unit, if any.

  

  If there is only one compilation unit, then entries may omit the `DW_IDX_compile_unit`

  attribute.

- <span id="nameindex-local-type-unit-count"></span>`fn local_type_unit_count(&self) -> u32`

  Return the number of local type units in this index.

- <span id="nameindex-local-type-unit"></span>`fn local_type_unit(&self, index: u32) -> Result<DebugInfoOffset<<R as >::Offset>>` — [`Result`](../../index.md#result), [`DebugInfoOffset`](../../index.md#debuginfooffset), [`Reader`](../index.md#reader)

  Get the `.debug_info` offset of a local type unit.

  

  `index` must be less than `Self::local_type_unit_count`.

  

  Returns an error if `index` is invalid.

- <span id="nameindex-foreign-type-unit-count"></span>`fn foreign_type_unit_count(&self) -> u32`

  Return the number of foreign type units in this index.

- <span id="nameindex-foreign-type-unit"></span>`fn foreign_type_unit(&self, index: u32) -> Result<DebugTypeSignature>` — [`Result`](../../index.md#result), [`DebugTypeSignature`](../../index.md#debugtypesignature)

  Get the signature of a foreign type unit.

  

  `index` must be less than `Self::foreign_type_unit_count`.

  

  Returns an error if `index` is invalid.

- <span id="nameindex-type-unit-count"></span>`fn type_unit_count(&self) -> u32`

  Return the number of type units in this index, both local and foreign.

- <span id="nameindex-type-unit"></span>`fn type_unit(&self, index: u32) -> Result<NameTypeUnit<<R as >::Offset>>` — [`Result`](../../index.md#result), [`NameTypeUnit`](../index.md#nametypeunit), [`Reader`](../index.md#reader)

  Get a type unit reference.

  

  `index` must be less than `Self::type_unit_count`, and normally is

  obtained from a `DW_IDX_type_unit` attribute.

  

  Returns an error if `index` is invalid.

- <span id="nameindex-has-hash-table"></span>`fn has_hash_table(&self) -> bool`

  Return true if the name index contains a hash table.

- <span id="nameindex-bucket-count"></span>`fn bucket_count(&self) -> u32`

  Return the number of buckets in the hash table.

- <span id="nameindex-find-by-bucket"></span>`fn find_by_bucket(&self, bucket_index: u32) -> Result<Option<NameBucketIter<R>>>` — [`Result`](../../index.md#result), [`NameBucketIter`](../index.md#namebucketiter)

  Iterate over the hash entries for a bucket in the hash table.

  

  This function is only for diagnostic uses. Usually `Self::find_by_hash` should be

  called instead.

  

  The given bucket index is 0 based, and must be less than `Self::bucket_count`.

  

  Returns an error if there is no hash table or the bucket index is invalid.

  Returns `Ok(None)` if the bucket is empty.

- <span id="nameindex-find-by-hash"></span>`fn find_by_hash(&self, hash_value: u32) -> Result<NameHashIter<R>>` — [`Result`](../../index.md#result), [`NameHashIter`](../index.md#namehashiter)

  Iterate over the indexes of the names with the given hash value.

  

  The user must then check each name to see if it matches the desired name.

  

  Returns an error if there is no hash table.

- <span id="nameindex-name-count"></span>`fn name_count(&self) -> u32`

  Get the number of names in the name index.

  

  This is 1 greater than the maximum valid [`NameTableIndex`](../index.md).

- <span id="nameindex-names"></span>`fn names(&self) -> NameTableIter` — [`NameTableIter`](../index.md#nametableiter)

  Iterate over the indexes of all names in the name table.

- <span id="nameindex-name-string-offset"></span>`fn name_string_offset(&self, index: NameTableIndex) -> Result<DebugStrOffset<<R as >::Offset>>` — [`NameTableIndex`](../index.md#nametableindex), [`Result`](../../index.md#result), [`DebugStrOffset`](../../index.md#debugstroffset), [`Reader`](../index.md#reader)

  Get the string table offset for the name at the given index.

  

  Returns an error if `index` is invalid.

- <span id="nameindex-name-string"></span>`fn name_string(&self, index: NameTableIndex, debug_str: &DebugStr<R>) -> Result<R>` — [`NameTableIndex`](../index.md#nametableindex), [`DebugStr`](../index.md#debugstr), [`Result`](../../index.md#result)

  Get the name at the given index using the provided `.debug_str` section.

  

  Returns an error if `index` is invalid, or the string table offset is invalid.

- <span id="nameindex-name-entries"></span>`fn name_entries(&self, index: NameTableIndex) -> Result<NameEntryIter<'_, R>>` — [`NameTableIndex`](../index.md#nametableindex), [`Result`](../../index.md#result), [`NameEntryIter`](../index.md#nameentryiter)

  Iterate over the series of entries for the given name table index.

  

  Each name in the name table has a corresponding series of entries

  with that name in the entry pool.

  

  Returns an error if `index` is invalid, or the entry pool offset is invalid.

- <span id="nameindex-name-entry"></span>`fn name_entry(&self, offset: NameEntryOffset<<R as >::Offset>) -> Result<NameEntry<R>>` — [`NameEntryOffset`](../index.md#nameentryoffset), [`Reader`](../index.md#reader), [`Result`](../../index.md#result), [`NameEntry`](../index.md#nameentry)

  Parse the entry at the given entry pool offset.

  

  This is useful for reading the entry referenced by a `DW_IDX_parent` attribute.

- <span id="nameindex-abbreviations"></span>`fn abbreviations(&self) -> &NameAbbreviations` — [`NameAbbreviations`](../index.md#nameabbreviations)

  Get the abbreviation table for name entries in this name index.

#### Trait Implementations

##### `impl<R: fmt::Debug + Reader> Debug for NameIndex<R>`

- <span id="nameindex-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `NameTableIter`

```rust
struct NameTableIter {
    name_table_index: NameTableIndex,
    name_count: u32,
}
```

An iterator over the indexes of all names in a name index.

#### Implementations

- <span id="nametableiter-new"></span>`fn new<R: Reader>(name_index: &NameIndex<R>) -> Self` — [`NameIndex`](../index.md#nameindex)

#### Trait Implementations

##### `impl Debug for NameTableIter`

- <span id="nametableiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for NameTableIter`

- <span id="nametableiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="nametableiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="nametableiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl Iterator for NameTableIter`

- <span id="nametableiter-iterator-type-item"></span>`type Item = NameTableIndex`

- <span id="nametableiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `NameBucketIter<R: Reader>`

```rust
struct NameBucketIter<R: Reader> {
    reader: R,
    name_table_index: NameTableIndex,
    name_count: u32,
    bucket_index: u32,
    bucket_count: u32,
}
```

An iterator over the hash entries for a bucket in a name index hash table.

#### Implementations

- <span id="namebucketiter-new"></span>`fn new(name_index: &NameIndex<R>, bucket_index: u32) -> Result<Option<Self>>` — [`NameIndex`](../index.md#nameindex), [`Result`](../../index.md#result)

- <span id="namebucketiter-next"></span>`fn next(&mut self) -> Result<Option<(NameTableIndex, u32)>>` — [`Result`](../../index.md#result), [`NameTableIndex`](../index.md#nametableindex)

  Advance the iterator and return the next name table index and hash.

#### Trait Implementations

##### `impl<R: fmt::Debug + Reader> Debug for NameBucketIter<R>`

- <span id="namebucketiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for NameBucketIter<R>`

- <span id="namebucketiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="namebucketiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="namebucketiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: Reader> Iterator for NameBucketIter<R>`

- <span id="namebucketiter-iterator-type-item"></span>`type Item = Result<(NameTableIndex, u32), Error>`

- <span id="namebucketiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `NameHashIter<R: Reader>`

```rust
struct NameHashIter<R: Reader> {
    bucket_iter: Option<NameBucketIter<R>>,
    hash: u32,
}
```

An iterator over the indexes of the names in a name index hash table that match a hash
value.

#### Implementations

- <span id="namehashiter-new"></span>`fn new(name_index: &NameIndex<R>, hash: u32) -> Result<Self>` — [`NameIndex`](../index.md#nameindex), [`Result`](../../index.md#result)

- <span id="namehashiter-next"></span>`fn next(&mut self) -> Result<Option<NameTableIndex>>` — [`Result`](../../index.md#result), [`NameTableIndex`](../index.md#nametableindex)

  Advance the iterator and return the next name table index.

#### Trait Implementations

##### `impl<R: fmt::Debug + Reader> Debug for NameHashIter<R>`

- <span id="namehashiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for NameHashIter<R>`

- <span id="namehashiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="namehashiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="namehashiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: Reader> Iterator for NameHashIter<R>`

- <span id="namehashiter-iterator-type-item"></span>`type Item = Result<NameTableIndex, Error>`

- <span id="namehashiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `NameEntryIter<'a, R: Reader>`

```rust
struct NameEntryIter<'a, R: Reader> {
    entries: R,
    end_offset: <R as >::Offset,
    abbreviations: &'a NameAbbreviations,
}
```

An iterator for a series of name entries in a name index entry pool.

Each name in a name index corresponds to a series of entries
with that name.

#### Implementations

- <span id="nameentryiter-new"></span>`fn new(name_index: &'a NameIndex<R>, index: NameTableIndex) -> Result<Self>` — [`NameIndex`](../index.md#nameindex), [`NameTableIndex`](../index.md#nametableindex), [`Result`](../../index.md#result)

- <span id="nameentryiter-next"></span>`fn next(&mut self) -> Result<Option<NameEntry<R>>>` — [`Result`](../../index.md#result), [`NameEntry`](../index.md#nameentry)

  Advance the iterator and return the next name entry.

#### Trait Implementations

##### `impl<R: fmt::Debug + Reader> Debug for NameEntryIter<'a, R>`

- <span id="nameentryiter-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for NameEntryIter<'a, R>`

- <span id="nameentryiter-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="nameentryiter-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="nameentryiter-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<R: Reader> Iterator for NameEntryIter<'a, R>`

- <span id="nameentryiter-iterator-type-item"></span>`type Item = Result<NameEntry<R>, Error>`

- <span id="nameentryiter-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `NameEntryOffset<T>`

```rust
struct NameEntryOffset<T>(T);
```

An offset into the entry pool of a name index.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for NameEntryOffset<T>`

- <span id="nameentryoffset-clone"></span>`fn clone(&self) -> NameEntryOffset<T>` — [`NameEntryOffset`](../index.md#nameentryoffset)

##### `impl<T: marker::Copy> Copy for NameEntryOffset<T>`

##### `impl<T: fmt::Debug> Debug for NameEntryOffset<T>`

- <span id="nameentryoffset-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for NameEntryOffset<T>`

##### `impl<T: hash::Hash> Hash for NameEntryOffset<T>`

- <span id="nameentryoffset-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<T: cmp::Ord> Ord for NameEntryOffset<T>`

- <span id="nameentryoffset-ord-cmp"></span>`fn cmp(&self, other: &NameEntryOffset<T>) -> cmp::Ordering` — [`NameEntryOffset`](../index.md#nameentryoffset)

##### `impl<T: cmp::PartialEq> PartialEq for NameEntryOffset<T>`

- <span id="nameentryoffset-partialeq-eq"></span>`fn eq(&self, other: &NameEntryOffset<T>) -> bool` — [`NameEntryOffset`](../index.md#nameentryoffset)

##### `impl<T: cmp::PartialOrd> PartialOrd for NameEntryOffset<T>`

- <span id="nameentryoffset-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &NameEntryOffset<T>) -> option::Option<cmp::Ordering>` — [`NameEntryOffset`](../index.md#nameentryoffset)

##### `impl<T> StructuralPartialEq for NameEntryOffset<T>`

### `NameEntry<R: Reader>`

```rust
struct NameEntry<R: Reader> {
    pub offset: NameEntryOffset<<R as >::Offset>,
    pub abbrev_code: u64,
    pub tag: constants::DwTag,
    pub attrs: alloc::vec::Vec<NameAttribute<R>>,
}
```

A parsed entry from the `.debug_names` section.

#### Fields

- **`offset`**: `NameEntryOffset<<R as >::Offset>`

  The offset of the entry in the entries pool.

- **`abbrev_code`**: `u64`

  The abbreviation code for this entry.

- **`tag`**: `constants::DwTag`

  The DIE tag for this entry.

- **`attrs`**: `alloc::vec::Vec<NameAttribute<R>>`

  The attributes for this entry.

#### Implementations

- <span id="nameentry-compile-unit"></span>`fn compile_unit(&self, names: &NameIndex<R>) -> Result<Option<DebugInfoOffset<<R as >::Offset>>>` — [`NameIndex`](../index.md#nameindex), [`Result`](../../index.md#result), [`DebugInfoOffset`](../../index.md#debuginfooffset), [`Reader`](../index.md#reader)

  Get the value of the `DW_IDX_compile_unit` attribute, if any.

  

  If neither `DW_IDX_compile_unit` nor `DW_IDX_type_unit` exist then you should use

  `NameIndex::default_compile_unit`.

  

  If both `DW_IDX_compile_unit` and `DW_IDX_type_unit` exist then this value is for

  a skeleton CU that may be used to locate a split DWARF object file containing

  the type unit.

- <span id="nameentry-type-unit"></span>`fn type_unit(&self, names: &NameIndex<R>) -> Result<Option<NameTypeUnit<<R as >::Offset>>>` — [`NameIndex`](../index.md#nameindex), [`Result`](../../index.md#result), [`NameTypeUnit`](../index.md#nametypeunit), [`Reader`](../index.md#reader)

  Get the value of the `DW_IDX_type_unit` attribute, if any.

- <span id="nameentry-die-offset"></span>`fn die_offset(&self) -> Result<Option<UnitOffset<<R as >::Offset>>>` — [`Result`](../../index.md#result), [`UnitOffset`](../../index.md#unitoffset), [`Reader`](../index.md#reader)

  Get the value of the `DW_IDX_die_offset` attribute, if any.

  

  This is the offset of the DIE within the compile unit or type unit.

- <span id="nameentry-parent"></span>`fn parent(&self) -> Result<Option<Option<NameEntryOffset<<R as >::Offset>>>>` — [`Result`](../../index.md#result), [`NameEntryOffset`](../index.md#nameentryoffset), [`Reader`](../index.md#reader)

  Get the value of the `DW_IDX_parent` attribute, if any.

  

  Returns `Ok(Some(Some(offset)))` if the DIE parent is indexed.

  Returns `Ok(Some(None))` if the DIE parent is not indexed.

  Returns `Ok(None)` if it is unknown whether the DIE parent is indexed

  because the producer did not generate a `DW_IDX_parent` attribute.

- <span id="nameentry-type-hash"></span>`fn type_hash(&self) -> Result<Option<u64>>` — [`Result`](../../index.md#result)

  Get the value of the `DW_IDX_type_hash` attribute, if any.

- <span id="nameentry-parse"></span>`fn parse(entry_reader: &mut R, offset: NameEntryOffset<<R as >::Offset>, abbreviations: &NameAbbreviations) -> Result<Option<NameEntry<R>>>` — [`NameEntryOffset`](../index.md#nameentryoffset), [`Reader`](../index.md#reader), [`NameAbbreviations`](../index.md#nameabbreviations), [`Result`](../../index.md#result), [`NameEntry`](../index.md#nameentry)

  Parse a single entry from the entry pool.

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for NameEntry<R>`

- <span id="nameentry-clone"></span>`fn clone(&self) -> NameEntry<R>` — [`NameEntry`](../index.md#nameentry)

##### `impl<R: fmt::Debug + Reader> Debug for NameEntry<R>`

- <span id="nameentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: cmp::Eq + Reader> Eq for NameEntry<R>`

##### `impl<R: cmp::PartialEq + Reader> PartialEq for NameEntry<R>`

- <span id="nameentry-partialeq-eq"></span>`fn eq(&self, other: &NameEntry<R>) -> bool` — [`NameEntry`](../index.md#nameentry)

##### `impl<R: Reader> StructuralPartialEq for NameEntry<R>`

### `NameAttribute<R: Reader>`

```rust
struct NameAttribute<R: Reader> {
    name: constants::DwIdx,
    form: constants::DwForm,
    value: NameAttributeValue<R>,
}
```

A parsed attribute for a [`NameEntry`](../index.md).

#### Implementations

- <span id="nameattribute-name"></span>`fn name(&self) -> constants::DwIdx` — [`DwIdx`](../../index.md#dwidx)

  Get the attribute name.

- <span id="nameattribute-form"></span>`fn form(&self) -> constants::DwForm` — [`DwForm`](../../index.md#dwform)

  Get the attribute form.

- <span id="nameattribute-value"></span>`fn value(&self) -> &NameAttributeValue<R>` — [`NameAttributeValue`](../index.md#nameattributevalue)

  Get the attribute value.

  

  Interpretation of this value depends on the name and form.

- <span id="nameattribute-compile-unit"></span>`fn compile_unit(&self, names: &NameIndex<R>) -> Result<DebugInfoOffset<<R as >::Offset>>` — [`NameIndex`](../index.md#nameindex), [`Result`](../../index.md#result), [`DebugInfoOffset`](../../index.md#debuginfooffset), [`Reader`](../index.md#reader)

  Get the value of a `DW_IDX_compile_unit` attribute.

- <span id="nameattribute-type-unit"></span>`fn type_unit(&self, names: &NameIndex<R>) -> Result<NameTypeUnit<<R as >::Offset>>` — [`NameIndex`](../index.md#nameindex), [`Result`](../../index.md#result), [`NameTypeUnit`](../index.md#nametypeunit), [`Reader`](../index.md#reader)

  Get the value of a `DW_IDX_type_unit` attribute.

- <span id="nameattribute-die-offset"></span>`fn die_offset(&self) -> Result<UnitOffset<<R as >::Offset>>` — [`Result`](../../index.md#result), [`UnitOffset`](../../index.md#unitoffset), [`Reader`](../index.md#reader)

  Get the value of a `DW_IDX_die_offset` attribute.

- <span id="nameattribute-parent"></span>`fn parent(&self) -> Result<Option<NameEntryOffset<<R as >::Offset>>>` — [`Result`](../../index.md#result), [`NameEntryOffset`](../index.md#nameentryoffset), [`Reader`](../index.md#reader)

  Get the value of a `DW_IDX_parent` attribute.

  

  Returns `Ok(Some(offset))` if the DIE parent is indexed.

  Returns `Ok(None)` if the DIE parent is not indexed.

- <span id="nameattribute-type-hash"></span>`fn type_hash(&self) -> Result<u64>` — [`Result`](../../index.md#result)

  Get the value of a `DW_IDX_type_hash` attribute.

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for NameAttribute<R>`

- <span id="nameattribute-clone"></span>`fn clone(&self) -> NameAttribute<R>` — [`NameAttribute`](../index.md#nameattribute)

##### `impl<R: marker::Copy + Reader> Copy for NameAttribute<R>`

##### `impl<R: fmt::Debug + Reader> Debug for NameAttribute<R>`

- <span id="nameattribute-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: cmp::Eq + Reader> Eq for NameAttribute<R>`

##### `impl<R: cmp::PartialEq + Reader> PartialEq for NameAttribute<R>`

- <span id="nameattribute-partialeq-eq"></span>`fn eq(&self, other: &NameAttribute<R>) -> bool` — [`NameAttribute`](../index.md#nameattribute)

##### `impl<R: Reader> StructuralPartialEq for NameAttribute<R>`

### `NameAbbreviations`

```rust
struct NameAbbreviations {
    abbreviations: alloc::vec::Vec<NameAbbreviation>,
}
```

A table of name entry abbreviations.

#### Fields

- **`abbreviations`**: `alloc::vec::Vec<NameAbbreviation>`

  The abbreviations in this table.

#### Implementations

- <span id="nameabbreviations-get"></span>`fn get(&self, code: u64) -> Option<&NameAbbreviation>` — [`NameAbbreviation`](../index.md#nameabbreviation)

  Get an abbreviation by its code.

- <span id="nameabbreviations-abbreviations"></span>`fn abbreviations(&self) -> &[NameAbbreviation]` — [`NameAbbreviation`](../index.md#nameabbreviation)

  Get all abbreviations.

- <span id="nameabbreviations-parse"></span>`fn parse<R: Reader>(reader: R) -> Result<NameAbbreviations>` — [`Result`](../../index.md#result), [`NameAbbreviations`](../index.md#nameabbreviations)

  Parse the abbreviation table from a reader.

#### Trait Implementations

##### `impl Clone for NameAbbreviations`

- <span id="nameabbreviations-clone"></span>`fn clone(&self) -> NameAbbreviations` — [`NameAbbreviations`](../index.md#nameabbreviations)

##### `impl Debug for NameAbbreviations`

- <span id="nameabbreviations-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for NameAbbreviations`

- <span id="nameabbreviations-default"></span>`fn default() -> NameAbbreviations` — [`NameAbbreviations`](../index.md#nameabbreviations)

### `NameAbbreviation`

```rust
struct NameAbbreviation {
    code: u64,
    tag: constants::DwTag,
    attributes: alloc::vec::Vec<NameAbbreviationAttribute>,
}
```

A name abbreviation entry defines how name entries are encoded.

#### Fields

- **`code`**: `u64`

  The abbreviation code.

- **`tag`**: `constants::DwTag`

  The DIE tag.

- **`attributes`**: `alloc::vec::Vec<NameAbbreviationAttribute>`

  The list of attribute specifications.

#### Implementations

- <span id="nameabbreviation-code"></span>`fn code(&self) -> u64`

  Get the abbreviation code.

- <span id="nameabbreviation-tag"></span>`fn tag(&self) -> constants::DwTag` — [`DwTag`](../../index.md#dwtag)

  Get the DIE tag.

- <span id="nameabbreviation-attributes"></span>`fn attributes(&self) -> &[NameAbbreviationAttribute]` — [`NameAbbreviationAttribute`](../index.md#nameabbreviationattribute)

  Get the attribute specifications.

#### Trait Implementations

##### `impl Clone for NameAbbreviation`

- <span id="nameabbreviation-clone"></span>`fn clone(&self) -> NameAbbreviation` — [`NameAbbreviation`](../index.md#nameabbreviation)

##### `impl Debug for NameAbbreviation`

- <span id="nameabbreviation-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `NameAbbreviationAttribute`

```rust
struct NameAbbreviationAttribute {
    name: constants::DwIdx,
    form: constants::DwForm,
}
```

An attribute specification in a name abbreviation.

#### Fields

- **`name`**: `constants::DwIdx`

  The attribute name (index type).

- **`form`**: `constants::DwForm`

  The attribute form.

#### Implementations

- <span id="nameabbreviationattribute-name"></span>`fn name(&self) -> constants::DwIdx` — [`DwIdx`](../../index.md#dwidx)

  Get the attribute name (index type).

- <span id="nameabbreviationattribute-form"></span>`fn form(&self) -> constants::DwForm` — [`DwForm`](../../index.md#dwform)

  Get the attribute form.

#### Trait Implementations

##### `impl Clone for NameAbbreviationAttribute`

- <span id="nameabbreviationattribute-clone"></span>`fn clone(&self) -> NameAbbreviationAttribute` — [`NameAbbreviationAttribute`](../index.md#nameabbreviationattribute)

##### `impl Debug for NameAbbreviationAttribute`

- <span id="nameabbreviationattribute-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Enums

### `NameTypeUnit<T>`

```rust
enum NameTypeUnit<T> {
    Local(crate::common::DebugInfoOffset<T>),
    Foreign(crate::common::DebugTypeSignature),
}
```

A reference to a type unit.

This is the result of looking up a type unit index obtained from a `DW_IDX_type_unit`
attribute.

#### Variants

- **`Local`**

  The offset of a local type unit in the `.debug_info` section.

- **`Foreign`**

  The type signature of a foreign type unit.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for NameTypeUnit<T>`

- <span id="nametypeunit-clone"></span>`fn clone(&self) -> NameTypeUnit<T>` — [`NameTypeUnit`](../index.md#nametypeunit)

##### `impl<T: marker::Copy> Copy for NameTypeUnit<T>`

##### `impl<T: fmt::Debug> Debug for NameTypeUnit<T>`

- <span id="nametypeunit-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for NameTypeUnit<T>`

##### `impl<T: cmp::PartialEq> PartialEq for NameTypeUnit<T>`

- <span id="nametypeunit-partialeq-eq"></span>`fn eq(&self, other: &NameTypeUnit<T>) -> bool` — [`NameTypeUnit`](../index.md#nametypeunit)

##### `impl<T> StructuralPartialEq for NameTypeUnit<T>`

### `NameAttributeValue<R: Reader>`

```rust
enum NameAttributeValue<R: Reader> {
    Unsigned(u64),
    Offset(<R as >::Offset),
    Flag(bool),
}
```

A parsed attribute value for a [`NameEntry`](../index.md).

#### Variants

- **`Unsigned`**

  An unsigned integer.
  
  This can be from the following forms:
  `DW_FORM_data1`, `DW_FORM_data2`, `DW_FORM_data4`, `DW_FORM_data8`, `DW_FORM_udata`

- **`Offset`**

  An offset within a DWARF section or part thereof.
  
  This can be from the following forms:
  `DW_FORM_ref1`, `DW_FORM_ref2`, `DW_FORM_ref4`, `DW_FORM_ref8`, `DW_FORM_ref_udata`

- **`Flag`**

  A boolean flag.
  
  This can be from the following forms:
  `DW_FORM_flag`, `DW_FORM_flag_present`

#### Trait Implementations

##### `impl<R: clone::Clone + Reader> Clone for NameAttributeValue<R>`

- <span id="nameattributevalue-clone"></span>`fn clone(&self) -> NameAttributeValue<R>` — [`NameAttributeValue`](../index.md#nameattributevalue)

##### `impl<R: marker::Copy + Reader> Copy for NameAttributeValue<R>`

##### `impl<R: fmt::Debug + Reader> Debug for NameAttributeValue<R>`

- <span id="nameattributevalue-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<R: cmp::Eq + Reader> Eq for NameAttributeValue<R>`

##### `impl<R: cmp::PartialEq + Reader> PartialEq for NameAttributeValue<R>`

- <span id="nameattributevalue-partialeq-eq"></span>`fn eq(&self, other: &NameAttributeValue<R>) -> bool` — [`NameAttributeValue`](../index.md#nameattributevalue)

##### `impl<R: Reader> StructuralPartialEq for NameAttributeValue<R>`

## Functions

### `read_debug_names_form_value`

```rust
fn read_debug_names_form_value<R: Reader>(input: &mut R, form: constants::DwForm) -> crate::read::Result<NameAttributeValue<R>>
```

Read an attribute value.

This handles the subset of DWARF forms used in `.debug_names` entry pools
(`DW_IDX_*` attributes).

