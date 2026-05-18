*[object](../../../index.md) / [read](../../index.md) / [elf](../index.md) / [version](index.md)*

---

# Module `version`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`VersionIndex`](#versionindex) | struct | A version index. |
| [`Version`](#version) | struct | A version definition or requirement. |
| [`VersionTable`](#versiontable) | struct | A table of version definitions and requirements. |
| [`VerdefIterator`](#verdefiterator) | struct | An iterator for the entries in an ELF [`elf::SHT_GNU_VERDEF`] section. |
| [`VerdauxIterator`](#verdauxiterator) | struct | An iterator for the auxiliary records for an entry in an ELF [`elf::SHT_GNU_VERDEF`] section. |
| [`VerneedIterator`](#verneediterator) | struct | An iterator for the entries in an ELF [`elf::SHT_GNU_VERNEED`] section. |
| [`VernauxIterator`](#vernauxiterator) | struct | An iterator for the auxiliary records for an entry in an ELF [`elf::SHT_GNU_VERNEED`] section. |

## Structs

### `VersionIndex`

```rust
struct VersionIndex(u16);
```

A version index.

#### Implementations

- <span id="versionindex-index"></span>`fn index(&self) -> u16`

  Return the version index.

- <span id="versionindex-is-local"></span>`fn is_local(&self) -> bool`

  Return true if it is the local index.

- <span id="versionindex-is-global"></span>`fn is_global(&self) -> bool`

  Return true if it is the global index.

- <span id="versionindex-is-hidden"></span>`fn is_hidden(&self) -> bool`

  Return the hidden flag.

#### Trait Implementations

##### `impl Clone for VersionIndex`

- <span id="versionindex-clone"></span>`fn clone(&self) -> VersionIndex` — [`VersionIndex`](../index.md#versionindex)

##### `impl Copy for VersionIndex`

##### `impl Debug for VersionIndex`

- <span id="versionindex-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for VersionIndex`

- <span id="versionindex-default"></span>`fn default() -> VersionIndex` — [`VersionIndex`](../index.md#versionindex)

### `Version<'data>`

```rust
struct Version<'data> {
    name: &'data [u8],
    hash: u32,
    valid: bool,
    file: Option<&'data [u8]>,
}
```

A version definition or requirement.

This is derived from entries in the [`elf::SHT_GNU_VERDEF`](../../../elf/index.md) and [`elf::SHT_GNU_VERNEED`](../../../elf/index.md) sections.

#### Implementations

- <span id="version-name"></span>`fn name(&self) -> &'data [u8]`

  Return the version name.

- <span id="version-hash"></span>`fn hash(&self) -> u32`

  Return hash of the version name.

- <span id="version-file"></span>`fn file(&self) -> Option<&'data [u8]>`

  Return the filename of the library containing this version.

  

  This is the `vn_file` field of the associated entry in [`elf::SHT_GNU_VERNEED`](../../../elf/index.md).

  or `None` if the version info was parsed from a [`elf::SHT_GNU_VERDEF`](../../../elf/index.md) section.

#### Trait Implementations

##### `impl Clone for Version<'data>`

- <span id="version-clone"></span>`fn clone(&self) -> Version<'data>` — [`Version`](../index.md#version)

##### `impl Copy for Version<'data>`

##### `impl Debug for Version<'data>`

- <span id="version-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Version<'data>`

- <span id="version-default"></span>`fn default() -> Version<'data>` — [`Version`](../index.md#version)

### `VersionTable<'data, Elf: FileHeader>`

```rust
struct VersionTable<'data, Elf: FileHeader> {
    symbols: &'data [elf::Versym<<Elf as >::Endian>],
    versions: alloc::vec::Vec<Version<'data>>,
}
```

A table of version definitions and requirements.

It allows looking up the version information for a given symbol index.

This is derived from entries in the [`elf::SHT_GNU_VERSYM`](../../../elf/index.md), [`elf::SHT_GNU_VERDEF`](../../../elf/index.md)
and [`elf::SHT_GNU_VERNEED`](../../../elf/index.md) sections.

Returned by [`SectionTable::versions`](super::SectionTable::versions).

#### Implementations

- <span id="versiontable-parse"></span>`fn parse<R: ReadRef<'data>>(endian: <Elf as >::Endian, versyms: &'data [elf::Versym<<Elf as >::Endian>], verdefs: Option<VerdefIterator<'data, Elf>>, verneeds: Option<VerneedIterator<'data, Elf>>, strings: StringTable<'data, R>) -> Result<Self>` — [`FileHeader`](../index.md#fileheader), [`Versym`](../../../elf/index.md#versym), [`VerdefIterator`](../index.md#verdefiterator), [`VerneedIterator`](../index.md#verneediterator), [`StringTable`](../../index.md#stringtable), [`Result`](../../../index.md#result)

  Parse the version sections.

- <span id="versiontable-is-empty"></span>`fn is_empty(&self) -> bool`

  Return true if the version table is empty.

- <span id="versiontable-version-index"></span>`fn version_index(&self, endian: <Elf as >::Endian, index: SymbolIndex) -> VersionIndex` — [`FileHeader`](../index.md#fileheader), [`SymbolIndex`](../../../index.md#symbolindex), [`VersionIndex`](../index.md#versionindex)

  Return version index for a given symbol index.

- <span id="versiontable-version"></span>`fn version(&self, index: VersionIndex) -> Result<Option<&Version<'data>>>` — [`VersionIndex`](../index.md#versionindex), [`Result`](../../../index.md#result), [`Version`](../index.md#version)

  Return version information for a given symbol version index.

  

  Returns `Ok(None)` for local and global versions.

  Returns `Err(_)` if index is invalid.

- <span id="versiontable-matches"></span>`fn matches(&self, endian: <Elf as >::Endian, index: SymbolIndex, need: Option<&Version<'_>>) -> bool` — [`FileHeader`](../index.md#fileheader), [`SymbolIndex`](../../../index.md#symbolindex), [`Version`](../index.md#version)

  Return true if the given symbol index satisfies the requirements of `need`.

  

  Returns false for any error.

  

  Note: this function hasn't been fully tested and is likely to be incomplete.

#### Trait Implementations

##### `impl<Elf: clone::Clone + FileHeader> Clone for VersionTable<'data, Elf>`

- <span id="versiontable-clone"></span>`fn clone(&self) -> VersionTable<'data, Elf>` — [`VersionTable`](../index.md#versiontable)

##### `impl<Elf: fmt::Debug + FileHeader> Debug for VersionTable<'data, Elf>`

- <span id="versiontable-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Elf: FileHeader> Default for VersionTable<'data, Elf>`

- <span id="versiontable-default"></span>`fn default() -> Self`

### `VerdefIterator<'data, Elf: FileHeader>`

```rust
struct VerdefIterator<'data, Elf: FileHeader> {
    endian: <Elf as >::Endian,
    data: crate::read::Bytes<'data>,
}
```

An iterator for the entries in an ELF [`elf::SHT_GNU_VERDEF`](../../../elf/index.md) section.

#### Implementations

- <span id="verdefiterator-new"></span>`fn new(endian: <Elf as >::Endian, data: &'data [u8]) -> Self` — [`FileHeader`](../index.md#fileheader)

- <span id="verdefiterator-next"></span>`fn next(&mut self) -> Result<Option<(&'data elf::Verdef<<Elf as >::Endian>, VerdauxIterator<'data, Elf>)>>` — [`Result`](../../../index.md#result), [`Verdef`](../../../elf/index.md#verdef), [`FileHeader`](../index.md#fileheader), [`VerdauxIterator`](../index.md#verdauxiterator)

  Return the next `Verdef` entry.

- <span id="verdefiterator-parse"></span>`fn parse(&mut self) -> Result<(&'data elf::Verdef<<Elf as >::Endian>, VerdauxIterator<'data, Elf>)>` — [`Result`](../../../index.md#result), [`Verdef`](../../../elf/index.md#verdef), [`FileHeader`](../index.md#fileheader), [`VerdauxIterator`](../index.md#verdauxiterator)

#### Trait Implementations

##### `impl<Elf: clone::Clone + FileHeader> Clone for VerdefIterator<'data, Elf>`

- <span id="verdefiterator-clone"></span>`fn clone(&self) -> VerdefIterator<'data, Elf>` — [`VerdefIterator`](../index.md#verdefiterator)

##### `impl<Elf: fmt::Debug + FileHeader> Debug for VerdefIterator<'data, Elf>`

- <span id="verdefiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for VerdefIterator<'data, Elf>`

- <span id="verdefiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="verdefiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="verdefiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Elf: FileHeader> Iterator for VerdefIterator<'data, Elf>`

- <span id="verdefiterator-iterator-type-item"></span>`type Item = Result<(&'data Verdef<<Elf as FileHeader>::Endian>, VerdauxIterator<'data, Elf>), Error>`

- <span id="verdefiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `VerdauxIterator<'data, Elf: FileHeader>`

```rust
struct VerdauxIterator<'data, Elf: FileHeader> {
    endian: <Elf as >::Endian,
    data: crate::read::Bytes<'data>,
    count: u16,
}
```

An iterator for the auxiliary records for an entry in an ELF [`elf::SHT_GNU_VERDEF`](../../../elf/index.md) section.

#### Implementations

- <span id="verdauxiterator-new"></span>`fn new(endian: <Elf as >::Endian, data: &'data [u8], count: u16) -> Self` — [`FileHeader`](../index.md#fileheader)

- <span id="verdauxiterator-next"></span>`fn next(&mut self) -> Result<Option<&'data elf::Verdaux<<Elf as >::Endian>>>` — [`Result`](../../../index.md#result), [`Verdaux`](../../../elf/index.md#verdaux), [`FileHeader`](../index.md#fileheader)

  Return the next `Verdaux` entry.

- <span id="verdauxiterator-parse"></span>`fn parse(&mut self) -> Result<&'data elf::Verdaux<<Elf as >::Endian>>` — [`Result`](../../../index.md#result), [`Verdaux`](../../../elf/index.md#verdaux), [`FileHeader`](../index.md#fileheader)

#### Trait Implementations

##### `impl<Elf: clone::Clone + FileHeader> Clone for VerdauxIterator<'data, Elf>`

- <span id="verdauxiterator-clone"></span>`fn clone(&self) -> VerdauxIterator<'data, Elf>` — [`VerdauxIterator`](../index.md#verdauxiterator)

##### `impl<Elf: fmt::Debug + FileHeader> Debug for VerdauxIterator<'data, Elf>`

- <span id="verdauxiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for VerdauxIterator<'data, Elf>`

- <span id="verdauxiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="verdauxiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="verdauxiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Elf: FileHeader> Iterator for VerdauxIterator<'data, Elf>`

- <span id="verdauxiterator-iterator-type-item"></span>`type Item = Result<&'data Verdaux<<Elf as FileHeader>::Endian>, Error>`

- <span id="verdauxiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `VerneedIterator<'data, Elf: FileHeader>`

```rust
struct VerneedIterator<'data, Elf: FileHeader> {
    endian: <Elf as >::Endian,
    data: crate::read::Bytes<'data>,
}
```

An iterator for the entries in an ELF [`elf::SHT_GNU_VERNEED`](../../../elf/index.md) section.

#### Implementations

- <span id="verneediterator-new"></span>`fn new(endian: <Elf as >::Endian, data: &'data [u8]) -> Self` — [`FileHeader`](../index.md#fileheader)

- <span id="verneediterator-next"></span>`fn next(&mut self) -> Result<Option<(&'data elf::Verneed<<Elf as >::Endian>, VernauxIterator<'data, Elf>)>>` — [`Result`](../../../index.md#result), [`Verneed`](../../../elf/index.md#verneed), [`FileHeader`](../index.md#fileheader), [`VernauxIterator`](../index.md#vernauxiterator)

  Return the next `Verneed` entry.

- <span id="verneediterator-parse"></span>`fn parse(&mut self) -> Result<(&'data elf::Verneed<<Elf as >::Endian>, VernauxIterator<'data, Elf>)>` — [`Result`](../../../index.md#result), [`Verneed`](../../../elf/index.md#verneed), [`FileHeader`](../index.md#fileheader), [`VernauxIterator`](../index.md#vernauxiterator)

#### Trait Implementations

##### `impl<Elf: clone::Clone + FileHeader> Clone for VerneedIterator<'data, Elf>`

- <span id="verneediterator-clone"></span>`fn clone(&self) -> VerneedIterator<'data, Elf>` — [`VerneedIterator`](../index.md#verneediterator)

##### `impl<Elf: fmt::Debug + FileHeader> Debug for VerneedIterator<'data, Elf>`

- <span id="verneediterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for VerneedIterator<'data, Elf>`

- <span id="verneediterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="verneediterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="verneediterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Elf: FileHeader> Iterator for VerneedIterator<'data, Elf>`

- <span id="verneediterator-iterator-type-item"></span>`type Item = Result<(&'data Verneed<<Elf as FileHeader>::Endian>, VernauxIterator<'data, Elf>), Error>`

- <span id="verneediterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

### `VernauxIterator<'data, Elf: FileHeader>`

```rust
struct VernauxIterator<'data, Elf: FileHeader> {
    endian: <Elf as >::Endian,
    data: crate::read::Bytes<'data>,
    count: u16,
}
```

An iterator for the auxiliary records for an entry in an ELF [`elf::SHT_GNU_VERNEED`](../../../elf/index.md) section.

#### Implementations

- <span id="vernauxiterator-new"></span>`fn new(endian: <Elf as >::Endian, data: &'data [u8], count: u16) -> Self` — [`FileHeader`](../index.md#fileheader)

- <span id="vernauxiterator-next"></span>`fn next(&mut self) -> Result<Option<&'data elf::Vernaux<<Elf as >::Endian>>>` — [`Result`](../../../index.md#result), [`Vernaux`](../../../elf/index.md#vernaux), [`FileHeader`](../index.md#fileheader)

  Return the next `Vernaux` entry.

- <span id="vernauxiterator-parse"></span>`fn parse(&mut self) -> Result<&'data elf::Vernaux<<Elf as >::Endian>>` — [`Result`](../../../index.md#result), [`Vernaux`](../../../elf/index.md#vernaux), [`FileHeader`](../index.md#fileheader)

#### Trait Implementations

##### `impl<Elf: clone::Clone + FileHeader> Clone for VernauxIterator<'data, Elf>`

- <span id="vernauxiterator-clone"></span>`fn clone(&self) -> VernauxIterator<'data, Elf>` — [`VernauxIterator`](../index.md#vernauxiterator)

##### `impl<Elf: fmt::Debug + FileHeader> Debug for VernauxIterator<'data, Elf>`

- <span id="vernauxiterator-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl IntoIterator for VernauxIterator<'data, Elf>`

- <span id="vernauxiterator-intoiterator-type-item"></span>`type Item = <I as Iterator>::Item`

- <span id="vernauxiterator-intoiterator-type-intoiter"></span>`type IntoIter = I`

- <span id="vernauxiterator-intoiterator-into-iter"></span>`fn into_iter(self) -> I`

##### `impl<Elf: FileHeader> Iterator for VernauxIterator<'data, Elf>`

- <span id="vernauxiterator-iterator-type-item"></span>`type Item = Result<&'data Vernaux<<Elf as FileHeader>::Endian>, Error>`

- <span id="vernauxiterator-iterator-next"></span>`fn next(&mut self) -> Option<<Self as >::Item>`

