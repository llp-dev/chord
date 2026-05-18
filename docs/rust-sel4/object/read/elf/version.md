**object > read > elf > version**

# Module: read::elf::version

## Contents

**Structs**

- [`VerdauxIterator`](#verdauxiterator) - An iterator for the auxiliary records for an entry in an ELF [`elf::SHT_GNU_VERDEF`] section.
- [`VerdefIterator`](#verdefiterator) - An iterator for the entries in an ELF [`elf::SHT_GNU_VERDEF`] section.
- [`VernauxIterator`](#vernauxiterator) - An iterator for the auxiliary records for an entry in an ELF [`elf::SHT_GNU_VERNEED`] section.
- [`VerneedIterator`](#verneediterator) - An iterator for the entries in an ELF [`elf::SHT_GNU_VERNEED`] section.
- [`Version`](#version) - A version definition or requirement.
- [`VersionIndex`](#versionindex) - A version index.
- [`VersionTable`](#versiontable) - A table of version definitions and requirements.

---

## object::read::elf::version::VerdauxIterator

*Struct*

An iterator for the auxiliary records for an entry in an ELF [`elf::SHT_GNU_VERDEF`] section.

**Generic Parameters:**
- 'data
- Elf

**Methods:**

- `fn next(self: & mut Self) -> Result<Option<&'data elf::Verdaux<<Elf as >::Endian>>>` - Return the next `Verdaux` entry.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> VerdauxIterator<'data, Elf>`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## object::read::elf::version::VerdefIterator

*Struct*

An iterator for the entries in an ELF [`elf::SHT_GNU_VERDEF`] section.

**Generic Parameters:**
- 'data
- Elf

**Methods:**

- `fn next(self: & mut Self) -> Result<Option<(&'data elf::Verdef<<Elf as >::Endian>, VerdauxIterator<'data, Elf>)>>` - Return the next `Verdef` entry.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> VerdefIterator<'data, Elf>`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## object::read::elf::version::VernauxIterator

*Struct*

An iterator for the auxiliary records for an entry in an ELF [`elf::SHT_GNU_VERNEED`] section.

**Generic Parameters:**
- 'data
- Elf

**Methods:**

- `fn next(self: & mut Self) -> Result<Option<&'data elf::Vernaux<<Elf as >::Endian>>>` - Return the next `Vernaux` entry.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> VernauxIterator<'data, Elf>`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## object::read::elf::version::VerneedIterator

*Struct*

An iterator for the entries in an ELF [`elf::SHT_GNU_VERNEED`] section.

**Generic Parameters:**
- 'data
- Elf

**Methods:**

- `fn next(self: & mut Self) -> Result<Option<(&'data elf::Verneed<<Elf as >::Endian>, VernauxIterator<'data, Elf>)>>` - Return the next `Verneed` entry.

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> VerneedIterator<'data, Elf>`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::read::elf::version::Version

*Struct*

A version definition or requirement.

This is derived from entries in the [`elf::SHT_GNU_VERDEF`] and [`elf::SHT_GNU_VERNEED`] sections.

**Generic Parameters:**
- 'data

**Methods:**

- `fn name(self: &Self) -> &'data [u8]` - Return the version name.
- `fn hash(self: &Self) -> u32` - Return hash of the version name.
- `fn file(self: &Self) -> Option<&'data [u8]>` - Return the filename of the library containing this version.

**Traits:** Copy

**Trait Implementations:**

- **Default**
  - `fn default() -> Version<'data>`
- **Clone**
  - `fn clone(self: &Self) -> Version<'data>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::read::elf::version::VersionIndex

*Struct*

A version index.

**Tuple Struct**: `(u16)`

**Methods:**

- `fn index(self: &Self) -> u16` - Return the version index.
- `fn is_local(self: &Self) -> bool` - Return true if it is the local index.
- `fn is_global(self: &Self) -> bool` - Return true if it is the global index.
- `fn is_hidden(self: &Self) -> bool` - Return the hidden flag.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> VersionIndex`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> VersionIndex`



## object::read::elf::version::VersionTable

*Struct*

A table of version definitions and requirements.

It allows looking up the version information for a given symbol index.

This is derived from entries in the [`elf::SHT_GNU_VERSYM`], [`elf::SHT_GNU_VERDEF`]
and [`elf::SHT_GNU_VERNEED`] sections.

Returned by [`SectionTable::versions`](super::SectionTable::versions).

**Generic Parameters:**
- 'data
- Elf

**Methods:**

- `fn parse<R>(endian: <Elf as >::Endian, versyms: &'data [elf::Versym<<Elf as >::Endian>], verdefs: Option<VerdefIterator<'data, Elf>>, verneeds: Option<VerneedIterator<'data, Elf>>, strings: StringTable<'data, R>) -> Result<Self>` - Parse the version sections.
- `fn is_empty(self: &Self) -> bool` - Return true if the version table is empty.
- `fn version_index(self: &Self, endian: <Elf as >::Endian, index: SymbolIndex) -> VersionIndex` - Return version index for a given symbol index.
- `fn version(self: &Self, index: VersionIndex) -> Result<Option<&Version<'data>>>` - Return version information for a given symbol version index.
- `fn matches(self: &Self, endian: <Elf as >::Endian, index: SymbolIndex, need: Option<&Version>) -> bool` - Return true if the given symbol index satisfies the requirements of `need`.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> VersionTable<'data, Elf>`
- **Default**
  - `fn default() -> Self`



