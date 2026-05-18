*[object](../index.md) / [write](index.md)*

---

# Module `write`

Interface for writing object files.

This module provides a unified write API for relocatable object files
using [`Object`](#object). This does not support writing executable files.
This supports the following file formats: COFF, ELF, Mach-O, and XCOFF.

The submodules define helpers for writing the raw structs. These support
writing both relocatable and executable files. There are writers for
the following file formats: [COFF](coff::Writer), [ELF](elf::Writer),
and [PE](pe::Writer).

## Contents

- [Modules](#modules)
  - [`coff`](#coff)
  - [`elf`](#elf)
  - [`macho`](#macho)
  - [`pe`](#pe)
  - [`xcoff`](#xcoff)
  - [`string`](#string)
  - [`util`](#util)
- [Structs](#structs)
  - [`MachOBuildVersion`](#machobuildversion)
  - [`StringId`](#stringid)
  - [`Error`](#error)
  - [`Object`](#object)
  - [`SectionId`](#sectionid)
  - [`Section`](#section)
  - [`SymbolId`](#symbolid)
  - [`Symbol`](#symbol)
  - [`Relocation`](#relocation)
  - [`ComdatId`](#comdatid)
  - [`Comdat`](#comdat)
  - [`StreamingBuffer`](#streamingbuffer)
- [Enums](#enums)
  - [`CoffExportStyle`](#coffexportstyle)
  - [`StandardSegment`](#standardsegment)
  - [`StandardSection`](#standardsection)
  - [`SymbolSection`](#symbolsection)
  - [`Mangling`](#mangling)
  - [`Architecture`](#architecture)
  - [`SubArchitecture`](#subarchitecture)
  - [`AddressSize`](#addresssize)
  - [`BinaryFormat`](#binaryformat)
  - [`SectionKind`](#sectionkind)
  - [`ComdatKind`](#comdatkind)
  - [`SymbolKind`](#symbolkind)
  - [`SymbolScope`](#symbolscope)
  - [`RelocationKind`](#relocationkind)
  - [`RelocationEncoding`](#relocationencoding)
  - [`FileFlags`](#fileflags)
  - [`SegmentFlags`](#segmentflags)
  - [`SectionFlags`](#sectionflags)
  - [`SymbolFlags`](#symbolflags)
  - [`RelocationFlags`](#relocationflags)
- [Traits](#traits)
  - [`WritableBuffer`](#writablebuffer)
  - [`BytesMut`](#bytesmut)
- [Functions](#functions)
  - [`write_uleb128`](#write-uleb128)
  - [`write_sleb128`](#write-sleb128)
  - [`align`](#align)
  - [`align_u32`](#align-u32)
  - [`align_u64`](#align-u64)
  - [`write_align`](#write-align)
- [Type Aliases](#type-aliases)
  - [`Result`](#result)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`coff`](#coff) | mod | Support for writing COFF files. |
| [`elf`](#elf) | mod | Support for writing ELF files. |
| [`macho`](#macho) | mod |  |
| [`pe`](#pe) | mod | Helper for writing PE files. |
| [`xcoff`](#xcoff) | mod |  |
| [`string`](#string) | mod |  |
| [`util`](#util) | mod |  |
| [`MachOBuildVersion`](#machobuildversion) | struct |  |
| [`StringId`](#stringid) | struct |  |
| [`Error`](#error) | struct | The error type used within the write module. |
| [`Object`](#object) | struct | A writable relocatable object file. |
| [`SectionId`](#sectionid) | struct | An identifier used to reference a section. |
| [`Section`](#section) | struct | A section in an object file. |
| [`SymbolId`](#symbolid) | struct | An identifier used to reference a symbol. |
| [`Symbol`](#symbol) | struct | A symbol in an object file. |
| [`Relocation`](#relocation) | struct | A relocation in an object file. |
| [`ComdatId`](#comdatid) | struct | An identifier used to reference a COMDAT section group. |
| [`Comdat`](#comdat) | struct | A COMDAT section group. |
| [`StreamingBuffer`](#streamingbuffer) | struct | A [`WritableBuffer`] that streams data to a [`Write`](std::io::Write) implementation. |
| [`CoffExportStyle`](#coffexportstyle) | enum |  |
| [`StandardSegment`](#standardsegment) | enum | A standard segment kind. |
| [`StandardSection`](#standardsection) | enum | A standard section kind. |
| [`SymbolSection`](#symbolsection) | enum | The section where a symbol is defined. |
| [`Mangling`](#mangling) | enum | The symbol name mangling scheme. |
| [`Architecture`](#architecture) | enum | A CPU architecture. |
| [`SubArchitecture`](#subarchitecture) | enum | A CPU sub-architecture. |
| [`AddressSize`](#addresssize) | enum | The size of an address value for an architecture. |
| [`BinaryFormat`](#binaryformat) | enum | A binary file format. |
| [`SectionKind`](#sectionkind) | enum | The kind of a section. |
| [`ComdatKind`](#comdatkind) | enum | The selection kind for a COMDAT section group. |
| [`SymbolKind`](#symbolkind) | enum | The kind of a symbol. |
| [`SymbolScope`](#symbolscope) | enum | A symbol scope. |
| [`RelocationKind`](#relocationkind) | enum | The operation used to calculate the result of the relocation. |
| [`RelocationEncoding`](#relocationencoding) | enum | Information about how the result of the relocation operation is encoded in the place. |
| [`FileFlags`](#fileflags) | enum | File flags that are specific to each file format. |
| [`SegmentFlags`](#segmentflags) | enum | Segment flags that are specific to each file format. |
| [`SectionFlags`](#sectionflags) | enum | Section flags that are specific to each file format. |
| [`SymbolFlags`](#symbolflags) | enum | Symbol flags that are specific to each file format. |
| [`RelocationFlags`](#relocationflags) | enum | Relocation fields that are specific to each file format and architecture. |
| [`WritableBuffer`](#writablebuffer) | trait | Trait for writable buffer. |
| [`BytesMut`](#bytesmut) | trait | A trait for mutable byte slices. |
| [`write_uleb128`](#write-uleb128) | fn | Write an unsigned number using the LEB128 encoding to a buffer. |
| [`write_sleb128`](#write-sleb128) | fn | Write a signed number using the LEB128 encoding to a buffer. |
| [`align`](#align) | fn |  |
| [`align_u32`](#align-u32) | fn |  |
| [`align_u64`](#align-u64) | fn |  |
| [`write_align`](#write-align) | fn |  |
| [`Result`](#result) | type | The result type used within the write module. |

## Modules

- [`coff`](coff/index.md) — Support for writing COFF files.
- [`elf`](elf/index.md) — Support for writing ELF files.
- [`macho`](macho/index.md)
- [`pe`](pe/index.md) — Helper for writing PE files.
- [`xcoff`](xcoff/index.md)
- [`string`](string/index.md)
- [`util`](util/index.md)

## Structs

### `MachOBuildVersion`

```rust
struct MachOBuildVersion {
    pub platform: u32,
    pub minos: u32,
    pub sdk: u32,
}
```

The customizable portion of a [`macho::BuildVersionCommand`](../macho/index.md).

#### Fields

- **`platform`**: `u32`

  One of the `PLATFORM_` constants (for example,
  [`object::macho::PLATFORM_MACOS`](macho::PLATFORM_MACOS)).

- **`minos`**: `u32`

  The minimum OS version, where `X.Y.Z` is encoded in nibbles as
  `xxxx.yy.zz`.

- **`sdk`**: `u32`

  The SDK version as `X.Y.Z`, where `X.Y.Z` is encoded in nibbles as
  `xxxx.yy.zz`.

#### Implementations

- <span id="machobuildversion-cmdsize"></span>`fn cmdsize(&self) -> u32`

#### Trait Implementations

##### `impl Clone for MachOBuildVersion`

- <span id="machobuildversion-clone"></span>`fn clone(&self) -> MachOBuildVersion` — [`MachOBuildVersion`](macho/index.md#machobuildversion)

##### `impl Copy for MachOBuildVersion`

##### `impl Debug for MachOBuildVersion`

- <span id="machobuildversion-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for MachOBuildVersion`

- <span id="machobuildversion-default"></span>`fn default() -> MachOBuildVersion` — [`MachOBuildVersion`](macho/index.md#machobuildversion)

### `StringId`

```rust
struct StringId(usize);
```

An identifier for an entry in a string table.

#### Trait Implementations

##### `impl Clone for StringId`

- <span id="stringid-clone"></span>`fn clone(&self) -> StringId` — [`StringId`](string/index.md#stringid)

##### `impl Copy for StringId`

##### `impl Debug for StringId`

- <span id="stringid-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for StringId`

##### `impl<K> Equivalent for StringId`

- <span id="stringid-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl PartialEq for StringId`

- <span id="stringid-partialeq-eq"></span>`fn eq(&self, other: &StringId) -> bool` — [`StringId`](string/index.md#stringid)

##### `impl StructuralPartialEq for StringId`

### `Error`

```rust
struct Error(alloc::string::String);
```

The error type used within the write module.

#### Trait Implementations

##### `impl Clone for Error`

- <span id="error-clone"></span>`fn clone(&self) -> Error` — [`Error`](#error)

##### `impl Debug for Error`

- <span id="error-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for Error`

- <span id="error-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Error`

##### `impl<K> Equivalent for Error`

- <span id="error-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Error for Error`

##### `impl PartialEq for Error`

- <span id="error-partialeq-eq"></span>`fn eq(&self, other: &Error) -> bool` — [`Error`](#error)

##### `impl StructuralPartialEq for Error`

##### `impl ToString for Error`

- <span id="error-tostring-to-string"></span>`fn to_string(&self) -> String`

### `Object<'a>`

```rust
struct Object<'a> {
    format: BinaryFormat,
    architecture: Architecture,
    sub_architecture: Option<SubArchitecture>,
    endian: crate::endian::Endianness,
    sections: alloc::vec::Vec<Section<'a>>,
    standard_sections: std::collections::HashMap<StandardSection, SectionId>,
    symbols: alloc::vec::Vec<Symbol>,
    symbol_map: std::collections::HashMap<alloc::vec::Vec<u8>, SymbolId>,
    comdats: alloc::vec::Vec<Comdat>,
    pub flags: FileFlags,
    pub mangling: Mangling,
    stub_symbols: std::collections::HashMap<SymbolId, SymbolId>,
    tlv_bootstrap: Option<SymbolId>,
    macho_cpu_subtype: Option<u32>,
    macho_build_version: Option<MachOBuildVersion>,
    macho_subsections_via_symbols: bool,
}
```

A writable relocatable object file.

#### Fields

- **`flags`**: `FileFlags`

  File flags that are specific to each file format.

- **`mangling`**: `Mangling`

  The symbol name mangling scheme.

- **`tlv_bootstrap`**: `Option<SymbolId>`

  Mach-O "_tlv_bootstrap" symbol.

- **`macho_cpu_subtype`**: `Option<u32>`

  Mach-O CPU subtype.

- **`macho_subsections_via_symbols`**: `bool`

  Mach-O MH_SUBSECTIONS_VIA_SYMBOLS flag. Only ever set if format is Mach-O.

#### Implementations

- <span id="object-coff-section-info"></span>`fn coff_section_info(&self, section: StandardSection) -> (&'static [u8], &'static [u8], SectionKind, SectionFlags)` — [`StandardSection`](#standardsection), [`SectionKind`](../index.md#sectionkind), [`SectionFlags`](../index.md#sectionflags)

- <span id="object-coff-subsection-name"></span>`fn coff_subsection_name(&self, section: &[u8], value: &[u8]) -> Vec<u8>`

- <span id="object-coff-section-flags"></span>`fn coff_section_flags(&self, section: &Section<'_>) -> SectionFlags` — [`Section`](#section), [`SectionFlags`](../index.md#sectionflags)

- <span id="object-coff-symbol-flags"></span>`fn coff_symbol_flags(&self, _symbol: &Symbol) -> SymbolFlags<SectionId, SymbolId>` — [`Symbol`](#symbol), [`SymbolFlags`](../index.md#symbolflags), [`SectionId`](#sectionid), [`SymbolId`](#symbolid)

- <span id="object-coff-translate-relocation"></span>`fn coff_translate_relocation(&mut self, reloc: &mut Relocation) -> Result<()>` — [`Relocation`](#relocation), [`Result`](#result)

- <span id="object-coff-adjust-addend"></span>`fn coff_adjust_addend(&self, relocation: &mut Relocation) -> Result<bool>` — [`Relocation`](#relocation), [`Result`](#result)

- <span id="object-coff-relocation-size"></span>`fn coff_relocation_size(&self, reloc: &Relocation) -> Result<u8>` — [`Relocation`](#relocation), [`Result`](#result)

- <span id="object-coff-add-stub-symbol"></span>`fn coff_add_stub_symbol(&mut self, symbol_id: SymbolId) -> Result<SymbolId>` — [`SymbolId`](#symbolid), [`Result`](#result)

- <span id="object-add-coff-exports"></span>`fn add_coff_exports(&mut self, style: CoffExportStyle)` — [`CoffExportStyle`](coff/index.md#coffexportstyle)

  Appends linker directives to the `.drectve` section to tell the linker

  to export all symbols with `SymbolScope::Dynamic`.

  

  This must be called after all symbols have been defined.

- <span id="object-coff-write"></span>`fn coff_write(&self, buffer: &mut dyn WritableBuffer) -> Result<()>` — [`WritableBuffer`](#writablebuffer), [`Result`](#result)

#### Trait Implementations

##### `impl Debug for Object<'a>`

- <span id="object-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `SectionId`

```rust
struct SectionId(usize);
```

An identifier used to reference a section.

#### Trait Implementations

##### `impl Clone for SectionId`

- <span id="sectionid-clone"></span>`fn clone(&self) -> SectionId` — [`SectionId`](#sectionid)

##### `impl<K> Comparable for SectionId`

- <span id="sectionid-comparable-compare"></span>`fn compare(&self, key: &K) -> Ordering`

##### `impl Copy for SectionId`

##### `impl Debug for SectionId`

- <span id="sectionid-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SectionId`

##### `impl<K> Equivalent for SectionId`

- <span id="sectionid-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Hash for SectionId`

- <span id="sectionid-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for SectionId`

- <span id="sectionid-ord-cmp"></span>`fn cmp(&self, other: &SectionId) -> cmp::Ordering` — [`SectionId`](#sectionid)

##### `impl PartialEq for SectionId`

- <span id="sectionid-partialeq-eq"></span>`fn eq(&self, other: &SectionId) -> bool` — [`SectionId`](#sectionid)

##### `impl PartialOrd for SectionId`

- <span id="sectionid-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &SectionId) -> option::Option<cmp::Ordering>` — [`SectionId`](#sectionid)

##### `impl StructuralPartialEq for SectionId`

### `Section<'a>`

```rust
struct Section<'a> {
    segment: alloc::vec::Vec<u8>,
    name: alloc::vec::Vec<u8>,
    kind: SectionKind,
    size: u64,
    align: u64,
    data: alloc::borrow::Cow<'a, [u8]>,
    relocations: alloc::vec::Vec<Relocation>,
    symbol: Option<SymbolId>,
    pub flags: SectionFlags,
}
```

A section in an object file.

#### Fields

- **`flags`**: `SectionFlags`

  Section flags that are specific to each file format.

#### Implementations

- <span id="section-name"></span>`fn name(&self) -> Option<&str>`

  Try to convert the name to a utf8 string.

- <span id="section-segment"></span>`fn segment(&self) -> Option<&str>`

  Try to convert the segment to a utf8 string.

- <span id="section-is-bss"></span>`fn is_bss(&self) -> bool`

  Return true if this section contains zerofill data.

- <span id="section-set-data"></span>`fn set_data<T>(&mut self, data: T, align: u64)`

  Set the data for a section.

  

  Must not be called for sections that already have data, or that contain uninitialized data.

  `align` must be a power of two.

- <span id="section-append-data"></span>`fn append_data(&mut self, append_data: &[u8], align: u64) -> u64`

  Append data to a section.

  

  Must not be called for sections that contain uninitialized data.

  `align` must be a power of two.

- <span id="section-append-bss"></span>`fn append_bss(&mut self, size: u64, align: u64) -> u64`

  Append uninitialized data to a section.

  

  Must not be called for sections that contain initialized data.

  `align` must be a power of two.

- <span id="section-data"></span>`fn data(&self) -> &[u8]`

  Returns the section as-built so far.

  

  This requires that the section is not a bss section.

- <span id="section-data-mut"></span>`fn data_mut(&mut self) -> &mut [u8]`

  Returns the section as-built so far.

  

  This requires that the section is not a bss section.

#### Trait Implementations

##### `impl Debug for Section<'a>`

- <span id="section-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `SymbolId`

```rust
struct SymbolId(usize);
```

An identifier used to reference a symbol.

#### Trait Implementations

##### `impl Clone for SymbolId`

- <span id="symbolid-clone"></span>`fn clone(&self) -> SymbolId` — [`SymbolId`](#symbolid)

##### `impl<K> Comparable for SymbolId`

- <span id="symbolid-comparable-compare"></span>`fn compare(&self, key: &K) -> Ordering`

##### `impl Copy for SymbolId`

##### `impl Debug for SymbolId`

- <span id="symbolid-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SymbolId`

##### `impl<K> Equivalent for SymbolId`

- <span id="symbolid-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Hash for SymbolId`

- <span id="symbolid-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for SymbolId`

- <span id="symbolid-ord-cmp"></span>`fn cmp(&self, other: &SymbolId) -> cmp::Ordering` — [`SymbolId`](#symbolid)

##### `impl PartialEq for SymbolId`

- <span id="symbolid-partialeq-eq"></span>`fn eq(&self, other: &SymbolId) -> bool` — [`SymbolId`](#symbolid)

##### `impl PartialOrd for SymbolId`

- <span id="symbolid-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &SymbolId) -> option::Option<cmp::Ordering>` — [`SymbolId`](#symbolid)

##### `impl StructuralPartialEq for SymbolId`

### `Symbol`

```rust
struct Symbol {
    pub name: alloc::vec::Vec<u8>,
    pub value: u64,
    pub size: u64,
    pub kind: SymbolKind,
    pub scope: SymbolScope,
    pub weak: bool,
    pub section: SymbolSection,
    pub flags: SymbolFlags<SectionId, SymbolId>,
}
```

A symbol in an object file.

#### Fields

- **`name`**: `alloc::vec::Vec<u8>`

  The name of the symbol.

- **`value`**: `u64`

  The value of the symbol.
  
  If the symbol defined in a section, then this is the section offset of the symbol.

- **`size`**: `u64`

  The size of the symbol.

- **`kind`**: `SymbolKind`

  The kind of the symbol.

- **`scope`**: `SymbolScope`

  The scope of the symbol.

- **`weak`**: `bool`

  Whether the symbol has weak binding.

- **`section`**: `SymbolSection`

  The section containing the symbol.

- **`flags`**: `SymbolFlags<SectionId, SymbolId>`

  Symbol flags that are specific to each file format.

#### Implementations

- <span id="symbol-name"></span>`fn name(&self) -> Option<&str>`

  Try to convert the name to a utf8 string.

- <span id="symbol-is-undefined"></span>`fn is_undefined(&self) -> bool`

  Return true if the symbol is undefined.

- <span id="symbol-is-common"></span>`fn is_common(&self) -> bool`

  Return true if the symbol is common data.

  

  Note: does not check for `SymbolSection::Section` with `SectionKind::Common`.

- <span id="symbol-is-local"></span>`fn is_local(&self) -> bool`

  Return true if the symbol scope is local.

#### Trait Implementations

##### `impl Debug for Symbol`

- <span id="symbol-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `Relocation`

```rust
struct Relocation {
    pub offset: u64,
    pub symbol: SymbolId,
    pub addend: i64,
    pub flags: RelocationFlags,
}
```

A relocation in an object file.

#### Fields

- **`offset`**: `u64`

  The section offset of the place of the relocation.

- **`symbol`**: `SymbolId`

  The symbol referred to by the relocation.
  
  This may be a section symbol.

- **`addend`**: `i64`

  The addend to use in the relocation calculation.
  
  This may be in addition to an implicit addend stored at the place of the relocation.

- **`flags`**: `RelocationFlags`

  The fields that define the relocation type.

#### Trait Implementations

##### `impl Debug for Relocation`

- <span id="relocation-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ComdatId`

```rust
struct ComdatId(usize);
```

An identifier used to reference a COMDAT section group.

#### Trait Implementations

##### `impl Clone for ComdatId`

- <span id="comdatid-clone"></span>`fn clone(&self) -> ComdatId` — [`ComdatId`](#comdatid)

##### `impl<K> Comparable for ComdatId`

- <span id="comdatid-comparable-compare"></span>`fn compare(&self, key: &K) -> Ordering`

##### `impl Copy for ComdatId`

##### `impl Debug for ComdatId`

- <span id="comdatid-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ComdatId`

##### `impl<K> Equivalent for ComdatId`

- <span id="comdatid-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Hash for ComdatId`

- <span id="comdatid-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for ComdatId`

- <span id="comdatid-ord-cmp"></span>`fn cmp(&self, other: &ComdatId) -> cmp::Ordering` — [`ComdatId`](#comdatid)

##### `impl PartialEq for ComdatId`

- <span id="comdatid-partialeq-eq"></span>`fn eq(&self, other: &ComdatId) -> bool` — [`ComdatId`](#comdatid)

##### `impl PartialOrd for ComdatId`

- <span id="comdatid-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &ComdatId) -> option::Option<cmp::Ordering>` — [`ComdatId`](#comdatid)

##### `impl StructuralPartialEq for ComdatId`

### `Comdat`

```rust
struct Comdat {
    pub kind: ComdatKind,
    pub symbol: SymbolId,
    pub sections: alloc::vec::Vec<SectionId>,
}
```

A COMDAT section group.

#### Fields

- **`kind`**: `ComdatKind`

  The COMDAT selection kind.
  
  This determines the way in which the linker resolves multiple definitions of the COMDAT
  sections.

- **`symbol`**: `SymbolId`

  The COMDAT symbol.
  
  If this symbol is referenced, then all sections in the group will be included by the
  linker.

- **`sections`**: `alloc::vec::Vec<SectionId>`

  The sections in the group.

#### Trait Implementations

##### `impl Debug for Comdat`

- <span id="comdat-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `StreamingBuffer<W>`

```rust
struct StreamingBuffer<W> {
    writer: W,
    len: usize,
    result: Result<(), io::Error>,
}
```

A [`WritableBuffer`](#writablebuffer) that streams data to a [`Write`](std::io::Write) implementation.

`Self::result` must be called to determine if an I/O error occurred during writing.
Alternatively, `Self::flush` will both check for errors and flush.

It is advisable to use a buffered writer like [`BufWriter`](std::io::BufWriter)
instead of an unbuffered writer like [`File`](std::fs::File).

#### Implementations

- <span id="streamingbuffer-new"></span>`fn new(writer: W) -> Self`

  Create a new `StreamingBuffer` backed by the given writer.

- <span id="streamingbuffer-into-inner"></span>`fn into_inner(self) -> W`

  Unwraps this [`StreamingBuffer`](#streamingbuffer) giving back the original writer.

- <span id="streamingbuffer-result"></span>`fn result(&mut self) -> Result<(), io::Error>`

  Returns any error that occurred during writing.

#### Trait Implementations

##### `impl<W: fmt::Debug> Debug for StreamingBuffer<W>`

- <span id="streamingbuffer-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<W: io::Write> WritableBuffer for StreamingBuffer<W>`

- <span id="streamingbuffer-writablebuffer-len"></span>`fn len(&self) -> usize`

- <span id="streamingbuffer-writablebuffer-reserve"></span>`fn reserve(&mut self, _size: usize) -> Result<(), ()>`

- <span id="streamingbuffer-writablebuffer-resize"></span>`fn resize(&mut self, new_len: usize)`

- <span id="streamingbuffer-writablebuffer-write-bytes"></span>`fn write_bytes(&mut self, val: &[u8])`

## Enums

### `CoffExportStyle`

```rust
enum CoffExportStyle {
    Msvc,
    Gnu,
}
```

Internal format to use for the `.drectve` section containing linker
directives for symbol exports.

#### Variants

- **`Msvc`**

  MSVC format supported by link.exe and LLD.

- **`Gnu`**

  Gnu format supported by GNU LD and LLD.

#### Trait Implementations

##### `impl Clone for CoffExportStyle`

- <span id="coffexportstyle-clone"></span>`fn clone(&self) -> CoffExportStyle` — [`CoffExportStyle`](coff/index.md#coffexportstyle)

##### `impl Copy for CoffExportStyle`

##### `impl Debug for CoffExportStyle`

- <span id="coffexportstyle-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for CoffExportStyle`

##### `impl<K> Equivalent for CoffExportStyle`

- <span id="coffexportstyle-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl PartialEq for CoffExportStyle`

- <span id="coffexportstyle-partialeq-eq"></span>`fn eq(&self, other: &CoffExportStyle) -> bool` — [`CoffExportStyle`](coff/index.md#coffexportstyle)

##### `impl StructuralPartialEq for CoffExportStyle`

### `StandardSegment`

```rust
enum StandardSegment {
    Text,
    Data,
    Debug,
}
```

A standard segment kind.

#### Trait Implementations

##### `impl Clone for StandardSegment`

- <span id="standardsegment-clone"></span>`fn clone(&self) -> StandardSegment` — [`StandardSegment`](#standardsegment)

##### `impl<K> Comparable for StandardSegment`

- <span id="standardsegment-comparable-compare"></span>`fn compare(&self, key: &K) -> Ordering`

##### `impl Copy for StandardSegment`

##### `impl Debug for StandardSegment`

- <span id="standardsegment-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for StandardSegment`

##### `impl<K> Equivalent for StandardSegment`

- <span id="standardsegment-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Hash for StandardSegment`

- <span id="standardsegment-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for StandardSegment`

- <span id="standardsegment-ord-cmp"></span>`fn cmp(&self, other: &StandardSegment) -> cmp::Ordering` — [`StandardSegment`](#standardsegment)

##### `impl PartialEq for StandardSegment`

- <span id="standardsegment-partialeq-eq"></span>`fn eq(&self, other: &StandardSegment) -> bool` — [`StandardSegment`](#standardsegment)

##### `impl PartialOrd for StandardSegment`

- <span id="standardsegment-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &StandardSegment) -> option::Option<cmp::Ordering>` — [`StandardSegment`](#standardsegment)

##### `impl StructuralPartialEq for StandardSegment`

### `StandardSection`

```rust
enum StandardSection {
    Text,
    Data,
    ReadOnlyData,
    ReadOnlyDataWithRel,
    ReadOnlyString,
    UninitializedData,
    Tls,
    UninitializedTls,
    TlsVariables,
    Common,
    GnuProperty,
}
```

A standard section kind.

#### Variants

- **`UninitializedTls`**

  Zero-fill TLS initializers. Unsupported for COFF.

- **`TlsVariables`**

  TLS variable structures. Only supported for Mach-O.

- **`Common`**

  Common data. Only supported for Mach-O.

- **`GnuProperty`**

  Notes for GNU properties. Only supported for ELF.

#### Implementations

- <span id="standardsection-kind"></span>`fn kind(self) -> SectionKind` — [`SectionKind`](../index.md#sectionkind)

  Return the section kind of a standard section.

- <span id="standardsection-all"></span>`fn all() -> &'static [StandardSection]` — [`StandardSection`](#standardsection)

#### Trait Implementations

##### `impl Clone for StandardSection`

- <span id="standardsection-clone"></span>`fn clone(&self) -> StandardSection` — [`StandardSection`](#standardsection)

##### `impl<K> Comparable for StandardSection`

- <span id="standardsection-comparable-compare"></span>`fn compare(&self, key: &K) -> Ordering`

##### `impl Copy for StandardSection`

##### `impl Debug for StandardSection`

- <span id="standardsection-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for StandardSection`

##### `impl<K> Equivalent for StandardSection`

- <span id="standardsection-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Hash for StandardSection`

- <span id="standardsection-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl Ord for StandardSection`

- <span id="standardsection-ord-cmp"></span>`fn cmp(&self, other: &StandardSection) -> cmp::Ordering` — [`StandardSection`](#standardsection)

##### `impl PartialEq for StandardSection`

- <span id="standardsection-partialeq-eq"></span>`fn eq(&self, other: &StandardSection) -> bool` — [`StandardSection`](#standardsection)

##### `impl PartialOrd for StandardSection`

- <span id="standardsection-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &StandardSection) -> option::Option<cmp::Ordering>` — [`StandardSection`](#standardsection)

##### `impl StructuralPartialEq for StandardSection`

### `SymbolSection`

```rust
enum SymbolSection {
    None,
    Undefined,
    Absolute,
    Common,
    Section(SectionId),
}
```

The section where a symbol is defined.

#### Variants

- **`None`**

  The section is not applicable for this symbol (such as file symbols).

- **`Undefined`**

  The symbol is undefined.

- **`Absolute`**

  The symbol has an absolute value.

- **`Common`**

  The symbol is a zero-initialized symbol that will be combined with duplicate definitions.

- **`Section`**

  The symbol is defined in the given section.

#### Implementations

- <span id="symbolsection-id"></span>`fn id(self) -> Option<SectionId>` — [`SectionId`](#sectionid)

  Returns the section id for the section where the symbol is defined.

  

  May return `None` if the symbol is not defined in a section.

#### Trait Implementations

##### `impl Clone for SymbolSection`

- <span id="symbolsection-clone"></span>`fn clone(&self) -> SymbolSection` — [`SymbolSection`](#symbolsection)

##### `impl Copy for SymbolSection`

##### `impl Debug for SymbolSection`

- <span id="symbolsection-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SymbolSection`

##### `impl<K> Equivalent for SymbolSection`

- <span id="symbolsection-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Hash for SymbolSection`

- <span id="symbolsection-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for SymbolSection`

- <span id="symbolsection-partialeq-eq"></span>`fn eq(&self, other: &SymbolSection) -> bool` — [`SymbolSection`](#symbolsection)

##### `impl StructuralPartialEq for SymbolSection`

### `Mangling`

```rust
enum Mangling {
    None,
    Coff,
    CoffI386,
    Elf,
    MachO,
    Xcoff,
}
```

The symbol name mangling scheme.

#### Variants

- **`None`**

  No symbol mangling.

- **`Coff`**

  Windows COFF symbol mangling.

- **`CoffI386`**

  Windows COFF i386 symbol mangling.

- **`Elf`**

  ELF symbol mangling.

- **`MachO`**

  Mach-O symbol mangling.

- **`Xcoff`**

  Xcoff symbol mangling.

#### Implementations

- <span id="mangling-default"></span>`fn default(format: BinaryFormat, architecture: Architecture) -> Self` — [`BinaryFormat`](../index.md#binaryformat), [`Architecture`](../index.md#architecture)

  Return the default symboling mangling for the given format and architecture.

- <span id="mangling-global-prefix"></span>`fn global_prefix(self) -> Option<u8>`

  Return the prefix to use for global symbols.

#### Trait Implementations

##### `impl Clone for Mangling`

- <span id="mangling-clone"></span>`fn clone(&self) -> Mangling` — [`Mangling`](#mangling)

##### `impl Copy for Mangling`

##### `impl Debug for Mangling`

- <span id="mangling-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Mangling`

##### `impl<K> Equivalent for Mangling`

- <span id="mangling-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Hash for Mangling`

- <span id="mangling-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Mangling`

- <span id="mangling-partialeq-eq"></span>`fn eq(&self, other: &Mangling) -> bool` — [`Mangling`](#mangling)

##### `impl StructuralPartialEq for Mangling`

### `Architecture`

```rust
enum Architecture {
    Unknown,
    Aarch64,
    Aarch64_Ilp32,
    Alpha,
    Arm,
    Avr,
    Bpf,
    Csky,
    E2K32,
    E2K64,
    I386,
    X86_64,
    X86_64_X32,
    Hexagon,
    Hppa,
    LoongArch32,
    LoongArch64,
    M68k,
    Mips,
    Mips64,
    Mips64_N32,
    Msp430,
    PowerPc,
    PowerPc64,
    Riscv32,
    Riscv64,
    S390x,
    Sbf,
    Sharc,
    Sparc,
    Sparc32Plus,
    Sparc64,
    SuperH,
    Wasm32,
    Wasm64,
    Xtensa,
}
```

A CPU architecture.

#### Implementations

- <span id="architecture-address-size"></span>`fn address_size(self) -> Option<AddressSize>` — [`AddressSize`](../index.md#addresssize)

  The size of an address value for this architecture.

  

  Returns `None` for unknown architectures.

#### Trait Implementations

##### `impl Clone for Architecture`

- <span id="architecture-clone"></span>`fn clone(&self) -> Architecture` — [`Architecture`](../index.md#architecture)

##### `impl Copy for Architecture`

##### `impl Debug for Architecture`

- <span id="architecture-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Architecture`

##### `impl<K> Equivalent for Architecture`

- <span id="architecture-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Hash for Architecture`

- <span id="architecture-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Architecture`

- <span id="architecture-partialeq-eq"></span>`fn eq(&self, other: &Architecture) -> bool` — [`Architecture`](../index.md#architecture)

##### `impl StructuralPartialEq for Architecture`

### `SubArchitecture`

```rust
enum SubArchitecture {
    Arm64E,
    Arm64EC,
}
```

A CPU sub-architecture.

#### Trait Implementations

##### `impl Clone for SubArchitecture`

- <span id="subarchitecture-clone"></span>`fn clone(&self) -> SubArchitecture` — [`SubArchitecture`](../index.md#subarchitecture)

##### `impl Copy for SubArchitecture`

##### `impl Debug for SubArchitecture`

- <span id="subarchitecture-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SubArchitecture`

##### `impl<K> Equivalent for SubArchitecture`

- <span id="subarchitecture-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Hash for SubArchitecture`

- <span id="subarchitecture-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for SubArchitecture`

- <span id="subarchitecture-partialeq-eq"></span>`fn eq(&self, other: &SubArchitecture) -> bool` — [`SubArchitecture`](../index.md#subarchitecture)

##### `impl StructuralPartialEq for SubArchitecture`

### `AddressSize`

```rust
enum AddressSize {
    U8,
    U16,
    U32,
    U64,
}
```

The size of an address value for an architecture.

This may differ from the address size supported by the file format (such as for COFF).

#### Implementations

- <span id="addresssize-bytes"></span>`fn bytes(self) -> u8`

  The size in bytes of an address value.

#### Trait Implementations

##### `impl Clone for AddressSize`

- <span id="addresssize-clone"></span>`fn clone(&self) -> AddressSize` — [`AddressSize`](../index.md#addresssize)

##### `impl Copy for AddressSize`

##### `impl Debug for AddressSize`

- <span id="addresssize-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for AddressSize`

##### `impl<K> Equivalent for AddressSize`

- <span id="addresssize-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Hash for AddressSize`

- <span id="addresssize-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for AddressSize`

- <span id="addresssize-partialeq-eq"></span>`fn eq(&self, other: &AddressSize) -> bool` — [`AddressSize`](../index.md#addresssize)

##### `impl StructuralPartialEq for AddressSize`

### `BinaryFormat`

```rust
enum BinaryFormat {
    Coff,
    Elf,
    MachO,
    Pe,
    Wasm,
    Xcoff,
}
```

A binary file format.

#### Implementations

- <span id="binaryformat-native-object"></span>`fn native_object() -> BinaryFormat` — [`BinaryFormat`](../index.md#binaryformat)

  The target's native binary format for relocatable object files.

  

  Defaults to `Elf` for unknown platforms.

#### Trait Implementations

##### `impl Clone for BinaryFormat`

- <span id="binaryformat-clone"></span>`fn clone(&self) -> BinaryFormat` — [`BinaryFormat`](../index.md#binaryformat)

##### `impl Copy for BinaryFormat`

##### `impl Debug for BinaryFormat`

- <span id="binaryformat-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for BinaryFormat`

##### `impl<K> Equivalent for BinaryFormat`

- <span id="binaryformat-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Hash for BinaryFormat`

- <span id="binaryformat-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for BinaryFormat`

- <span id="binaryformat-partialeq-eq"></span>`fn eq(&self, other: &BinaryFormat) -> bool` — [`BinaryFormat`](../index.md#binaryformat)

##### `impl StructuralPartialEq for BinaryFormat`

### `SectionKind`

```rust
enum SectionKind {
    Unknown,
    Text,
    Data,
    ReadOnlyData,
    ReadOnlyDataWithRel,
    ReadOnlyString,
    UninitializedData,
    Common,
    Tls,
    UninitializedTls,
    TlsVariables,
    OtherString,
    Other,
    Debug,
    DebugString,
    Linker,
    Note,
    Metadata,
    Elf(u32),
}
```

The kind of a section.

#### Variants

- **`Unknown`**

  The section kind is unknown.

- **`Text`**

  An executable code section.
  
  Example ELF sections: `.text`
  
  Example Mach-O sections: `__TEXT/__text`

- **`Data`**

  A data section.
  
  Example ELF sections: `.data`
  
  Example Mach-O sections: `__DATA/__data`

- **`ReadOnlyData`**

  A read only data section.
  
  Example ELF sections: `.rodata`
  
  Example Mach-O sections: `__TEXT/__const`, `__DATA/__const`, `__TEXT/__literal4`

- **`ReadOnlyDataWithRel`**

  A read only data section with relocations.
  
  This is the same as either `Data` or `ReadOnlyData`, depending on the file format.
  This value is only used in the API for writing files. It is never returned when reading files.

- **`ReadOnlyString`**

  A loadable string section.
  
  Example ELF sections: `.rodata.str`
  
  Example Mach-O sections: `__TEXT/__cstring`

- **`UninitializedData`**

  An uninitialized data section.
  
  Example ELF sections: `.bss`
  
  Example Mach-O sections: `__DATA/__bss`

- **`Common`**

  An uninitialized common data section.
  
  Example Mach-O sections: `__DATA/__common`

- **`Tls`**

  A TLS data section.
  
  Example ELF sections: `.tdata`
  
  Example Mach-O sections: `__DATA/__thread_data`

- **`UninitializedTls`**

  An uninitialized TLS data section.
  
  Example ELF sections: `.tbss`
  
  Example Mach-O sections: `__DATA/__thread_bss`

- **`TlsVariables`**

  A TLS variables section.
  
  This contains TLS variable structures, rather than the variable initializers.
  
  Example Mach-O sections: `__DATA/__thread_vars`

- **`OtherString`**

  A non-loadable string section.
  
  Example ELF sections: `.comment`, `.debug_str`

- **`Other`**

  Some other non-loadable section.
  
  Example ELF sections: `.debug_info`

- **`Debug`**

  Debug information.
  
  Example Mach-O sections: `__DWARF/__debug_info`

- **`DebugString`**

  Debug strings.
  
  This is the same as either `Debug` or `OtherString`, depending on the file format.
  This value is only used in the API for writing files. It is never returned when reading files.

- **`Linker`**

  Information for the linker.
  
  Example COFF sections: `.drectve`

- **`Note`**

  ELF note section.

- **`Metadata`**

  Metadata such as symbols or relocations.
  
  Example ELF sections: `.symtab`, `.strtab`, `.group`

- **`Elf`**

  Some other ELF section type.
  
  This is the `sh_type` field in the section header.
  The meaning may be dependent on the architecture.

#### Implementations

- <span id="sectionkind-is-bss"></span>`fn is_bss(self) -> bool`

  Return true if this section contains zerofill data.

#### Trait Implementations

##### `impl Clone for SectionKind`

- <span id="sectionkind-clone"></span>`fn clone(&self) -> SectionKind` — [`SectionKind`](../index.md#sectionkind)

##### `impl Copy for SectionKind`

##### `impl Debug for SectionKind`

- <span id="sectionkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SectionKind`

##### `impl<K> Equivalent for SectionKind`

- <span id="sectionkind-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Hash for SectionKind`

- <span id="sectionkind-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for SectionKind`

- <span id="sectionkind-partialeq-eq"></span>`fn eq(&self, other: &SectionKind) -> bool` — [`SectionKind`](../index.md#sectionkind)

##### `impl StructuralPartialEq for SectionKind`

### `ComdatKind`

```rust
enum ComdatKind {
    Unknown,
    Any,
    NoDuplicates,
    SameSize,
    ExactMatch,
    Largest,
    Newest,
}
```

The selection kind for a COMDAT section group.

This determines the way in which the linker resolves multiple definitions of the COMDAT
sections.

#### Variants

- **`Unknown`**

  The selection kind is unknown.

- **`Any`**

  Multiple definitions are allowed.
  
  An arbitrary definition is selected, and the rest are removed.
  
  This is the only supported selection kind for ELF.

- **`NoDuplicates`**

  Multiple definitions are not allowed.
  
  This is used to group sections without allowing duplicates.

- **`SameSize`**

  Multiple definitions must have the same size.
  
  An arbitrary definition is selected, and the rest are removed.

- **`ExactMatch`**

  Multiple definitions must match exactly.
  
  An arbitrary definition is selected, and the rest are removed.

- **`Largest`**

  Multiple definitions are allowed, and the largest is selected.
  
  An arbitrary definition with the largest size is selected, and the rest are removed.

- **`Newest`**

  Multiple definitions are allowed, and the newest is selected.

#### Trait Implementations

##### `impl Clone for ComdatKind`

- <span id="comdatkind-clone"></span>`fn clone(&self) -> ComdatKind` — [`ComdatKind`](../index.md#comdatkind)

##### `impl Copy for ComdatKind`

##### `impl Debug for ComdatKind`

- <span id="comdatkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ComdatKind`

##### `impl<K> Equivalent for ComdatKind`

- <span id="comdatkind-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Hash for ComdatKind`

- <span id="comdatkind-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for ComdatKind`

- <span id="comdatkind-partialeq-eq"></span>`fn eq(&self, other: &ComdatKind) -> bool` — [`ComdatKind`](../index.md#comdatkind)

##### `impl StructuralPartialEq for ComdatKind`

### `SymbolKind`

```rust
enum SymbolKind {
    Unknown,
    Text,
    Data,
    Section,
    File,
    Label,
    Tls,
}
```

The kind of a symbol.

#### Variants

- **`Unknown`**

  The symbol kind is unknown.

- **`Text`**

  The symbol is for executable code.

- **`Data`**

  The symbol is for a data object.

- **`Section`**

  The symbol is for a section.

- **`File`**

  The symbol is the name of a file. It precedes symbols within that file.

- **`Label`**

  The symbol is for a code label.

- **`Tls`**

  The symbol is for a thread local storage entity.

#### Trait Implementations

##### `impl Clone for SymbolKind`

- <span id="symbolkind-clone"></span>`fn clone(&self) -> SymbolKind` — [`SymbolKind`](../index.md#symbolkind)

##### `impl Copy for SymbolKind`

##### `impl Debug for SymbolKind`

- <span id="symbolkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SymbolKind`

##### `impl<K> Equivalent for SymbolKind`

- <span id="symbolkind-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Hash for SymbolKind`

- <span id="symbolkind-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for SymbolKind`

- <span id="symbolkind-partialeq-eq"></span>`fn eq(&self, other: &SymbolKind) -> bool` — [`SymbolKind`](../index.md#symbolkind)

##### `impl StructuralPartialEq for SymbolKind`

### `SymbolScope`

```rust
enum SymbolScope {
    Unknown,
    Compilation,
    Linkage,
    Dynamic,
}
```

A symbol scope.

#### Variants

- **`Unknown`**

  Unknown scope.

- **`Compilation`**

  Symbol is visible to the compilation unit.

- **`Linkage`**

  Symbol is visible to the static linkage unit.

- **`Dynamic`**

  Symbol is visible to dynamically linked objects.

#### Trait Implementations

##### `impl Clone for SymbolScope`

- <span id="symbolscope-clone"></span>`fn clone(&self) -> SymbolScope` — [`SymbolScope`](../index.md#symbolscope)

##### `impl Copy for SymbolScope`

##### `impl Debug for SymbolScope`

- <span id="symbolscope-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SymbolScope`

##### `impl<K> Equivalent for SymbolScope`

- <span id="symbolscope-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Hash for SymbolScope`

- <span id="symbolscope-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for SymbolScope`

- <span id="symbolscope-partialeq-eq"></span>`fn eq(&self, other: &SymbolScope) -> bool` — [`SymbolScope`](../index.md#symbolscope)

##### `impl StructuralPartialEq for SymbolScope`

### `RelocationKind`

```rust
enum RelocationKind {
    Unknown,
    None,
    Absolute,
    Relative,
    Got,
    GotRelative,
    GotBaseRelative,
    GotBaseOffset,
    PltRelative,
    ImageOffset,
    SectionOffset,
    SectionIndex,
}
```

The operation used to calculate the result of the relocation.

The relocation descriptions use the following definitions. Note that
these definitions probably don't match any ELF ABI.

* A - The value of the addend.
* G - The address of the symbol's entry within the global offset table.
* L - The address of the symbol's entry within the procedure linkage table.
* P - The address of the place of the relocation.
* S - The address of the symbol.
* GotBase - The address of the global offset table.
* Image - The base address of the image.
* Section - The address of the section containing the symbol.

'XxxRelative' means 'Xxx + A - P'.  'XxxOffset' means 'S + A - Xxx'.

#### Variants

- **`Unknown`**

  The operation is unknown.

- **`None`**

  No relocation.

- **`Absolute`**

  S + A

- **`Relative`**

  S + A - P

- **`Got`**

  G + A - GotBase

- **`GotRelative`**

  G + A - P

- **`GotBaseRelative`**

  GotBase + A - P

- **`GotBaseOffset`**

  S + A - GotBase

- **`PltRelative`**

  L + A - P

- **`ImageOffset`**

  S + A - Image

- **`SectionOffset`**

  S + A - Section

- **`SectionIndex`**

  The index of the section containing the symbol.

#### Trait Implementations

##### `impl Clone for RelocationKind`

- <span id="relocationkind-clone"></span>`fn clone(&self) -> RelocationKind` — [`RelocationKind`](../index.md#relocationkind)

##### `impl Copy for RelocationKind`

##### `impl Debug for RelocationKind`

- <span id="relocationkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for RelocationKind`

##### `impl<K> Equivalent for RelocationKind`

- <span id="relocationkind-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Hash for RelocationKind`

- <span id="relocationkind-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for RelocationKind`

- <span id="relocationkind-partialeq-eq"></span>`fn eq(&self, other: &RelocationKind) -> bool` — [`RelocationKind`](../index.md#relocationkind)

##### `impl StructuralPartialEq for RelocationKind`

### `RelocationEncoding`

```rust
enum RelocationEncoding {
    Unknown,
    Generic,
    X86Signed,
    X86RipRelative,
    X86RipRelativeMovq,
    X86Branch,
    S390xDbl,
    AArch64Call,
    LoongArchBranch,
    SharcTypeA,
    SharcTypeB,
    E2KLit,
    E2KDisp,
}
```

Information about how the result of the relocation operation is encoded in the place.

This is usually architecture specific, such as specifying an addressing mode or
a specific instruction.

#### Variants

- **`Unknown`**

  The relocation encoding is unknown.

- **`Generic`**

  Generic encoding.

- **`X86Signed`**

  x86 sign extension at runtime.
  
  Used with `RelocationKind::Absolute`.

- **`X86RipRelative`**

  x86 rip-relative addressing.
  
  The `RelocationKind` must be PC relative.

- **`X86RipRelativeMovq`**

  x86 rip-relative addressing in movq instruction.
  
  The `RelocationKind` must be PC relative.

- **`X86Branch`**

  x86 branch instruction.
  
  The `RelocationKind` must be PC relative.

- **`S390xDbl`**

  s390x PC-relative offset shifted right by one bit.
  
  The `RelocationKind` must be PC relative.

- **`AArch64Call`**

  AArch64 call target.
  
  The `RelocationKind` must be PC relative.

- **`LoongArchBranch`**

  LoongArch branch offset with two trailing zeros.
  
  The `RelocationKind` must be PC relative.

- **`SharcTypeA`**

  SHARC+ 48-bit Type A instruction
  
  Represents these possible variants, each with a corresponding
  `R_SHARC_*` constant:
  
  * 24-bit absolute address
  * 32-bit absolute address
  * 6-bit relative address
  * 24-bit relative address
  * 6-bit absolute address in the immediate value field
  * 16-bit absolute address in the immediate value field

- **`SharcTypeB`**

  SHARC+ 32-bit Type B instruction
  
  Represents these possible variants, each with a corresponding
  `R_SHARC_*` constant:
  
  * 6-bit absolute address in the immediate value field
  * 7-bit absolute address in the immediate value field
  * 16-bit absolute address
  * 6-bit relative address

- **`E2KLit`**

  E2K 64-bit value stored in two LTS
  
  Memory representation:
  ```text
  0: LTS1 = value[63:32]
  4: LTS0 = value[31:0]
  ```

- **`E2KDisp`**

  E2K 28-bit value stored in CS0

#### Trait Implementations

##### `impl Clone for RelocationEncoding`

- <span id="relocationencoding-clone"></span>`fn clone(&self) -> RelocationEncoding` — [`RelocationEncoding`](../index.md#relocationencoding)

##### `impl Copy for RelocationEncoding`

##### `impl Debug for RelocationEncoding`

- <span id="relocationencoding-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for RelocationEncoding`

##### `impl<K> Equivalent for RelocationEncoding`

- <span id="relocationencoding-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Hash for RelocationEncoding`

- <span id="relocationencoding-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for RelocationEncoding`

- <span id="relocationencoding-partialeq-eq"></span>`fn eq(&self, other: &RelocationEncoding) -> bool` — [`RelocationEncoding`](../index.md#relocationencoding)

##### `impl StructuralPartialEq for RelocationEncoding`

### `FileFlags`

```rust
enum FileFlags {
    None,
    Elf {
        os_abi: u8,
        abi_version: u8,
        e_flags: u32,
    },
    MachO {
        flags: u32,
    },
    Coff {
        characteristics: u16,
    },
    Xcoff {
        f_flags: u16,
    },
}
```

File flags that are specific to each file format.

#### Variants

- **`None`**

  No file flags.

- **`Elf`**

  ELF file flags.

- **`MachO`**

  Mach-O file flags.

- **`Coff`**

  COFF file flags.

- **`Xcoff`**

  XCOFF file flags.

#### Trait Implementations

##### `impl Clone for FileFlags`

- <span id="fileflags-clone"></span>`fn clone(&self) -> FileFlags` — [`FileFlags`](../index.md#fileflags)

##### `impl Copy for FileFlags`

##### `impl Debug for FileFlags`

- <span id="fileflags-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for FileFlags`

##### `impl<K> Equivalent for FileFlags`

- <span id="fileflags-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Hash for FileFlags`

- <span id="fileflags-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for FileFlags`

- <span id="fileflags-partialeq-eq"></span>`fn eq(&self, other: &FileFlags) -> bool` — [`FileFlags`](../index.md#fileflags)

##### `impl StructuralPartialEq for FileFlags`

### `SegmentFlags`

```rust
enum SegmentFlags {
    None,
    Elf {
        p_flags: u32,
    },
    MachO {
        flags: u32,
        maxprot: u32,
        initprot: u32,
    },
    Coff {
        characteristics: u32,
    },
}
```

Segment flags that are specific to each file format.

#### Variants

- **`None`**

  No segment flags.

- **`Elf`**

  ELF segment flags.

- **`MachO`**

  Mach-O segment flags.

- **`Coff`**

  COFF segment flags.

#### Trait Implementations

##### `impl Clone for SegmentFlags`

- <span id="segmentflags-clone"></span>`fn clone(&self) -> SegmentFlags` — [`SegmentFlags`](../index.md#segmentflags)

##### `impl Copy for SegmentFlags`

##### `impl Debug for SegmentFlags`

- <span id="segmentflags-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SegmentFlags`

##### `impl<K> Equivalent for SegmentFlags`

- <span id="segmentflags-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Hash for SegmentFlags`

- <span id="segmentflags-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for SegmentFlags`

- <span id="segmentflags-partialeq-eq"></span>`fn eq(&self, other: &SegmentFlags) -> bool` — [`SegmentFlags`](../index.md#segmentflags)

##### `impl StructuralPartialEq for SegmentFlags`

### `SectionFlags`

```rust
enum SectionFlags {
    None,
    Elf {
        sh_flags: u64,
    },
    MachO {
        flags: u32,
    },
    Coff {
        characteristics: u32,
    },
    Xcoff {
        s_flags: u32,
    },
}
```

Section flags that are specific to each file format.

#### Variants

- **`None`**

  No section flags.

- **`Elf`**

  ELF section flags.

- **`MachO`**

  Mach-O section flags.

- **`Coff`**

  COFF section flags.

- **`Xcoff`**

  XCOFF section flags.

#### Trait Implementations

##### `impl Clone for SectionFlags`

- <span id="sectionflags-clone"></span>`fn clone(&self) -> SectionFlags` — [`SectionFlags`](../index.md#sectionflags)

##### `impl Copy for SectionFlags`

##### `impl Debug for SectionFlags`

- <span id="sectionflags-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SectionFlags`

##### `impl<K> Equivalent for SectionFlags`

- <span id="sectionflags-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Hash for SectionFlags`

- <span id="sectionflags-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for SectionFlags`

- <span id="sectionflags-partialeq-eq"></span>`fn eq(&self, other: &SectionFlags) -> bool` — [`SectionFlags`](../index.md#sectionflags)

##### `impl StructuralPartialEq for SectionFlags`

### `SymbolFlags<Section, Symbol>`

```rust
enum SymbolFlags<Section, Symbol> {
    None,
    Elf {
        st_info: u8,
        st_other: u8,
    },
    MachO {
        n_desc: u16,
    },
    CoffSection {
        selection: u8,
        associative_section: Option<Section>,
    },
    Xcoff {
        n_sclass: u8,
        x_smtyp: u8,
        x_smclas: u8,
        containing_csect: Option<Symbol>,
    },
}
```

Symbol flags that are specific to each file format.

#### Variants

- **`None`**

  No symbol flags.

- **`Elf`**

  ELF symbol flags.

- **`MachO`**

  Mach-O symbol flags.

- **`CoffSection`**

  COFF flags for a section symbol.

- **`Xcoff`**

  XCOFF symbol flags.

#### Trait Implementations

##### `impl<Section: clone::Clone, Symbol: clone::Clone> Clone for SymbolFlags<Section, Symbol>`

- <span id="symbolflags-clone"></span>`fn clone(&self) -> SymbolFlags<Section, Symbol>` — [`SymbolFlags`](../index.md#symbolflags)

##### `impl<Section: marker::Copy, Symbol: marker::Copy> Copy for SymbolFlags<Section, Symbol>`

##### `impl<Section: fmt::Debug, Symbol: fmt::Debug> Debug for SymbolFlags<Section, Symbol>`

- <span id="symbolflags-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Section: cmp::Eq, Symbol: cmp::Eq> Eq for SymbolFlags<Section, Symbol>`

##### `impl<K> Equivalent for SymbolFlags<Section, Symbol>`

- <span id="symbolflags-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl<Section: hash::Hash, Symbol: hash::Hash> Hash for SymbolFlags<Section, Symbol>`

- <span id="symbolflags-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<Section: cmp::PartialEq, Symbol: cmp::PartialEq> PartialEq for SymbolFlags<Section, Symbol>`

- <span id="symbolflags-partialeq-eq"></span>`fn eq(&self, other: &SymbolFlags<Section, Symbol>) -> bool` — [`SymbolFlags`](../index.md#symbolflags)

##### `impl<Section, Symbol> StructuralPartialEq for SymbolFlags<Section, Symbol>`

### `RelocationFlags`

```rust
enum RelocationFlags {
    Generic {
        kind: RelocationKind,
        encoding: RelocationEncoding,
        size: u8,
    },
    Elf {
        r_type: u32,
    },
    MachO {
        r_type: u8,
        r_pcrel: bool,
        r_length: u8,
    },
    Coff {
        typ: u16,
    },
    Xcoff {
        r_rtype: u8,
        r_rsize: u8,
    },
}
```

Relocation fields that are specific to each file format and architecture.

#### Variants

- **`Generic`**

  Format independent representation.

- **`Elf`**

  ELF relocation fields.

- **`MachO`**

  Mach-O relocation fields.

- **`Coff`**

  COFF relocation fields.

- **`Xcoff`**

  XCOFF relocation fields.

#### Trait Implementations

##### `impl Clone for RelocationFlags`

- <span id="relocationflags-clone"></span>`fn clone(&self) -> RelocationFlags` — [`RelocationFlags`](../index.md#relocationflags)

##### `impl Copy for RelocationFlags`

##### `impl Debug for RelocationFlags`

- <span id="relocationflags-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for RelocationFlags`

##### `impl<K> Equivalent for RelocationFlags`

- <span id="relocationflags-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Hash for RelocationFlags`

- <span id="relocationflags-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for RelocationFlags`

- <span id="relocationflags-partialeq-eq"></span>`fn eq(&self, other: &RelocationFlags) -> bool` — [`RelocationFlags`](../index.md#relocationflags)

##### `impl StructuralPartialEq for RelocationFlags`

## Traits

### `WritableBuffer`

```rust
trait WritableBuffer { ... }
```

Trait for writable buffer.

#### Required Methods

- `fn len(&self) -> usize`

  Returns position/offset for data to be written at.

- `fn reserve(&mut self, size: usize) -> Result<(), ()>`

  Reserves specified number of bytes in the buffer.

- `fn resize(&mut self, new_len: usize)`

  Writes zero bytes at the end of the buffer until the buffer

- `fn write_bytes(&mut self, val: &[u8])`

  Writes the specified slice of bytes at the end of the buffer.

#### Provided Methods

- `fn write_pod<T: Pod>(&mut self, val: &T)`

  Writes the specified `Pod` type at the end of the buffer.

- `fn write_pod_slice<T: Pod>(&mut self, val: &[T])`

  Writes the specified `Pod` slice at the end of the buffer.

#### Implementors

- [`StreamingBuffer`](#streamingbuffer)
- `alloc::vec::Vec<u8>`

### `BytesMut`

```rust
trait BytesMut { ... }
```

A trait for mutable byte slices.

It provides convenience methods for `Pod` types.

#### Required Methods

- `fn write_at<T: Pod>(self, offset: usize, val: &T) -> Result<(), ()>`

#### Implementors

- `&'a mut [u8]`

## Functions

### `write_uleb128`

```rust
fn write_uleb128(buf: &mut alloc::vec::Vec<u8>, val: u64) -> usize
```

Write an unsigned number using the LEB128 encoding to a buffer.

Returns the number of bytes written.

### `write_sleb128`

```rust
fn write_sleb128(buf: &mut alloc::vec::Vec<u8>, val: i64) -> usize
```

Write a signed number using the LEB128 encoding to a buffer.

Returns the number of bytes written.

### `align`

```rust
fn align(offset: usize, size: usize) -> usize
```

### `align_u32`

```rust
fn align_u32(offset: u32, size: u32) -> u32
```

### `align_u64`

```rust
fn align_u64(offset: u64, size: u64) -> u64
```

### `write_align`

```rust
fn write_align(buffer: &mut dyn WritableBuffer, size: usize)
```

## Type Aliases

### `Result<T>`

```rust
type Result<T> = result::Result<T, Error>;
```

The result type used within the write module.

