**object > read > any**

# Module: read::any

## Contents

**Structs**

- [`Comdat`](#comdat) - A COMDAT section group in a [`File`].
- [`ComdatIterator`](#comdatiterator) - An iterator for the COMDAT section groups in a [`File`].
- [`ComdatSectionIterator`](#comdatsectioniterator) - An iterator for the sections in a [`Comdat`].
- [`DynamicRelocationIterator`](#dynamicrelocationiterator) - An iterator for the dynamic relocation entries in a [`File`].
- [`Section`](#section) - A section in a [`File`].
- [`SectionIterator`](#sectioniterator) - An iterator for the sections in a [`File`].
- [`SectionRelocationIterator`](#sectionrelocationiterator) - An iterator for the relocation entries in a [`Section`].
- [`Segment`](#segment) - A loadable segment in a [`File`].
- [`SegmentIterator`](#segmentiterator) - An iterator for the loadable segments in a [`File`].
- [`Symbol`](#symbol) - An symbol in a [`SymbolTable`].
- [`SymbolIterator`](#symboliterator) - An iterator for the symbols in a [`SymbolTable`].
- [`SymbolTable`](#symboltable) - A symbol table in a [`File`].

**Enums**

- [`File`](#file) - An object file that can be any supported file format.

---

## object::read::any::Comdat

*Struct*

A COMDAT section group in a [`File`].

Most functionality is provided by the [`ObjectComdat`] trait implementation.

**Generic Parameters:**
- 'data
- 'file
- R

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **ObjectComdat**
  - `fn kind(self: &Self) -> ComdatKind`
  - `fn symbol(self: &Self) -> SymbolIndex`
  - `fn name_bytes(self: &Self) -> Result<&'data [u8]>`
  - `fn name(self: &Self) -> Result<&'data str>`
  - `fn sections(self: &Self) -> ComdatSectionIterator<'data, 'file, R>`



## object::read::any::ComdatIterator

*Struct*

An iterator for the COMDAT section groups in a [`File`].

**Generic Parameters:**
- 'data
- 'file
- R

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## object::read::any::ComdatSectionIterator

*Struct*

An iterator for the sections in a [`Comdat`].

**Generic Parameters:**
- 'data
- 'file
- R

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## object::read::any::DynamicRelocationIterator

*Struct*

An iterator for the dynamic relocation entries in a [`File`].

**Generic Parameters:**
- 'data
- 'file
- R

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## object::read::any::File

*Enum*

An object file that can be any supported file format.

Most functionality is provided by the [`Object`] trait implementation.

**Generic Parameters:**
- 'data
- R

**Variants:**
- `Coff(coff::CoffFile<'data, R>)`
- `CoffBig(coff::CoffBigFile<'data, R>)`
- `Elf32(elf::ElfFile32<'data, crate::endian::Endianness, R>)`
- `Elf64(elf::ElfFile64<'data, crate::endian::Endianness, R>)`
- `MachO32(macho::MachOFile32<'data, crate::endian::Endianness, R>)`
- `MachO64(macho::MachOFile64<'data, crate::endian::Endianness, R>)`
- `Pe32(pe::PeFile32<'data, R>)`
- `Pe64(pe::PeFile64<'data, R>)`
- `Wasm(wasm::WasmFile<'data, R>)`
- `Xcoff32(xcoff::XcoffFile32<'data, R>)`
- `Xcoff64(xcoff::XcoffFile64<'data, R>)`

**Methods:**

- `fn parse(data: R) -> Result<Self>` - Parse the raw file data.
- `fn parse_dyld_cache_image<'cache, E>(image: &macho::DyldCacheImage<'data, 'cache, E, R>) -> Result<Self>` - Parse a Mach-O image from the dyld shared cache.
- `fn format(self: &Self) -> BinaryFormat` - Return the file format.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Object**
  - `fn architecture(self: &Self) -> Architecture`
  - `fn sub_architecture(self: &Self) -> Option<SubArchitecture>`
  - `fn is_little_endian(self: &Self) -> bool`
  - `fn is_64(self: &Self) -> bool`
  - `fn kind(self: &Self) -> ObjectKind`
  - `fn segments(self: &Self) -> SegmentIterator<'data, R>`
  - `fn section_by_name_bytes<'file>(self: &'file Self, section_name: &[u8]) -> Option<Section<'data, 'file, R>>`
  - `fn section_by_index(self: &Self, index: SectionIndex) -> Result<Section<'data, R>>`
  - `fn sections(self: &Self) -> SectionIterator<'data, R>`
  - `fn comdats(self: &Self) -> ComdatIterator<'data, R>`
  - `fn symbol_by_index(self: &Self, index: SymbolIndex) -> Result<Symbol<'data, R>>`
  - `fn symbols(self: &Self) -> SymbolIterator<'data, R>`
  - `fn symbol_table(self: &Self) -> Option<SymbolTable<'data, R>>`
  - `fn dynamic_symbols(self: &Self) -> SymbolIterator<'data, R>`
  - `fn dynamic_symbol_table(self: &Self) -> Option<SymbolTable<'data, R>>`
  - `fn dynamic_relocations(self: &Self) -> Option<DynamicRelocationIterator<'data, R>>`
  - `fn symbol_map(self: &Self) -> SymbolMap<SymbolMapName<'data>>`
  - `fn object_map(self: &Self) -> ObjectMap<'data>`
  - `fn imports(self: &Self) -> Result<Vec<Import<'data>>>`
  - `fn exports(self: &Self) -> Result<Vec<Export<'data>>>`
  - `fn has_debug_symbols(self: &Self) -> bool`
  - `fn mach_uuid(self: &Self) -> Result<Option<[u8; 16]>>`
  - `fn build_id(self: &Self) -> Result<Option<&'data [u8]>>`
  - `fn gnu_debuglink(self: &Self) -> Result<Option<(&'data [u8], u32)>>`
  - `fn gnu_debugaltlink(self: &Self) -> Result<Option<(&'data [u8], &'data [u8])>>`
  - `fn pdb_info(self: &Self) -> Result<Option<CodeView>>`
  - `fn relative_address_base(self: &Self) -> u64`
  - `fn entry(self: &Self) -> u64`
  - `fn flags(self: &Self) -> FileFlags`



## object::read::any::Section

*Struct*

A section in a [`File`].

Most functionality is provided by the [`ObjectSection`] trait implementation.

**Generic Parameters:**
- 'data
- 'file
- R

**Trait Implementations:**

- **ObjectSection**
  - `fn index(self: &Self) -> SectionIndex`
  - `fn address(self: &Self) -> u64`
  - `fn size(self: &Self) -> u64`
  - `fn align(self: &Self) -> u64`
  - `fn file_range(self: &Self) -> Option<(u64, u64)>`
  - `fn data(self: &Self) -> Result<&'data [u8]>`
  - `fn data_range(self: &Self, address: u64, size: u64) -> Result<Option<&'data [u8]>>`
  - `fn compressed_file_range(self: &Self) -> Result<CompressedFileRange>`
  - `fn compressed_data(self: &Self) -> Result<CompressedData<'data>>`
  - `fn name_bytes(self: &Self) -> Result<&'data [u8]>`
  - `fn name(self: &Self) -> Result<&'data str>`
  - `fn segment_name_bytes(self: &Self) -> Result<Option<&[u8]>>`
  - `fn segment_name(self: &Self) -> Result<Option<&str>>`
  - `fn kind(self: &Self) -> SectionKind`
  - `fn relocations(self: &Self) -> SectionRelocationIterator<'data, 'file, R>`
  - `fn relocation_map(self: &Self) -> Result<RelocationMap>`
  - `fn flags(self: &Self) -> SectionFlags`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## object::read::any::SectionIterator

*Struct*

An iterator for the sections in a [`File`].

**Generic Parameters:**
- 'data
- 'file
- R

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## object::read::any::SectionRelocationIterator

*Struct*

An iterator for the relocation entries in a [`Section`].

**Generic Parameters:**
- 'data
- 'file
- R

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## object::read::any::Segment

*Struct*

A loadable segment in a [`File`].

Most functionality is provided by the [`ObjectSegment`] trait implementation.

**Generic Parameters:**
- 'data
- 'file
- R

**Trait Implementations:**

- **ObjectSegment**
  - `fn address(self: &Self) -> u64`
  - `fn size(self: &Self) -> u64`
  - `fn align(self: &Self) -> u64`
  - `fn file_range(self: &Self) -> (u64, u64)`
  - `fn data(self: &Self) -> Result<&'data [u8]>`
  - `fn data_range(self: &Self, address: u64, size: u64) -> Result<Option<&'data [u8]>>`
  - `fn name_bytes(self: &Self) -> Result<Option<&[u8]>>`
  - `fn name(self: &Self) -> Result<Option<&str>>`
  - `fn flags(self: &Self) -> SegmentFlags`
- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## object::read::any::SegmentIterator

*Struct*

An iterator for the loadable segments in a [`File`].

**Generic Parameters:**
- 'data
- 'file
- R

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## object::read::any::Symbol

*Struct*

An symbol in a [`SymbolTable`].

Most functionality is provided by the [`ObjectSymbol`] trait implementation.

**Generic Parameters:**
- 'data
- 'file
- R

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **ObjectSymbol**
  - `fn index(self: &Self) -> SymbolIndex`
  - `fn name_bytes(self: &Self) -> Result<&'data [u8]>`
  - `fn name(self: &Self) -> Result<&'data str>`
  - `fn address(self: &Self) -> u64`
  - `fn size(self: &Self) -> u64`
  - `fn kind(self: &Self) -> SymbolKind`
  - `fn section(self: &Self) -> SymbolSection`
  - `fn is_undefined(self: &Self) -> bool`
  - `fn is_definition(self: &Self) -> bool`
  - `fn is_common(self: &Self) -> bool`
  - `fn is_weak(self: &Self) -> bool`
  - `fn scope(self: &Self) -> SymbolScope`
  - `fn is_global(self: &Self) -> bool`
  - `fn is_local(self: &Self) -> bool`
  - `fn flags(self: &Self) -> SymbolFlags<SectionIndex, SymbolIndex>`



## object::read::any::SymbolIterator

*Struct*

An iterator for the symbols in a [`SymbolTable`].

**Generic Parameters:**
- 'data
- 'file
- R

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## object::read::any::SymbolTable

*Struct*

A symbol table in a [`File`].

Most functionality is provided by the [`ObjectSymbolTable`] trait implementation.

**Generic Parameters:**
- 'data
- 'file
- R

**Trait Implementations:**

- **ObjectSymbolTable**
  - `fn symbols(self: &Self) -> <Self as >::SymbolIterator`
  - `fn symbol_by_index(self: &Self, index: SymbolIndex) -> Result<<Self as >::Symbol>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



