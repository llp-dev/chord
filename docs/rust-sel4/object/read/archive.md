**object > read > archive**

# Module: read::archive

## Contents

**Structs**

- [`ArchiveFile`](#archivefile) - A partially parsed archive file.
- [`ArchiveMember`](#archivemember) - A partially parsed archive member.
- [`ArchiveMemberIterator`](#archivememberiterator) - An iterator over the members of an archive.
- [`ArchiveOffset`](#archiveoffset) - An offset of a member in an archive.
- [`ArchiveSymbol`](#archivesymbol) - A symbol in the archive symbol table.
- [`ArchiveSymbolIterator`](#archivesymboliterator) - An iterator over the symbols in the archive symbol table.

**Enums**

- [`ArchiveKind`](#archivekind) - The kind of archive format.

---

## object::read::archive::ArchiveFile

*Struct*

A partially parsed archive file.

**Generic Parameters:**
- 'data
- R

**Methods:**

- `fn parse(data: R) -> read::Result<Self>` - Parse the archive header and special members.
- `fn kind(self: &Self) -> ArchiveKind` - Return the archive format.
- `fn is_thin(self: &Self) -> bool` - Return true if the archive is a thin archive.
- `fn members(self: &Self) -> ArchiveMemberIterator<'data, R>` - Iterate over the members of the archive.
- `fn member(self: &Self, member: ArchiveOffset) -> read::Result<ArchiveMember<'data>>` - Return the member at the given offset.
- `fn symbols(self: &Self) -> read::Result<Option<ArchiveSymbolIterator<'data>>>` - Iterate over the symbols in the archive.

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ArchiveFile<'data, R>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::read::archive::ArchiveKind

*Enum*

The kind of archive format.

**Variants:**
- `Unknown` - There are no special files that indicate the archive format.
- `Gnu` - The GNU (or System V) archive format.
- `Gnu64` - The GNU (or System V) archive format with 64-bit symbol table.
- `Bsd` - The BSD archive format.
- `Bsd64` - The BSD archive format with 64-bit symbol table.
- `Coff` - The Windows COFF archive format.
- `AixBig` - The AIX big archive format.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &ArchiveKind) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> ArchiveKind`



## object::read::archive::ArchiveMember

*Struct*

A partially parsed archive member.

**Generic Parameters:**
- 'data

**Methods:**

- `fn header(self: &Self) -> Option<&'data archive::Header>` - Return the raw header that is common to many archive formats.
- `fn aix_header(self: &Self) -> Option<&'data archive::AixHeader>` - Return the raw header for AIX big archives.
- `fn name(self: &Self) -> &'data [u8]` - Return the parsed file name.
- `fn date(self: &Self) -> Option<u64>` - Parse the file modification timestamp from the header.
- `fn uid(self: &Self) -> Option<u64>` - Parse the user ID from the header.
- `fn gid(self: &Self) -> Option<u64>` - Parse the group ID from the header.
- `fn mode(self: &Self) -> Option<u64>` - Parse the file mode from the header.
- `fn size(self: &Self) -> u64` - Return the size of the file data.
- `fn file_range(self: &Self) -> (u64, u64)` - Return the offset and size of the file data.
- `fn is_thin(self: &Self) -> bool` - Return true if the member is a thin member.
- `fn data<R>(self: &Self, data: R) -> read::Result<&'data [u8]>` - Return the file data.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::read::archive::ArchiveMemberIterator

*Struct*

An iterator over the members of an archive.

**Generic Parameters:**
- 'data
- R

**Trait Implementations:**

- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::read::archive::ArchiveOffset

*Struct*

An offset of a member in an archive.

**Tuple Struct**: `(u64)`

**Traits:** Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ArchiveOffset`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::read::archive::ArchiveSymbol

*Struct*

A symbol in the archive symbol table.

This is used to find the member containing the symbol.

**Generic Parameters:**
- 'data

**Methods:**

- `fn name(self: &Self) -> &'data [u8]` - Return the symbol name.
- `fn offset(self: &Self) -> ArchiveOffset` - Return the offset of the header for the member containing the symbol.

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ArchiveSymbol<'data>`



## object::read::archive::ArchiveSymbolIterator

*Struct*

An iterator over the symbols in the archive symbol table.

**Generic Parameters:**
- 'data

**Tuple Struct**: `()`

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> ArchiveSymbolIterator<'data>`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`
  - `fn size_hint(self: &Self) -> (usize, Option<usize>)`



