**object > read > macho > file**

# Module: read::macho::file

## Contents

**Structs**

- [`MachOComdat`](#machocomdat) - A COMDAT section group in a [`MachOFile`].
- [`MachOComdatIterator`](#machocomdatiterator) - An iterator for the COMDAT section groups in a [`MachOFile`].
- [`MachOComdatSectionIterator`](#machocomdatsectioniterator) - An iterator for the sections in a COMDAT section group in a [`MachOFile`].
- [`MachOFile`](#machofile) - A partially parsed Mach-O file.

**Traits**

- [`MachHeader`](#machheader) - A trait for generic access to [`macho::MachHeader32`] and [`macho::MachHeader64`].

**Type Aliases**

- [`MachOComdat32`](#machocomdat32) - A COMDAT section group in a [`MachOFile32`].
- [`MachOComdat64`](#machocomdat64) - A COMDAT section group in a [`MachOFile64`].
- [`MachOComdatIterator32`](#machocomdatiterator32) - An iterator for the COMDAT section groups in a [`MachOFile64`].
- [`MachOComdatIterator64`](#machocomdatiterator64) - An iterator for the COMDAT section groups in a [`MachOFile64`].
- [`MachOComdatSectionIterator32`](#machocomdatsectioniterator32) - An iterator for the sections in a COMDAT section group in a [`MachOFile32`].
- [`MachOComdatSectionIterator64`](#machocomdatsectioniterator64) - An iterator for the sections in a COMDAT section group in a [`MachOFile64`].
- [`MachOFile32`](#machofile32) - A 32-bit Mach-O object file.
- [`MachOFile64`](#machofile64) - A 64-bit Mach-O object file.

---

## object::read::macho::file::MachHeader

*Trait*

A trait for generic access to [`macho::MachHeader32`] and [`macho::MachHeader64`].

**Methods:**

- `Word`
- `Endian`
- `Segment`
- `Section`
- `Nlist`
- `is_type_64`: Return true if this type is a 64-bit header.
- `is_big_endian`: Return true if the `magic` field signifies big-endian.
- `is_little_endian`: Return true if the `magic` field signifies little-endian.
- `magic`
- `cputype`
- `cpusubtype`
- `filetype`
- `ncmds`
- `sizeofcmds`
- `flags`
- `parse`: Read the file header.
- `is_supported`
- `endian`
- `load_commands`
- `uuid`: Return the UUID from the `LC_UUID` load command, if one is present.



## object::read::macho::file::MachOComdat

*Struct*

A COMDAT section group in a [`MachOFile`].

This is a stub that doesn't implement any functionality.

**Generic Parameters:**
- 'data
- 'file
- Mach
- R

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **ObjectComdat**
  - `fn kind(self: &Self) -> ComdatKind`
  - `fn symbol(self: &Self) -> SymbolIndex`
  - `fn name_bytes(self: &Self) -> Result<&'data [u8]>`
  - `fn name(self: &Self) -> Result<&'data str>`
  - `fn sections(self: &Self) -> <Self as >::SectionIterator`



## object::read::macho::file::MachOComdat32

*Type Alias*: `MachOComdat<'data, 'file, macho::MachHeader32<Endian>, R>`

A COMDAT section group in a [`MachOFile32`].



## object::read::macho::file::MachOComdat64

*Type Alias*: `MachOComdat<'data, 'file, macho::MachHeader64<Endian>, R>`

A COMDAT section group in a [`MachOFile64`].



## object::read::macho::file::MachOComdatIterator

*Struct*

An iterator for the COMDAT section groups in a [`MachOFile`].

This is a stub that doesn't implement any functionality.

**Generic Parameters:**
- 'data
- 'file
- Mach
- R

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## object::read::macho::file::MachOComdatIterator32

*Type Alias*: `MachOComdatIterator<'data, 'file, macho::MachHeader32<Endian>, R>`

An iterator for the COMDAT section groups in a [`MachOFile64`].



## object::read::macho::file::MachOComdatIterator64

*Type Alias*: `MachOComdatIterator<'data, 'file, macho::MachHeader64<Endian>, R>`

An iterator for the COMDAT section groups in a [`MachOFile64`].



## object::read::macho::file::MachOComdatSectionIterator

*Struct*

An iterator for the sections in a COMDAT section group in a [`MachOFile`].

This is a stub that doesn't implement any functionality.

**Generic Parameters:**
- 'data
- 'file
- Mach
- R

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## object::read::macho::file::MachOComdatSectionIterator32

*Type Alias*: `MachOComdatSectionIterator<'data, 'file, macho::MachHeader32<Endian>, R>`

An iterator for the sections in a COMDAT section group in a [`MachOFile32`].



## object::read::macho::file::MachOComdatSectionIterator64

*Type Alias*: `MachOComdatSectionIterator<'data, 'file, macho::MachHeader64<Endian>, R>`

An iterator for the sections in a COMDAT section group in a [`MachOFile64`].



## object::read::macho::file::MachOFile

*Struct*

A partially parsed Mach-O file.

Most of the functionality of this type is provided by the [`Object`] trait implementation.

**Generic Parameters:**
- 'data
- Mach
- R

**Methods:**

- `fn parse(data: R) -> Result<Self>` - Parse the raw Mach-O file data.
- `fn parse_dyld_cache_image<'cache, E>(image: &DyldCacheImage<'data, 'cache, E, R>) -> Result<Self>` - Parse the Mach-O file for the given image from the dyld shared cache.
- `fn endian(self: &Self) -> <Mach as >::Endian` - Returns the endianness.
- `fn data(self: &Self) -> R` - Returns the raw data.
- `fn raw_header(self: &Self) -> &'data Mach` - Returns the raw Mach-O file header.
- `fn macho_header(self: &Self) -> &'data Mach` - Get the raw Mach-O file header.
- `fn macho_load_commands(self: &Self) -> Result<LoadCommandIterator<'data, <Mach as >::Endian>>` - Get the Mach-O load commands.
- `fn macho_symbol_table(self: &Self) -> &SymbolTable<'data, Mach, R>` - Get the Mach-O symbol table.
- `fn build_version(self: &Self) -> Result<Option<&'data macho::BuildVersionCommand<<Mach as >::Endian>>>` - Return the `LC_BUILD_VERSION` load command if present.

**Trait Implementations:**

- **Object**
  - `fn architecture(self: &Self) -> Architecture`
  - `fn sub_architecture(self: &Self) -> Option<SubArchitecture>`
  - `fn is_little_endian(self: &Self) -> bool`
  - `fn is_64(self: &Self) -> bool`
  - `fn kind(self: &Self) -> ObjectKind`
  - `fn segments(self: &Self) -> MachOSegmentIterator<'data, Mach, R>`
  - `fn section_by_name_bytes<'file>(self: &'file Self, section_name: &[u8]) -> Option<MachOSection<'data, 'file, Mach, R>>`
  - `fn section_by_index(self: &Self, index: SectionIndex) -> Result<MachOSection<'data, Mach, R>>`
  - `fn sections(self: &Self) -> MachOSectionIterator<'data, Mach, R>`
  - `fn comdats(self: &Self) -> MachOComdatIterator<'data, Mach, R>`
  - `fn symbol_by_index(self: &Self, index: SymbolIndex) -> Result<MachOSymbol<'data, Mach, R>>`
  - `fn symbols(self: &Self) -> MachOSymbolIterator<'data, Mach, R>`
  - `fn symbol_table(self: &Self) -> Option<MachOSymbolTable<'data, Mach, R>>`
  - `fn dynamic_symbols(self: &Self) -> MachOSymbolIterator<'data, Mach, R>`
  - `fn dynamic_symbol_table(self: &Self) -> Option<MachOSymbolTable<'data, Mach, R>>`
  - `fn object_map(self: &Self) -> ObjectMap<'data>`
  - `fn imports(self: &Self) -> Result<Vec<Import<'data>>>`
  - `fn exports(self: &Self) -> Result<Vec<Export<'data>>>`
  - `fn dynamic_relocations(self: &Self) -> Option<NoDynamicRelocationIterator>`
  - `fn has_debug_symbols(self: &Self) -> bool`
  - `fn mach_uuid(self: &Self) -> Result<Option<[u8; 16]>>`
  - `fn relative_address_base(self: &Self) -> u64`
  - `fn entry(self: &Self) -> u64`
  - `fn flags(self: &Self) -> FileFlags`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::read::macho::file::MachOFile32

*Type Alias*: `MachOFile<'data, macho::MachHeader32<Endian>, R>`

A 32-bit Mach-O object file.

This is a file that starts with [`macho::MachHeader32`], and corresponds
to [`crate::FileKind::MachO32`].



## object::read::macho::file::MachOFile64

*Type Alias*: `MachOFile<'data, macho::MachHeader64<Endian>, R>`

A 64-bit Mach-O object file.

This is a file that starts with [`macho::MachHeader64`], and corresponds
to [`crate::FileKind::MachO64`].



