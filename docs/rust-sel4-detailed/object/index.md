# Crate `object`

The `object` crate provides a unified interface to working with object files
across platforms. It supports reading relocatable object files and executable files,
and writing relocatable object files and some executable files.

## Raw struct definitions

Raw structs are defined for: [ELF](elf), [Mach-O](macho), [PE/COFF](pe),
[XCOFF](xcoff), [archive](#archive).
Types and traits for zerocopy support are defined in the [`pod`](pod/index.md) and [`endian`](endian/index.md) modules.

## Unified read API

The [`read`](read/index.md) module provides a unified read API using the [`read::Object`](read/index.md) trait.
There is an implementation of this trait for [`read::File`](read/index.md), which allows reading any
file format, as well as implementations for each file format.

## Low level read API

The [`read#modules`](read/index.md) submodules define helpers that operate on the raw structs.
These can be used instead of the unified API, or in conjunction with it to access
details that are not available via the unified API.

## Unified write API

The [`mod@write`](write/index.md) module provides a unified write API for relocatable object files
using [`write::Object`](write/index.md). This does not support writing executable files.

## Low level write API

The [`mod@write#modules`](write/index.md) submodules define helpers for writing the raw structs.

## Build API

The [`mod@build`](build/index.md) submodules define helpers for building object files, either from
scratch or by modifying existing files.

## Shared definitions

The crate provides a number of definitions that are used by both the read and write
APIs. These are defined at the top level module, but none of these are the main entry
points of the crate.

## Contents

- [Modules](#modules)
  - [`common`](#common)
  - [`endian`](#endian)
  - [`pod`](#pod)
  - [`read`](#read)
  - [`write`](#write)
  - [`build`](#build)
  - [`archive`](#archive)
  - [`elf`](#elf)
  - [`macho`](#macho)
  - [`pe`](#pe)
  - [`xcoff`](#xcoff)
  - [`read_ref`](#read-ref)
  - [`read_cache`](#read-cache)
  - [`util`](#util)
  - [`gnu_compression`](#gnu-compression)
  - [`any`](#any)
  - [`archive`](#archive)
  - [`coff`](#coff)
  - [`elf`](#elf)
  - [`macho`](#macho)
  - [`pe`](#pe)
  - [`wasm`](#wasm)
  - [`xcoff`](#xcoff)
  - [`traits`](#traits)
  - [`private`](#private)
- [Structs](#structs)
  - [`LittleEndian`](#littleendian)
  - [`BigEndian`](#bigendian)
  - [`U16Bytes`](#u16bytes)
  - [`U32Bytes`](#u32bytes)
  - [`U64Bytes`](#u64bytes)
  - [`I16Bytes`](#i16bytes)
  - [`I32Bytes`](#i32bytes)
  - [`I64Bytes`](#i64bytes)
  - [`Error`](#error)
  - [`SectionIndex`](#sectionindex)
  - [`SymbolIndex`](#symbolindex)
  - [`SymbolMap`](#symbolmap)
  - [`SymbolMapName`](#symbolmapname)
  - [`ObjectMap`](#objectmap)
  - [`ObjectMapEntry`](#objectmapentry)
  - [`ObjectMapFile`](#objectmapfile)
  - [`Import`](#import)
  - [`Export`](#export)
  - [`CodeView`](#codeview)
  - [`Relocation`](#relocation)
  - [`RelocationMap`](#relocationmap)
  - [`RelocationMapEntry`](#relocationmapentry)
  - [`CompressedFileRange`](#compressedfilerange)
  - [`CompressedData`](#compresseddata)
- [Enums](#enums)
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
  - [`Endianness`](#endianness)
  - [`FileKind`](#filekind)
  - [`ObjectKind`](#objectkind)
  - [`SymbolSection`](#symbolsection)
  - [`RelocationTarget`](#relocationtarget)
  - [`CompressionFormat`](#compressionformat)
- [Traits](#traits)
  - [`Endian`](#endian)
  - [`Pod`](#pod)
  - [`ReadError`](#readerror)
  - [`SymbolMapEntry`](#symbolmapentry)
- [Functions](#functions)
  - [`from_bytes`](#from-bytes)
  - [`from_bytes_mut`](#from-bytes-mut)
  - [`slice_from_bytes`](#slice-from-bytes)
  - [`slice_from_bytes_mut`](#slice-from-bytes-mut)
  - [`slice_from_all_bytes`](#slice-from-all-bytes)
  - [`slice_from_all_bytes_mut`](#slice-from-all-bytes-mut)
  - [`bytes_of`](#bytes-of)
  - [`bytes_of_mut`](#bytes-of-mut)
  - [`bytes_of_slice`](#bytes-of-slice)
  - [`bytes_of_slice_mut`](#bytes-of-slice-mut)
- [Type Aliases](#type-aliases)
  - [`NativeEndian`](#nativeendian)
  - [`U16`](#u16)
  - [`U32`](#u32)
  - [`U64`](#u64)
  - [`I16`](#i16)
  - [`I32`](#i32)
  - [`I64`](#i64)
  - [`Result`](#result)
  - [`Result`](#result)
  - [`NativeFile`](#nativefile)
- [Macros](#macros)
  - [`unsafe_impl_endian_pod!`](#unsafe-impl-endian-pod)
  - [`unsafe_impl_pod!`](#unsafe-impl-pod)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`common`](#common) | mod |  |
| [`endian`](#endian) | mod | Types for compile-time and run-time endianness. |
| [`pod`](#pod) | mod | Tools for converting file format structures to and from bytes. |
| [`read`](#read) | mod | Interface for reading object files. |
| [`write`](#write) | mod | Interface for writing object files. |
| [`build`](#build) | mod | Interface for building object files. |
| [`archive`](#archive) | mod | Archive definitions. |
| [`elf`](#elf) | mod | ELF definitions. |
| [`macho`](#macho) | mod | Mach-O definitions. |
| [`pe`](#pe) | mod | PE/COFF definitions. |
| [`xcoff`](#xcoff) | mod | XCOFF definitions |
| [`read_ref`](#read-ref) | mod |  |
| [`read_cache`](#read-cache) | mod |  |
| [`util`](#util) | mod |  |
| [`gnu_compression`](#gnu-compression) | mod |  |
| [`any`](#any) | mod |  |
| [`archive`](#archive) | mod | Support for archive files. |
| [`coff`](#coff) | mod | Support for reading Windows COFF files. |
| [`elf`](#elf) | mod | Support for reading ELF files. |
| [`macho`](#macho) | mod | Support for reading Mach-O files. |
| [`pe`](#pe) | mod | Support for reading PE files. |
| [`wasm`](#wasm) | mod | Support for reading Wasm files. |
| [`xcoff`](#xcoff) | mod | Support for reading AIX XCOFF files. |
| [`traits`](#traits) | mod |  |
| [`private`](#private) | mod |  |
| [`LittleEndian`](#littleendian) | struct | Compile-time little endian byte order. |
| [`BigEndian`](#bigendian) | struct | Compile-time big endian byte order. |
| [`U16Bytes`](#u16bytes) | struct | An unaligned `u16` value with an externally specified endianness of type `E`. |
| [`U32Bytes`](#u32bytes) | struct | An unaligned `u32` value with an externally specified endianness of type `E`. |
| [`U64Bytes`](#u64bytes) | struct | An unaligned `u64` value with an externally specified endianness of type `E`. |
| [`I16Bytes`](#i16bytes) | struct | An unaligned `i16` value with an externally specified endianness of type `E`. |
| [`I32Bytes`](#i32bytes) | struct | An unaligned `i32` value with an externally specified endianness of type `E`. |
| [`I64Bytes`](#i64bytes) | struct | An unaligned `i64` value with an externally specified endianness of type `E`. |
| [`Error`](#error) | struct | The error type used within the read module. |
| [`SectionIndex`](#sectionindex) | struct | The index used to identify a section in a file. |
| [`SymbolIndex`](#symbolindex) | struct | The index used to identify a symbol in a symbol table. |
| [`SymbolMap`](#symbolmap) | struct | A map from addresses to symbol information. |
| [`SymbolMapName`](#symbolmapname) | struct | The type used for entries in a [`SymbolMap`] that maps from addresses to names. |
| [`ObjectMap`](#objectmap) | struct | A map from addresses to symbol names and object files. |
| [`ObjectMapEntry`](#objectmapentry) | struct | A symbol in an [`ObjectMap`]. |
| [`ObjectMapFile`](#objectmapfile) | struct | An object file name in an [`ObjectMap`]. |
| [`Import`](#import) | struct | An imported symbol. |
| [`Export`](#export) | struct | An exported symbol. |
| [`CodeView`](#codeview) | struct | PDB information from the debug directory in a PE file. |
| [`Relocation`](#relocation) | struct | A relocation entry. |
| [`RelocationMap`](#relocationmap) | struct | A map from section offsets to relocation information. |
| [`RelocationMapEntry`](#relocationmapentry) | struct |  |
| [`CompressedFileRange`](#compressedfilerange) | struct | A range in a file that may be compressed. |
| [`CompressedData`](#compresseddata) | struct | Data that may be compressed. |
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
| [`Endianness`](#endianness) | enum | An endianness that is selectable at run-time. |
| [`FileKind`](#filekind) | enum | A file format kind. |
| [`ObjectKind`](#objectkind) | enum | An object kind. |
| [`SymbolSection`](#symbolsection) | enum | The section where an [`ObjectSymbol`] is defined. |
| [`RelocationTarget`](#relocationtarget) | enum | The target referenced by a [`Relocation`]. |
| [`CompressionFormat`](#compressionformat) | enum | A data compression format. |
| [`Endian`](#endian) | trait | A trait for using an endianness specification. |
| [`Pod`](#pod) | trait | A trait for types that can safely be converted from and to byte slices. |
| [`ReadError`](#readerror) | trait |  |
| [`SymbolMapEntry`](#symbolmapentry) | trait | An entry in a [`SymbolMap`]. |
| [`from_bytes`](#from-bytes) | fn | Cast the head of a byte slice to a `Pod` type. |
| [`from_bytes_mut`](#from-bytes-mut) | fn | Cast the head of a mutable byte slice to a `Pod` type. |
| [`slice_from_bytes`](#slice-from-bytes) | fn | Cast the head of a byte slice to a slice of a `Pod` type. |
| [`slice_from_bytes_mut`](#slice-from-bytes-mut) | fn | Cast the head of a mutable byte slice to a slice of a `Pod` type. |
| [`slice_from_all_bytes`](#slice-from-all-bytes) | fn | Cast all of a byte slice to a slice of a `Pod` type. |
| [`slice_from_all_bytes_mut`](#slice-from-all-bytes-mut) | fn | Cast all of a byte slice to a slice of a `Pod` type. |
| [`bytes_of`](#bytes-of) | fn | Cast a `Pod` type to a byte slice. |
| [`bytes_of_mut`](#bytes-of-mut) | fn | Cast a `Pod` type to a mutable byte slice. |
| [`bytes_of_slice`](#bytes-of-slice) | fn | Cast a slice of a `Pod` type to a byte slice. |
| [`bytes_of_slice_mut`](#bytes-of-slice-mut) | fn | Cast a slice of a `Pod` type to a mutable byte slice. |
| [`NativeEndian`](#nativeendian) | type | The native endianness for the target platform. |
| [`U16`](#u16) | type | A `u16` value with an externally specified endianness of type `E`. |
| [`U32`](#u32) | type | A `u32` value with an externally specified endianness of type `E`. |
| [`U64`](#u64) | type | A `u64` value with an externally specified endianness of type `E`. |
| [`I16`](#i16) | type | An `i16` value with an externally specified endianness of type `E`. |
| [`I32`](#i32) | type | An `i32` value with an externally specified endianness of type `E`. |
| [`I64`](#i64) | type | An `i64` value with an externally specified endianness of type `E`. |
| [`Result`](#result) | type |  |
| [`Result`](#result) | type | The result type used within the read module. |
| [`NativeFile`](#nativefile) | type | The native executable file for the target platform. |
| [`unsafe_impl_endian_pod!`](#unsafe-impl-endian-pod) | macro |  |
| [`unsafe_impl_pod!`](#unsafe-impl-pod) | macro |  |

## Modules

- [`common`](common/index.md)
- [`endian`](endian/index.md) — Types for compile-time and run-time endianness.
- [`pod`](pod/index.md) — Tools for converting file format structures to and from bytes.
- [`read`](read/index.md) — Interface for reading object files.
- [`write`](write/index.md) — Interface for writing object files.
- [`build`](build/index.md) — Interface for building object files.
- [`archive`](archive/index.md) — Archive definitions.
- [`elf`](elf/index.md) — ELF definitions.
- [`macho`](macho/index.md) — Mach-O definitions.
- [`pe`](pe/index.md) — PE/COFF definitions.
- [`xcoff`](xcoff/index.md) — XCOFF definitions
- [`read_ref`](read_ref/index.md)
- [`read_cache`](read_cache/index.md)
- [`util`](util/index.md)
- [`gnu_compression`](gnu_compression/index.md)
- [`any`](any/index.md)
- [`archive`](archive/index.md) — Support for archive files.
- [`coff`](coff/index.md) — Support for reading Windows COFF files.
- [`elf`](elf/index.md) — Support for reading ELF files.
- [`macho`](macho/index.md) — Support for reading Mach-O files.
- [`pe`](pe/index.md) — Support for reading PE files.
- [`wasm`](wasm/index.md) — Support for reading Wasm files.
- [`xcoff`](xcoff/index.md) — Support for reading AIX XCOFF files.
- [`traits`](traits/index.md)
- [`private`](private/index.md)

## Structs

### `LittleEndian`

```rust
struct LittleEndian;
```

Compile-time little endian byte order.

#### Trait Implementations

##### `impl Clone for LittleEndian`

- <span id="littleendian-clone"></span>`fn clone(&self) -> LittleEndian` — [`LittleEndian`](#littleendian)

##### `impl Copy for LittleEndian`

##### `impl Debug for LittleEndian`

- <span id="littleendian-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for LittleEndian`

- <span id="littleendian-default"></span>`fn default() -> LittleEndian` — [`LittleEndian`](#littleendian)

##### `impl Endian for LittleEndian`

- <span id="littleendian-endian-from-big-endian"></span>`fn from_big_endian(big_endian: bool) -> Option<Self>`

- <span id="littleendian-endian-is-big-endian"></span>`fn is_big_endian(self) -> bool`

##### `impl Eq for LittleEndian`

##### `impl<K> Equivalent for LittleEndian`

- <span id="littleendian-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Hash for LittleEndian`

- <span id="littleendian-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for LittleEndian`

- <span id="littleendian-partialeq-eq"></span>`fn eq(&self, other: &LittleEndian) -> bool` — [`LittleEndian`](#littleendian)

##### `impl StructuralPartialEq for LittleEndian`

### `BigEndian`

```rust
struct BigEndian;
```

Compile-time big endian byte order.

#### Trait Implementations

##### `impl Clone for BigEndian`

- <span id="bigendian-clone"></span>`fn clone(&self) -> BigEndian` — [`BigEndian`](#bigendian)

##### `impl Copy for BigEndian`

##### `impl Debug for BigEndian`

- <span id="bigendian-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for BigEndian`

- <span id="bigendian-default"></span>`fn default() -> BigEndian` — [`BigEndian`](#bigendian)

##### `impl Endian for BigEndian`

- <span id="bigendian-endian-from-big-endian"></span>`fn from_big_endian(big_endian: bool) -> Option<Self>`

- <span id="bigendian-endian-is-big-endian"></span>`fn is_big_endian(self) -> bool`

##### `impl Eq for BigEndian`

##### `impl<K> Equivalent for BigEndian`

- <span id="bigendian-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Hash for BigEndian`

- <span id="bigendian-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for BigEndian`

- <span id="bigendian-partialeq-eq"></span>`fn eq(&self, other: &BigEndian) -> bool` — [`BigEndian`](#bigendian)

##### `impl StructuralPartialEq for BigEndian`

### `U16Bytes<E: Endian>`

```rust
struct U16Bytes<E: Endian>([u8; 2], core::marker::PhantomData<E>);
```

An unaligned `u16` value with an externally specified endianness of type `E`.

#### Implementations

- <span id="u16bytes-from-bytes"></span>`const fn from_bytes(n: [u8; 2]) -> Self`

  Construct a new value given bytes that already have the required endianness.

- <span id="u16bytes-new"></span>`fn new(e: E, n: u16) -> Self`

  Construct a new value given a native endian value.

- <span id="u16bytes-get"></span>`fn get(self, e: E) -> u16`

  Return the value as a native endian value.

- <span id="u16bytes-set"></span>`fn set(&mut self, e: E, n: u16)`

  Set the value given a native endian value.

#### Trait Implementations

##### `impl<E: clone::Clone + Endian> Clone for U16Bytes<E>`

- <span id="u16bytes-clone"></span>`fn clone(&self) -> U16Bytes<E>` — [`U16Bytes`](#u16bytes)

##### `impl<K> Comparable for U16Bytes<E>`

- <span id="u16bytes-comparable-compare"></span>`fn compare(&self, key: &K) -> Ordering`

##### `impl<E: marker::Copy + Endian> Copy for U16Bytes<E>`

##### `impl<E: Endian> Debug for U16Bytes<E>`

- <span id="u16bytes-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E: default::Default + Endian> Default for U16Bytes<E>`

- <span id="u16bytes-default"></span>`fn default() -> U16Bytes<E>` — [`U16Bytes`](#u16bytes)

##### `impl<E: cmp::Eq + Endian> Eq for U16Bytes<E>`

##### `impl<K> Equivalent for U16Bytes<E>`

- <span id="u16bytes-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl<E: hash::Hash + Endian> Hash for U16Bytes<E>`

- <span id="u16bytes-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<E: cmp::Ord + Endian> Ord for U16Bytes<E>`

- <span id="u16bytes-ord-cmp"></span>`fn cmp(&self, other: &U16Bytes<E>) -> cmp::Ordering` — [`U16Bytes`](#u16bytes)

##### `impl<E: cmp::PartialEq + Endian> PartialEq for U16Bytes<E>`

- <span id="u16bytes-partialeq-eq"></span>`fn eq(&self, other: &U16Bytes<E>) -> bool` — [`U16Bytes`](#u16bytes)

##### `impl<E: cmp::PartialOrd + Endian> PartialOrd for U16Bytes<E>`

- <span id="u16bytes-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &U16Bytes<E>) -> option::Option<cmp::Ordering>` — [`U16Bytes`](#u16bytes)

##### `impl<E: Endian> Pod for U16Bytes<E>`

##### `impl<E: Endian> StructuralPartialEq for U16Bytes<E>`

### `U32Bytes<E: Endian>`

```rust
struct U32Bytes<E: Endian>([u8; 4], core::marker::PhantomData<E>);
```

An unaligned `u32` value with an externally specified endianness of type `E`.

#### Implementations

- <span id="u32bytes-from-bytes"></span>`const fn from_bytes(n: [u8; 4]) -> Self`

  Construct a new value given bytes that already have the required endianness.

- <span id="u32bytes-new"></span>`fn new(e: E, n: u32) -> Self`

  Construct a new value given a native endian value.

- <span id="u32bytes-get"></span>`fn get(self, e: E) -> u32`

  Return the value as a native endian value.

- <span id="u32bytes-set"></span>`fn set(&mut self, e: E, n: u32)`

  Set the value given a native endian value.

#### Trait Implementations

##### `impl<E: clone::Clone + Endian> Clone for U32Bytes<E>`

- <span id="u32bytes-clone"></span>`fn clone(&self) -> U32Bytes<E>` — [`U32Bytes`](#u32bytes)

##### `impl<K> Comparable for U32Bytes<E>`

- <span id="u32bytes-comparable-compare"></span>`fn compare(&self, key: &K) -> Ordering`

##### `impl<E: marker::Copy + Endian> Copy for U32Bytes<E>`

##### `impl<E: Endian> Debug for U32Bytes<E>`

- <span id="u32bytes-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E: default::Default + Endian> Default for U32Bytes<E>`

- <span id="u32bytes-default"></span>`fn default() -> U32Bytes<E>` — [`U32Bytes`](#u32bytes)

##### `impl<E: cmp::Eq + Endian> Eq for U32Bytes<E>`

##### `impl<K> Equivalent for U32Bytes<E>`

- <span id="u32bytes-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl<E: hash::Hash + Endian> Hash for U32Bytes<E>`

- <span id="u32bytes-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<E: cmp::Ord + Endian> Ord for U32Bytes<E>`

- <span id="u32bytes-ord-cmp"></span>`fn cmp(&self, other: &U32Bytes<E>) -> cmp::Ordering` — [`U32Bytes`](#u32bytes)

##### `impl<E: cmp::PartialEq + Endian> PartialEq for U32Bytes<E>`

- <span id="u32bytes-partialeq-eq"></span>`fn eq(&self, other: &U32Bytes<E>) -> bool` — [`U32Bytes`](#u32bytes)

##### `impl<E: cmp::PartialOrd + Endian> PartialOrd for U32Bytes<E>`

- <span id="u32bytes-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &U32Bytes<E>) -> option::Option<cmp::Ordering>` — [`U32Bytes`](#u32bytes)

##### `impl<E: Endian> Pod for U32Bytes<E>`

##### `impl<E: Endian> StructuralPartialEq for U32Bytes<E>`

### `U64Bytes<E: Endian>`

```rust
struct U64Bytes<E: Endian>([u8; 8], core::marker::PhantomData<E>);
```

An unaligned `u64` value with an externally specified endianness of type `E`.

#### Implementations

- <span id="u64bytes-from-bytes"></span>`const fn from_bytes(n: [u8; 8]) -> Self`

  Construct a new value given bytes that already have the required endianness.

- <span id="u64bytes-new"></span>`fn new(e: E, n: u64) -> Self`

  Construct a new value given a native endian value.

- <span id="u64bytes-get"></span>`fn get(self, e: E) -> u64`

  Return the value as a native endian value.

- <span id="u64bytes-set"></span>`fn set(&mut self, e: E, n: u64)`

  Set the value given a native endian value.

#### Trait Implementations

##### `impl<E: clone::Clone + Endian> Clone for U64Bytes<E>`

- <span id="u64bytes-clone"></span>`fn clone(&self) -> U64Bytes<E>` — [`U64Bytes`](#u64bytes)

##### `impl<K> Comparable for U64Bytes<E>`

- <span id="u64bytes-comparable-compare"></span>`fn compare(&self, key: &K) -> Ordering`

##### `impl<E: marker::Copy + Endian> Copy for U64Bytes<E>`

##### `impl<E: Endian> Debug for U64Bytes<E>`

- <span id="u64bytes-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E: default::Default + Endian> Default for U64Bytes<E>`

- <span id="u64bytes-default"></span>`fn default() -> U64Bytes<E>` — [`U64Bytes`](#u64bytes)

##### `impl<E: cmp::Eq + Endian> Eq for U64Bytes<E>`

##### `impl<K> Equivalent for U64Bytes<E>`

- <span id="u64bytes-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl<E: hash::Hash + Endian> Hash for U64Bytes<E>`

- <span id="u64bytes-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<E: cmp::Ord + Endian> Ord for U64Bytes<E>`

- <span id="u64bytes-ord-cmp"></span>`fn cmp(&self, other: &U64Bytes<E>) -> cmp::Ordering` — [`U64Bytes`](#u64bytes)

##### `impl<E: cmp::PartialEq + Endian> PartialEq for U64Bytes<E>`

- <span id="u64bytes-partialeq-eq"></span>`fn eq(&self, other: &U64Bytes<E>) -> bool` — [`U64Bytes`](#u64bytes)

##### `impl<E: cmp::PartialOrd + Endian> PartialOrd for U64Bytes<E>`

- <span id="u64bytes-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &U64Bytes<E>) -> option::Option<cmp::Ordering>` — [`U64Bytes`](#u64bytes)

##### `impl<E: Endian> Pod for U64Bytes<E>`

##### `impl<E: Endian> StructuralPartialEq for U64Bytes<E>`

### `I16Bytes<E: Endian>`

```rust
struct I16Bytes<E: Endian>([u8; 2], core::marker::PhantomData<E>);
```

An unaligned `i16` value with an externally specified endianness of type `E`.

#### Implementations

- <span id="i16bytes-from-bytes"></span>`const fn from_bytes(n: [u8; 2]) -> Self`

  Construct a new value given bytes that already have the required endianness.

- <span id="i16bytes-new"></span>`fn new(e: E, n: i16) -> Self`

  Construct a new value given a native endian value.

- <span id="i16bytes-get"></span>`fn get(self, e: E) -> i16`

  Return the value as a native endian value.

- <span id="i16bytes-set"></span>`fn set(&mut self, e: E, n: i16)`

  Set the value given a native endian value.

#### Trait Implementations

##### `impl<E: clone::Clone + Endian> Clone for I16Bytes<E>`

- <span id="i16bytes-clone"></span>`fn clone(&self) -> I16Bytes<E>` — [`I16Bytes`](#i16bytes)

##### `impl<K> Comparable for I16Bytes<E>`

- <span id="i16bytes-comparable-compare"></span>`fn compare(&self, key: &K) -> Ordering`

##### `impl<E: marker::Copy + Endian> Copy for I16Bytes<E>`

##### `impl<E: Endian> Debug for I16Bytes<E>`

- <span id="i16bytes-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E: default::Default + Endian> Default for I16Bytes<E>`

- <span id="i16bytes-default"></span>`fn default() -> I16Bytes<E>` — [`I16Bytes`](#i16bytes)

##### `impl<E: cmp::Eq + Endian> Eq for I16Bytes<E>`

##### `impl<K> Equivalent for I16Bytes<E>`

- <span id="i16bytes-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl<E: hash::Hash + Endian> Hash for I16Bytes<E>`

- <span id="i16bytes-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<E: cmp::Ord + Endian> Ord for I16Bytes<E>`

- <span id="i16bytes-ord-cmp"></span>`fn cmp(&self, other: &I16Bytes<E>) -> cmp::Ordering` — [`I16Bytes`](#i16bytes)

##### `impl<E: cmp::PartialEq + Endian> PartialEq for I16Bytes<E>`

- <span id="i16bytes-partialeq-eq"></span>`fn eq(&self, other: &I16Bytes<E>) -> bool` — [`I16Bytes`](#i16bytes)

##### `impl<E: cmp::PartialOrd + Endian> PartialOrd for I16Bytes<E>`

- <span id="i16bytes-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &I16Bytes<E>) -> option::Option<cmp::Ordering>` — [`I16Bytes`](#i16bytes)

##### `impl<E: Endian> Pod for I16Bytes<E>`

##### `impl<E: Endian> StructuralPartialEq for I16Bytes<E>`

### `I32Bytes<E: Endian>`

```rust
struct I32Bytes<E: Endian>([u8; 4], core::marker::PhantomData<E>);
```

An unaligned `i32` value with an externally specified endianness of type `E`.

#### Implementations

- <span id="i32bytes-from-bytes"></span>`const fn from_bytes(n: [u8; 4]) -> Self`

  Construct a new value given bytes that already have the required endianness.

- <span id="i32bytes-new"></span>`fn new(e: E, n: i32) -> Self`

  Construct a new value given a native endian value.

- <span id="i32bytes-get"></span>`fn get(self, e: E) -> i32`

  Return the value as a native endian value.

- <span id="i32bytes-set"></span>`fn set(&mut self, e: E, n: i32)`

  Set the value given a native endian value.

#### Trait Implementations

##### `impl<E: clone::Clone + Endian> Clone for I32Bytes<E>`

- <span id="i32bytes-clone"></span>`fn clone(&self) -> I32Bytes<E>` — [`I32Bytes`](#i32bytes)

##### `impl<K> Comparable for I32Bytes<E>`

- <span id="i32bytes-comparable-compare"></span>`fn compare(&self, key: &K) -> Ordering`

##### `impl<E: marker::Copy + Endian> Copy for I32Bytes<E>`

##### `impl<E: Endian> Debug for I32Bytes<E>`

- <span id="i32bytes-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E: default::Default + Endian> Default for I32Bytes<E>`

- <span id="i32bytes-default"></span>`fn default() -> I32Bytes<E>` — [`I32Bytes`](#i32bytes)

##### `impl<E: cmp::Eq + Endian> Eq for I32Bytes<E>`

##### `impl<K> Equivalent for I32Bytes<E>`

- <span id="i32bytes-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl<E: hash::Hash + Endian> Hash for I32Bytes<E>`

- <span id="i32bytes-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<E: cmp::Ord + Endian> Ord for I32Bytes<E>`

- <span id="i32bytes-ord-cmp"></span>`fn cmp(&self, other: &I32Bytes<E>) -> cmp::Ordering` — [`I32Bytes`](#i32bytes)

##### `impl<E: cmp::PartialEq + Endian> PartialEq for I32Bytes<E>`

- <span id="i32bytes-partialeq-eq"></span>`fn eq(&self, other: &I32Bytes<E>) -> bool` — [`I32Bytes`](#i32bytes)

##### `impl<E: cmp::PartialOrd + Endian> PartialOrd for I32Bytes<E>`

- <span id="i32bytes-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &I32Bytes<E>) -> option::Option<cmp::Ordering>` — [`I32Bytes`](#i32bytes)

##### `impl<E: Endian> Pod for I32Bytes<E>`

##### `impl<E: Endian> StructuralPartialEq for I32Bytes<E>`

### `I64Bytes<E: Endian>`

```rust
struct I64Bytes<E: Endian>([u8; 8], core::marker::PhantomData<E>);
```

An unaligned `i64` value with an externally specified endianness of type `E`.

#### Implementations

- <span id="i64bytes-from-bytes"></span>`const fn from_bytes(n: [u8; 8]) -> Self`

  Construct a new value given bytes that already have the required endianness.

- <span id="i64bytes-new"></span>`fn new(e: E, n: i64) -> Self`

  Construct a new value given a native endian value.

- <span id="i64bytes-get"></span>`fn get(self, e: E) -> i64`

  Return the value as a native endian value.

- <span id="i64bytes-set"></span>`fn set(&mut self, e: E, n: i64)`

  Set the value given a native endian value.

#### Trait Implementations

##### `impl<E: clone::Clone + Endian> Clone for I64Bytes<E>`

- <span id="i64bytes-clone"></span>`fn clone(&self) -> I64Bytes<E>` — [`I64Bytes`](#i64bytes)

##### `impl<K> Comparable for I64Bytes<E>`

- <span id="i64bytes-comparable-compare"></span>`fn compare(&self, key: &K) -> Ordering`

##### `impl<E: marker::Copy + Endian> Copy for I64Bytes<E>`

##### `impl<E: Endian> Debug for I64Bytes<E>`

- <span id="i64bytes-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<E: default::Default + Endian> Default for I64Bytes<E>`

- <span id="i64bytes-default"></span>`fn default() -> I64Bytes<E>` — [`I64Bytes`](#i64bytes)

##### `impl<E: cmp::Eq + Endian> Eq for I64Bytes<E>`

##### `impl<K> Equivalent for I64Bytes<E>`

- <span id="i64bytes-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl<E: hash::Hash + Endian> Hash for I64Bytes<E>`

- <span id="i64bytes-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<E: cmp::Ord + Endian> Ord for I64Bytes<E>`

- <span id="i64bytes-ord-cmp"></span>`fn cmp(&self, other: &I64Bytes<E>) -> cmp::Ordering` — [`I64Bytes`](#i64bytes)

##### `impl<E: cmp::PartialEq + Endian> PartialEq for I64Bytes<E>`

- <span id="i64bytes-partialeq-eq"></span>`fn eq(&self, other: &I64Bytes<E>) -> bool` — [`I64Bytes`](#i64bytes)

##### `impl<E: cmp::PartialOrd + Endian> PartialOrd for I64Bytes<E>`

- <span id="i64bytes-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &I64Bytes<E>) -> option::Option<cmp::Ordering>` — [`I64Bytes`](#i64bytes)

##### `impl<E: Endian> Pod for I64Bytes<E>`

##### `impl<E: Endian> StructuralPartialEq for I64Bytes<E>`

### `Error`

```rust
struct Error(&'static str);
```

The error type used within the read module.

#### Trait Implementations

##### `impl Clone for Error`

- <span id="error-clone"></span>`fn clone(&self) -> Error` — [`Error`](#error)

##### `impl Copy for Error`

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

### `SectionIndex`

```rust
struct SectionIndex(usize);
```

The index used to identify a section in a file.

#### Trait Implementations

##### `impl Clone for SectionIndex`

- <span id="sectionindex-clone"></span>`fn clone(&self) -> SectionIndex` — [`SectionIndex`](#sectionindex)

##### `impl Copy for SectionIndex`

##### `impl Debug for SectionIndex`

- <span id="sectionindex-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for SectionIndex`

- <span id="sectionindex-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SectionIndex`

##### `impl<K> Equivalent for SectionIndex`

- <span id="sectionindex-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Hash for SectionIndex`

- <span id="sectionindex-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for SectionIndex`

- <span id="sectionindex-partialeq-eq"></span>`fn eq(&self, other: &SectionIndex) -> bool` — [`SectionIndex`](#sectionindex)

##### `impl StructuralPartialEq for SectionIndex`

##### `impl ToString for SectionIndex`

- <span id="sectionindex-tostring-to-string"></span>`fn to_string(&self) -> String`

### `SymbolIndex`

```rust
struct SymbolIndex(usize);
```

The index used to identify a symbol in a symbol table.

#### Trait Implementations

##### `impl Clone for SymbolIndex`

- <span id="symbolindex-clone"></span>`fn clone(&self) -> SymbolIndex` — [`SymbolIndex`](#symbolindex)

##### `impl Copy for SymbolIndex`

##### `impl Debug for SymbolIndex`

- <span id="symbolindex-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Display for SymbolIndex`

- <span id="symbolindex-display-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SymbolIndex`

##### `impl<K> Equivalent for SymbolIndex`

- <span id="symbolindex-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Hash for SymbolIndex`

- <span id="symbolindex-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for SymbolIndex`

- <span id="symbolindex-partialeq-eq"></span>`fn eq(&self, other: &SymbolIndex) -> bool` — [`SymbolIndex`](#symbolindex)

##### `impl StructuralPartialEq for SymbolIndex`

##### `impl ToString for SymbolIndex`

- <span id="symbolindex-tostring-to-string"></span>`fn to_string(&self) -> String`

### `SymbolMap<T: SymbolMapEntry>`

```rust
struct SymbolMap<T: SymbolMapEntry> {
    symbols: alloc::vec::Vec<T>,
}
```

A map from addresses to symbol information.

The symbol information depends on the chosen entry type, such as [`SymbolMapName`](#symbolmapname).

Returned by `Object::symbol_map`.

#### Implementations

- <span id="symbolmap-new"></span>`fn new(symbols: Vec<T>) -> Self`

  Construct a new symbol map.

  

  This function will sort the symbols by address.

- <span id="symbolmap-get"></span>`fn get(&self, address: u64) -> Option<&T>`

  Get the symbol before the given address.

- <span id="symbolmap-symbols"></span>`fn symbols(&self) -> &[T]`

  Get all symbols in the map.

#### Trait Implementations

##### `impl<T: clone::Clone + SymbolMapEntry> Clone for SymbolMap<T>`

- <span id="symbolmap-clone"></span>`fn clone(&self) -> SymbolMap<T>` — [`SymbolMap`](#symbolmap)

##### `impl<T: fmt::Debug + SymbolMapEntry> Debug for SymbolMap<T>`

- <span id="symbolmap-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: default::Default + SymbolMapEntry> Default for SymbolMap<T>`

- <span id="symbolmap-default"></span>`fn default() -> SymbolMap<T>` — [`SymbolMap`](#symbolmap)

### `SymbolMapName<'data>`

```rust
struct SymbolMapName<'data> {
    address: u64,
    name: &'data str,
}
```

The type used for entries in a [`SymbolMap`](#symbolmap) that maps from addresses to names.

#### Implementations

- <span id="symbolmapname-new"></span>`fn new(address: u64, name: &'data str) -> Self`

  Construct a `SymbolMapName`.

- <span id="symbolmapname-address"></span>`fn address(&self) -> u64`

  The symbol address.

- <span id="symbolmapname-name"></span>`fn name(&self) -> &'data str`

  The symbol name.

#### Trait Implementations

##### `impl Clone for SymbolMapName<'data>`

- <span id="symbolmapname-clone"></span>`fn clone(&self) -> SymbolMapName<'data>` — [`SymbolMapName`](#symbolmapname)

##### `impl Copy for SymbolMapName<'data>`

##### `impl Debug for SymbolMapName<'data>`

- <span id="symbolmapname-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SymbolMapName<'data>`

##### `impl<K> Equivalent for SymbolMapName<'data>`

- <span id="symbolmapname-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Hash for SymbolMapName<'data>`

- <span id="symbolmapname-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for SymbolMapName<'data>`

- <span id="symbolmapname-partialeq-eq"></span>`fn eq(&self, other: &SymbolMapName<'data>) -> bool` — [`SymbolMapName`](#symbolmapname)

##### `impl StructuralPartialEq for SymbolMapName<'data>`

##### `impl SymbolMapEntry for SymbolMapName<'data>`

- <span id="symbolmapname-symbolmapentry-address"></span>`fn address(&self) -> u64`

### `ObjectMap<'data>`

```rust
struct ObjectMap<'data> {
    symbols: SymbolMap<ObjectMapEntry<'data>>,
    objects: alloc::vec::Vec<ObjectMapFile<'data>>,
}
```

A map from addresses to symbol names and object files.

This is derived from STAB entries in Mach-O files.

Returned by `Object::object_map`.

#### Implementations

- <span id="objectmap-get"></span>`fn get(&self, address: u64) -> Option<&ObjectMapEntry<'data>>` — [`ObjectMapEntry`](#objectmapentry)

  Get the entry containing the given address.

- <span id="objectmap-symbols"></span>`fn symbols(&self) -> &[ObjectMapEntry<'data>]` — [`ObjectMapEntry`](#objectmapentry)

  Get all symbols in the map.

- <span id="objectmap-objects"></span>`fn objects(&self) -> &[ObjectMapFile<'data>]` — [`ObjectMapFile`](#objectmapfile)

  Get all objects in the map.

#### Trait Implementations

##### `impl Clone for ObjectMap<'data>`

- <span id="objectmap-clone"></span>`fn clone(&self) -> ObjectMap<'data>` — [`ObjectMap`](#objectmap)

##### `impl Debug for ObjectMap<'data>`

- <span id="objectmap-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ObjectMap<'data>`

- <span id="objectmap-default"></span>`fn default() -> ObjectMap<'data>` — [`ObjectMap`](#objectmap)

### `ObjectMapEntry<'data>`

```rust
struct ObjectMapEntry<'data> {
    address: u64,
    size: u64,
    name: &'data [u8],
    object: usize,
}
```

A symbol in an [`ObjectMap`](#objectmap).

#### Implementations

- <span id="objectmapentry-address"></span>`fn address(&self) -> u64`

  Get the symbol address.

- <span id="objectmapentry-size"></span>`fn size(&self) -> u64`

  Get the symbol size.

  

  This may be 0 if the size is unknown.

- <span id="objectmapentry-name"></span>`fn name(&self) -> &'data [u8]`

  Get the symbol name.

- <span id="objectmapentry-object-index"></span>`fn object_index(&self) -> usize`

  Get the index of the object file name.

- <span id="objectmapentry-object"></span>`fn object<'a>(&self, map: &'a ObjectMap<'data>) -> &'a ObjectMapFile<'data>` — [`ObjectMap`](#objectmap), [`ObjectMapFile`](#objectmapfile)

  Get the object file name.

#### Trait Implementations

##### `impl Clone for ObjectMapEntry<'data>`

- <span id="objectmapentry-clone"></span>`fn clone(&self) -> ObjectMapEntry<'data>` — [`ObjectMapEntry`](#objectmapentry)

##### `impl Copy for ObjectMapEntry<'data>`

##### `impl Debug for ObjectMapEntry<'data>`

- <span id="objectmapentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for ObjectMapEntry<'data>`

- <span id="objectmapentry-default"></span>`fn default() -> ObjectMapEntry<'data>` — [`ObjectMapEntry`](#objectmapentry)

##### `impl Eq for ObjectMapEntry<'data>`

##### `impl<K> Equivalent for ObjectMapEntry<'data>`

- <span id="objectmapentry-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Hash for ObjectMapEntry<'data>`

- <span id="objectmapentry-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for ObjectMapEntry<'data>`

- <span id="objectmapentry-partialeq-eq"></span>`fn eq(&self, other: &ObjectMapEntry<'data>) -> bool` — [`ObjectMapEntry`](#objectmapentry)

##### `impl StructuralPartialEq for ObjectMapEntry<'data>`

##### `impl SymbolMapEntry for ObjectMapEntry<'data>`

- <span id="objectmapentry-symbolmapentry-address"></span>`fn address(&self) -> u64`

### `ObjectMapFile<'data>`

```rust
struct ObjectMapFile<'data> {
    path: &'data [u8],
    member: Option<&'data [u8]>,
}
```

An object file name in an [`ObjectMap`](#objectmap).

#### Implementations

- <span id="objectmapfile-new"></span>`fn new(path: &'data [u8], member: Option<&'data [u8]>) -> Self`

- <span id="objectmapfile-path"></span>`fn path(&self) -> &'data [u8]`

  Get the path to the file containing the object.

- <span id="objectmapfile-member"></span>`fn member(&self) -> Option<&'data [u8]>`

  If the file is an archive, get the name of the member containing the object.

#### Trait Implementations

##### `impl Clone for ObjectMapFile<'data>`

- <span id="objectmapfile-clone"></span>`fn clone(&self) -> ObjectMapFile<'data>` — [`ObjectMapFile`](#objectmapfile)

##### `impl Copy for ObjectMapFile<'data>`

##### `impl Debug for ObjectMapFile<'data>`

- <span id="objectmapfile-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ObjectMapFile<'data>`

##### `impl<K> Equivalent for ObjectMapFile<'data>`

- <span id="objectmapfile-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Hash for ObjectMapFile<'data>`

- <span id="objectmapfile-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for ObjectMapFile<'data>`

- <span id="objectmapfile-partialeq-eq"></span>`fn eq(&self, other: &ObjectMapFile<'data>) -> bool` — [`ObjectMapFile`](#objectmapfile)

##### `impl StructuralPartialEq for ObjectMapFile<'data>`

### `Import<'data>`

```rust
struct Import<'data> {
    library: ByteString<'data>,
    name: ByteString<'data>,
}
```

An imported symbol.

Returned by `Object::imports`.

#### Implementations

- <span id="import-name"></span>`fn name(&self) -> &'data [u8]`

  The symbol name.

- <span id="import-library"></span>`fn library(&self) -> &'data [u8]`

  The name of the library to import the symbol from.

#### Trait Implementations

##### `impl Clone for Import<'data>`

- <span id="import-clone"></span>`fn clone(&self) -> Import<'data>` — [`Import`](#import)

##### `impl Copy for Import<'data>`

##### `impl Debug for Import<'data>`

- <span id="import-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Import<'data>`

##### `impl<K> Equivalent for Import<'data>`

- <span id="import-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl PartialEq for Import<'data>`

- <span id="import-partialeq-eq"></span>`fn eq(&self, other: &Import<'data>) -> bool` — [`Import`](#import)

##### `impl StructuralPartialEq for Import<'data>`

### `Export<'data>`

```rust
struct Export<'data> {
    name: ByteString<'data>,
    address: u64,
}
```

An exported symbol.

Returned by `Object::exports`.

#### Implementations

- <span id="export-name"></span>`fn name(&self) -> &'data [u8]`

  The symbol name.

- <span id="export-address"></span>`fn address(&self) -> u64`

  The virtual address of the symbol.

#### Trait Implementations

##### `impl Clone for Export<'data>`

- <span id="export-clone"></span>`fn clone(&self) -> Export<'data>` — [`Export`](#export)

##### `impl Copy for Export<'data>`

##### `impl Debug for Export<'data>`

- <span id="export-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Export<'data>`

##### `impl<K> Equivalent for Export<'data>`

- <span id="export-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl PartialEq for Export<'data>`

- <span id="export-partialeq-eq"></span>`fn eq(&self, other: &Export<'data>) -> bool` — [`Export`](#export)

##### `impl StructuralPartialEq for Export<'data>`

### `CodeView<'data>`

```rust
struct CodeView<'data> {
    guid: [u8; 16],
    path: ByteString<'data>,
    age: u32,
}
```

PDB information from the debug directory in a PE file.

#### Implementations

- <span id="codeview-path"></span>`fn path(&self) -> &'data [u8]`

  The path to the PDB as stored in CodeView.

- <span id="codeview-age"></span>`fn age(&self) -> u32`

  The age of the PDB.

- <span id="codeview-guid"></span>`fn guid(&self) -> [u8; 16]`

  The GUID of the PDB.

#### Trait Implementations

##### `impl Clone for CodeView<'data>`

- <span id="codeview-clone"></span>`fn clone(&self) -> CodeView<'data>` — [`CodeView`](#codeview)

##### `impl Copy for CodeView<'data>`

##### `impl Debug for CodeView<'data>`

- <span id="codeview-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for CodeView<'data>`

##### `impl<K> Equivalent for CodeView<'data>`

- <span id="codeview-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl PartialEq for CodeView<'data>`

- <span id="codeview-partialeq-eq"></span>`fn eq(&self, other: &CodeView<'data>) -> bool` — [`CodeView`](#codeview)

##### `impl StructuralPartialEq for CodeView<'data>`

### `Relocation`

```rust
struct Relocation {
    kind: RelocationKind,
    encoding: RelocationEncoding,
    size: u8,
    target: RelocationTarget,
    addend: i64,
    implicit_addend: bool,
    flags: RelocationFlags,
}
```

A relocation entry.

Returned by `Object::dynamic_relocations` or `ObjectSection::relocations`.

#### Implementations

- <span id="relocation-kind"></span>`fn kind(&self) -> RelocationKind` — [`RelocationKind`](#relocationkind)

  The operation used to calculate the result of the relocation.

- <span id="relocation-encoding"></span>`fn encoding(&self) -> RelocationEncoding` — [`RelocationEncoding`](#relocationencoding)

  Information about how the result of the relocation operation is encoded in the place.

- <span id="relocation-size"></span>`fn size(&self) -> u8`

  The size in bits of the place of the relocation.

  

  If 0, then the size is determined by the relocation kind.

- <span id="relocation-target"></span>`fn target(&self) -> RelocationTarget` — [`RelocationTarget`](#relocationtarget)

  The target of the relocation.

- <span id="relocation-addend"></span>`fn addend(&self) -> i64`

  The addend to use in the relocation calculation.

- <span id="relocation-set-addend"></span>`fn set_addend(&mut self, addend: i64)`

  Set the addend to use in the relocation calculation.

- <span id="relocation-has-implicit-addend"></span>`fn has_implicit_addend(&self) -> bool`

  Returns true if there is an implicit addend stored in the data at the offset

  to be relocated.

- <span id="relocation-flags"></span>`fn flags(&self) -> RelocationFlags` — [`RelocationFlags`](#relocationflags)

  Relocation flags that are specific to each file format.

  

  The values returned by `kind`, `encoding` and `size` are derived

  from these flags.

#### Trait Implementations

##### `impl Debug for Relocation`

- <span id="relocation-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `RelocationMap`

```rust
struct RelocationMap(std::collections::hash_map::HashMap<u64, RelocationMapEntry>);
```

A map from section offsets to relocation information.

This can be used to apply relocations to a value at a given section offset.
This is intended for use with DWARF in relocatable object files, and only
supports relocations that are used in DWARF.

Returned by `ObjectSection::relocation_map`.

#### Implementations

- <span id="relocationmap-new"></span>`fn new<'data, 'file, T>(file: &'file T, section: &<T as >::Section) -> Result<Self>` — [`Object`](read/index.md#object), [`Result`](#result)

  Construct a new relocation map for a section.

  

  Fails if any relocation cannot be added to the map.

  You can manually use `add` if you need different error handling,

  such as to list all errors or to ignore them.

- <span id="relocationmap-add"></span>`fn add<'data: 'file, 'file, T>(&mut self, file: &'file T, offset: u64, relocation: Relocation) -> Result<()>` — [`Relocation`](#relocation), [`Result`](#result)

  Add a single relocation to the map.

- <span id="relocationmap-relocate"></span>`fn relocate(&self, offset: u64, value: u64) -> u64`

  Relocate a value that was read from the section at the given offset.

#### Trait Implementations

##### `impl Debug for RelocationMap`

- <span id="relocationmap-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for RelocationMap`

- <span id="relocationmap-default"></span>`fn default() -> RelocationMap` — [`RelocationMap`](#relocationmap)

### `RelocationMapEntry`

```rust
struct RelocationMapEntry {
    implicit_addend: bool,
    addend: u64,
}
```

#### Trait Implementations

##### `impl Clone for RelocationMapEntry`

- <span id="relocationmapentry-clone"></span>`fn clone(&self) -> RelocationMapEntry` — [`RelocationMapEntry`](read/index.md#relocationmapentry)

##### `impl Copy for RelocationMapEntry`

##### `impl Debug for RelocationMapEntry`

- <span id="relocationmapentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for RelocationMapEntry`

##### `impl<K> Equivalent for RelocationMapEntry`

- <span id="relocationmapentry-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Hash for RelocationMapEntry`

- <span id="relocationmapentry-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for RelocationMapEntry`

- <span id="relocationmapentry-partialeq-eq"></span>`fn eq(&self, other: &RelocationMapEntry) -> bool` — [`RelocationMapEntry`](read/index.md#relocationmapentry)

##### `impl StructuralPartialEq for RelocationMapEntry`

### `CompressedFileRange`

```rust
struct CompressedFileRange {
    pub format: CompressionFormat,
    pub offset: u64,
    pub compressed_size: u64,
    pub uncompressed_size: u64,
}
```

A range in a file that may be compressed.

Returned by `ObjectSection::compressed_file_range`.

#### Fields

- **`format`**: `CompressionFormat`

  The data compression format.

- **`offset`**: `u64`

  The file offset of the compressed data.

- **`compressed_size`**: `u64`

  The compressed data size.

- **`uncompressed_size`**: `u64`

  The uncompressed data size.

#### Implementations

- <span id="compressedfilerange-none"></span>`fn none(range: Option<(u64, u64)>) -> Self`

  Data that is uncompressed.

- <span id="compressedfilerange-data"></span>`fn data<'data, R: ReadRef<'data>>(self, file: R) -> Result<CompressedData<'data>>` — [`Result`](#result), [`CompressedData`](#compresseddata)

  Convert to [`CompressedData`](#compresseddata) by reading from the file.

#### Trait Implementations

##### `impl Clone for CompressedFileRange`

- <span id="compressedfilerange-clone"></span>`fn clone(&self) -> CompressedFileRange` — [`CompressedFileRange`](#compressedfilerange)

##### `impl Copy for CompressedFileRange`

##### `impl Debug for CompressedFileRange`

- <span id="compressedfilerange-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for CompressedFileRange`

##### `impl<K> Equivalent for CompressedFileRange`

- <span id="compressedfilerange-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Hash for CompressedFileRange`

- <span id="compressedfilerange-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for CompressedFileRange`

- <span id="compressedfilerange-partialeq-eq"></span>`fn eq(&self, other: &CompressedFileRange) -> bool` — [`CompressedFileRange`](#compressedfilerange)

##### `impl StructuralPartialEq for CompressedFileRange`

### `CompressedData<'data>`

```rust
struct CompressedData<'data> {
    pub format: CompressionFormat,
    pub data: &'data [u8],
    pub uncompressed_size: u64,
}
```

Data that may be compressed.

Returned by `ObjectSection::compressed_data`.

#### Fields

- **`format`**: `CompressionFormat`

  The data compression format.

- **`data`**: `&'data [u8]`

  The compressed data.

- **`uncompressed_size`**: `u64`

  The uncompressed data size.

#### Implementations

- <span id="compresseddata-none"></span>`fn none(data: &'data [u8]) -> Self`

  Data that is uncompressed.

- <span id="compresseddata-decompress"></span>`fn decompress(self) -> Result<Cow<'data, [u8]>>` — [`Result`](#result)

  Return the uncompressed data.

  

  Returns an error for invalid data or unsupported compression.

  This includes if the data is compressed but the `compression` feature

  for this crate is disabled.

#### Trait Implementations

##### `impl Clone for CompressedData<'data>`

- <span id="compresseddata-clone"></span>`fn clone(&self) -> CompressedData<'data>` — [`CompressedData`](#compresseddata)

##### `impl Copy for CompressedData<'data>`

##### `impl Debug for CompressedData<'data>`

- <span id="compresseddata-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for CompressedData<'data>`

##### `impl<K> Equivalent for CompressedData<'data>`

- <span id="compresseddata-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Hash for CompressedData<'data>`

- <span id="compresseddata-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for CompressedData<'data>`

- <span id="compresseddata-partialeq-eq"></span>`fn eq(&self, other: &CompressedData<'data>) -> bool` — [`CompressedData`](#compresseddata)

##### `impl StructuralPartialEq for CompressedData<'data>`

## Enums

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

- <span id="architecture-address-size"></span>`fn address_size(self) -> Option<AddressSize>` — [`AddressSize`](#addresssize)

  The size of an address value for this architecture.

  

  Returns `None` for unknown architectures.

#### Trait Implementations

##### `impl Clone for Architecture`

- <span id="architecture-clone"></span>`fn clone(&self) -> Architecture` — [`Architecture`](#architecture)

##### `impl Copy for Architecture`

##### `impl Debug for Architecture`

- <span id="architecture-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Architecture`

##### `impl<K> Equivalent for Architecture`

- <span id="architecture-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Hash for Architecture`

- <span id="architecture-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Architecture`

- <span id="architecture-partialeq-eq"></span>`fn eq(&self, other: &Architecture) -> bool` — [`Architecture`](#architecture)

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

- <span id="subarchitecture-clone"></span>`fn clone(&self) -> SubArchitecture` — [`SubArchitecture`](#subarchitecture)

##### `impl Copy for SubArchitecture`

##### `impl Debug for SubArchitecture`

- <span id="subarchitecture-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SubArchitecture`

##### `impl<K> Equivalent for SubArchitecture`

- <span id="subarchitecture-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Hash for SubArchitecture`

- <span id="subarchitecture-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for SubArchitecture`

- <span id="subarchitecture-partialeq-eq"></span>`fn eq(&self, other: &SubArchitecture) -> bool` — [`SubArchitecture`](#subarchitecture)

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

- <span id="addresssize-clone"></span>`fn clone(&self) -> AddressSize` — [`AddressSize`](#addresssize)

##### `impl Copy for AddressSize`

##### `impl Debug for AddressSize`

- <span id="addresssize-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for AddressSize`

##### `impl<K> Equivalent for AddressSize`

- <span id="addresssize-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Hash for AddressSize`

- <span id="addresssize-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for AddressSize`

- <span id="addresssize-partialeq-eq"></span>`fn eq(&self, other: &AddressSize) -> bool` — [`AddressSize`](#addresssize)

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

- <span id="binaryformat-native-object"></span>`fn native_object() -> BinaryFormat` — [`BinaryFormat`](#binaryformat)

  The target's native binary format for relocatable object files.

  

  Defaults to `Elf` for unknown platforms.

#### Trait Implementations

##### `impl Clone for BinaryFormat`

- <span id="binaryformat-clone"></span>`fn clone(&self) -> BinaryFormat` — [`BinaryFormat`](#binaryformat)

##### `impl Copy for BinaryFormat`

##### `impl Debug for BinaryFormat`

- <span id="binaryformat-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for BinaryFormat`

##### `impl<K> Equivalent for BinaryFormat`

- <span id="binaryformat-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Hash for BinaryFormat`

- <span id="binaryformat-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for BinaryFormat`

- <span id="binaryformat-partialeq-eq"></span>`fn eq(&self, other: &BinaryFormat) -> bool` — [`BinaryFormat`](#binaryformat)

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

- <span id="sectionkind-clone"></span>`fn clone(&self) -> SectionKind` — [`SectionKind`](#sectionkind)

##### `impl Copy for SectionKind`

##### `impl Debug for SectionKind`

- <span id="sectionkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SectionKind`

##### `impl<K> Equivalent for SectionKind`

- <span id="sectionkind-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Hash for SectionKind`

- <span id="sectionkind-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for SectionKind`

- <span id="sectionkind-partialeq-eq"></span>`fn eq(&self, other: &SectionKind) -> bool` — [`SectionKind`](#sectionkind)

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

- <span id="comdatkind-clone"></span>`fn clone(&self) -> ComdatKind` — [`ComdatKind`](#comdatkind)

##### `impl Copy for ComdatKind`

##### `impl Debug for ComdatKind`

- <span id="comdatkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ComdatKind`

##### `impl<K> Equivalent for ComdatKind`

- <span id="comdatkind-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Hash for ComdatKind`

- <span id="comdatkind-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for ComdatKind`

- <span id="comdatkind-partialeq-eq"></span>`fn eq(&self, other: &ComdatKind) -> bool` — [`ComdatKind`](#comdatkind)

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

- <span id="symbolkind-clone"></span>`fn clone(&self) -> SymbolKind` — [`SymbolKind`](#symbolkind)

##### `impl Copy for SymbolKind`

##### `impl Debug for SymbolKind`

- <span id="symbolkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SymbolKind`

##### `impl<K> Equivalent for SymbolKind`

- <span id="symbolkind-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Hash for SymbolKind`

- <span id="symbolkind-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for SymbolKind`

- <span id="symbolkind-partialeq-eq"></span>`fn eq(&self, other: &SymbolKind) -> bool` — [`SymbolKind`](#symbolkind)

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

- <span id="symbolscope-clone"></span>`fn clone(&self) -> SymbolScope` — [`SymbolScope`](#symbolscope)

##### `impl Copy for SymbolScope`

##### `impl Debug for SymbolScope`

- <span id="symbolscope-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SymbolScope`

##### `impl<K> Equivalent for SymbolScope`

- <span id="symbolscope-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Hash for SymbolScope`

- <span id="symbolscope-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for SymbolScope`

- <span id="symbolscope-partialeq-eq"></span>`fn eq(&self, other: &SymbolScope) -> bool` — [`SymbolScope`](#symbolscope)

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

- <span id="relocationkind-clone"></span>`fn clone(&self) -> RelocationKind` — [`RelocationKind`](#relocationkind)

##### `impl Copy for RelocationKind`

##### `impl Debug for RelocationKind`

- <span id="relocationkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for RelocationKind`

##### `impl<K> Equivalent for RelocationKind`

- <span id="relocationkind-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Hash for RelocationKind`

- <span id="relocationkind-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for RelocationKind`

- <span id="relocationkind-partialeq-eq"></span>`fn eq(&self, other: &RelocationKind) -> bool` — [`RelocationKind`](#relocationkind)

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

- <span id="relocationencoding-clone"></span>`fn clone(&self) -> RelocationEncoding` — [`RelocationEncoding`](#relocationencoding)

##### `impl Copy for RelocationEncoding`

##### `impl Debug for RelocationEncoding`

- <span id="relocationencoding-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for RelocationEncoding`

##### `impl<K> Equivalent for RelocationEncoding`

- <span id="relocationencoding-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Hash for RelocationEncoding`

- <span id="relocationencoding-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for RelocationEncoding`

- <span id="relocationencoding-partialeq-eq"></span>`fn eq(&self, other: &RelocationEncoding) -> bool` — [`RelocationEncoding`](#relocationencoding)

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

- <span id="fileflags-clone"></span>`fn clone(&self) -> FileFlags` — [`FileFlags`](#fileflags)

##### `impl Copy for FileFlags`

##### `impl Debug for FileFlags`

- <span id="fileflags-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for FileFlags`

##### `impl<K> Equivalent for FileFlags`

- <span id="fileflags-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Hash for FileFlags`

- <span id="fileflags-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for FileFlags`

- <span id="fileflags-partialeq-eq"></span>`fn eq(&self, other: &FileFlags) -> bool` — [`FileFlags`](#fileflags)

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

- <span id="segmentflags-clone"></span>`fn clone(&self) -> SegmentFlags` — [`SegmentFlags`](#segmentflags)

##### `impl Copy for SegmentFlags`

##### `impl Debug for SegmentFlags`

- <span id="segmentflags-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SegmentFlags`

##### `impl<K> Equivalent for SegmentFlags`

- <span id="segmentflags-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Hash for SegmentFlags`

- <span id="segmentflags-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for SegmentFlags`

- <span id="segmentflags-partialeq-eq"></span>`fn eq(&self, other: &SegmentFlags) -> bool` — [`SegmentFlags`](#segmentflags)

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

- <span id="sectionflags-clone"></span>`fn clone(&self) -> SectionFlags` — [`SectionFlags`](#sectionflags)

##### `impl Copy for SectionFlags`

##### `impl Debug for SectionFlags`

- <span id="sectionflags-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for SectionFlags`

##### `impl<K> Equivalent for SectionFlags`

- <span id="sectionflags-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Hash for SectionFlags`

- <span id="sectionflags-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for SectionFlags`

- <span id="sectionflags-partialeq-eq"></span>`fn eq(&self, other: &SectionFlags) -> bool` — [`SectionFlags`](#sectionflags)

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

- <span id="symbolflags-clone"></span>`fn clone(&self) -> SymbolFlags<Section, Symbol>` — [`SymbolFlags`](#symbolflags)

##### `impl<Section: marker::Copy, Symbol: marker::Copy> Copy for SymbolFlags<Section, Symbol>`

##### `impl<Section: fmt::Debug, Symbol: fmt::Debug> Debug for SymbolFlags<Section, Symbol>`

- <span id="symbolflags-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<Section: cmp::Eq, Symbol: cmp::Eq> Eq for SymbolFlags<Section, Symbol>`

##### `impl<K> Equivalent for SymbolFlags<Section, Symbol>`

- <span id="symbolflags-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl<Section: hash::Hash, Symbol: hash::Hash> Hash for SymbolFlags<Section, Symbol>`

- <span id="symbolflags-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl<Section: cmp::PartialEq, Symbol: cmp::PartialEq> PartialEq for SymbolFlags<Section, Symbol>`

- <span id="symbolflags-partialeq-eq"></span>`fn eq(&self, other: &SymbolFlags<Section, Symbol>) -> bool` — [`SymbolFlags`](#symbolflags)

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

- <span id="relocationflags-clone"></span>`fn clone(&self) -> RelocationFlags` — [`RelocationFlags`](#relocationflags)

##### `impl Copy for RelocationFlags`

##### `impl Debug for RelocationFlags`

- <span id="relocationflags-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for RelocationFlags`

##### `impl<K> Equivalent for RelocationFlags`

- <span id="relocationflags-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Hash for RelocationFlags`

- <span id="relocationflags-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for RelocationFlags`

- <span id="relocationflags-partialeq-eq"></span>`fn eq(&self, other: &RelocationFlags) -> bool` — [`RelocationFlags`](#relocationflags)

##### `impl StructuralPartialEq for RelocationFlags`

### `Endianness`

```rust
enum Endianness {
    Little,
    Big,
}
```

An endianness that is selectable at run-time.

#### Variants

- **`Little`**

  Little endian byte order.

- **`Big`**

  Big endian byte order.

#### Trait Implementations

##### `impl Clone for Endianness`

- <span id="endianness-clone"></span>`fn clone(&self) -> Endianness` — [`Endianness`](#endianness)

##### `impl Copy for Endianness`

##### `impl Debug for Endianness`

- <span id="endianness-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for Endianness`

- <span id="endianness-default"></span>`fn default() -> Endianness` — [`Endianness`](#endianness)

##### `impl Endian for Endianness`

- <span id="endianness-endian-from-big-endian"></span>`fn from_big_endian(big_endian: bool) -> Option<Self>`

- <span id="endianness-endian-is-big-endian"></span>`fn is_big_endian(self) -> bool`

##### `impl Eq for Endianness`

##### `impl<K> Equivalent for Endianness`

- <span id="endianness-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Hash for Endianness`

- <span id="endianness-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for Endianness`

- <span id="endianness-partialeq-eq"></span>`fn eq(&self, other: &Endianness) -> bool` — [`Endianness`](#endianness)

##### `impl StructuralPartialEq for Endianness`

### `FileKind`

```rust
enum FileKind {
    Archive,
    Coff,
    CoffBig,
    CoffImport,
    DyldCache,
    Elf32,
    Elf64,
    MachO32,
    MachO64,
    MachOFat32,
    MachOFat64,
    Pe32,
    Pe64,
    Wasm,
    Xcoff32,
    Xcoff64,
}
```

A file format kind.

#### Variants

- **`Archive`**

  A Unix archive.
  
  See [`archive::ArchiveFile`](read/archive/index.md).

- **`Coff`**

  A COFF object file.
  
  See [`coff::CoffFile`](read/coff/index.md).

- **`CoffBig`**

  A COFF bigobj object file.
  
  This supports a larger number of sections.
  
  See [`coff::CoffBigFile`](read/coff/index.md).

- **`CoffImport`**

  A Windows short import file.
  
  See [`coff::ImportFile`](read/coff/index.md).

- **`DyldCache`**

  A dyld cache file containing Mach-O images.
  
  See [`macho::DyldCache`](read/macho/index.md)

- **`Elf32`**

  A 32-bit ELF file.
  
  See [`elf::ElfFile32`](read/elf/index.md).

- **`Elf64`**

  A 64-bit ELF file.
  
  See [`elf::ElfFile64`](read/elf/index.md).

- **`MachO32`**

  A 32-bit Mach-O file.
  
  See [`macho::MachOFile32`](read/macho/index.md).

- **`MachO64`**

  A 64-bit Mach-O file.
  
  See [`macho::MachOFile64`](read/macho/index.md).

- **`MachOFat32`**

  A 32-bit Mach-O fat binary.
  
  See [`macho::MachOFatFile32`](read/macho/index.md).

- **`MachOFat64`**

  A 64-bit Mach-O fat binary.
  
  See [`macho::MachOFatFile64`](read/macho/index.md).

- **`Pe32`**

  A 32-bit PE file.
  
  See [`pe::PeFile32`](read/pe/index.md).

- **`Pe64`**

  A 64-bit PE file.
  
  See [`pe::PeFile64`](read/pe/index.md).

- **`Wasm`**

  A Wasm file.
  
  See [`wasm::WasmFile`](read/wasm/index.md).

- **`Xcoff32`**

  A 32-bit XCOFF file.
  
  See [`xcoff::XcoffFile32`](read/xcoff/index.md).

- **`Xcoff64`**

  A 64-bit XCOFF file.
  
  See [`xcoff::XcoffFile64`](read/xcoff/index.md).

#### Implementations

- <span id="filekind-parse"></span>`fn parse<'data, R: ReadRef<'data>>(data: R) -> Result<FileKind>` — [`Result`](#result), [`FileKind`](#filekind)

  Determine a file kind by parsing the start of the file.

- <span id="filekind-parse-at"></span>`fn parse_at<'data, R: ReadRef<'data>>(data: R, offset: u64) -> Result<FileKind>` — [`Result`](#result), [`FileKind`](#filekind)

  Determine a file kind by parsing at the given offset.

#### Trait Implementations

##### `impl Clone for FileKind`

- <span id="filekind-clone"></span>`fn clone(&self) -> FileKind` — [`FileKind`](#filekind)

##### `impl Copy for FileKind`

##### `impl Debug for FileKind`

- <span id="filekind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for FileKind`

##### `impl<K> Equivalent for FileKind`

- <span id="filekind-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Hash for FileKind`

- <span id="filekind-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for FileKind`

- <span id="filekind-partialeq-eq"></span>`fn eq(&self, other: &FileKind) -> bool` — [`FileKind`](#filekind)

##### `impl StructuralPartialEq for FileKind`

### `ObjectKind`

```rust
enum ObjectKind {
    Unknown,
    Relocatable,
    Executable,
    Dynamic,
    Core,
}
```

An object kind.

Returned by `Object::kind`.

#### Variants

- **`Unknown`**

  The object kind is unknown.

- **`Relocatable`**

  Relocatable object.

- **`Executable`**

  Executable.

- **`Dynamic`**

  Dynamic shared object.

- **`Core`**

  Core.

#### Trait Implementations

##### `impl Clone for ObjectKind`

- <span id="objectkind-clone"></span>`fn clone(&self) -> ObjectKind` — [`ObjectKind`](#objectkind)

##### `impl Copy for ObjectKind`

##### `impl Debug for ObjectKind`

- <span id="objectkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ObjectKind`

##### `impl<K> Equivalent for ObjectKind`

- <span id="objectkind-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Hash for ObjectKind`

- <span id="objectkind-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for ObjectKind`

- <span id="objectkind-partialeq-eq"></span>`fn eq(&self, other: &ObjectKind) -> bool` — [`ObjectKind`](#objectkind)

##### `impl StructuralPartialEq for ObjectKind`

### `SymbolSection`

```rust
enum SymbolSection {
    Unknown,
    None,
    Undefined,
    Absolute,
    Common,
    Section(SectionIndex),
}
```

The section where an [`ObjectSymbol`](read/index.md) is defined.

#### Variants

- **`Unknown`**

  The section is unknown.

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

- <span id="symbolsection-index"></span>`fn index(self) -> Option<SectionIndex>` — [`SectionIndex`](#sectionindex)

  Returns the section index for the section where the symbol is defined.

  

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

### `RelocationTarget`

```rust
enum RelocationTarget {
    Symbol(SymbolIndex),
    Section(SectionIndex),
    Absolute,
}
```

The target referenced by a [`Relocation`](#relocation).

#### Variants

- **`Symbol`**

  The target is a symbol.

- **`Section`**

  The target is a section.

- **`Absolute`**

  The offset is an absolute address.

#### Trait Implementations

##### `impl Clone for RelocationTarget`

- <span id="relocationtarget-clone"></span>`fn clone(&self) -> RelocationTarget` — [`RelocationTarget`](#relocationtarget)

##### `impl Copy for RelocationTarget`

##### `impl Debug for RelocationTarget`

- <span id="relocationtarget-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for RelocationTarget`

##### `impl<K> Equivalent for RelocationTarget`

- <span id="relocationtarget-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Hash for RelocationTarget`

- <span id="relocationtarget-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for RelocationTarget`

- <span id="relocationtarget-partialeq-eq"></span>`fn eq(&self, other: &RelocationTarget) -> bool` — [`RelocationTarget`](#relocationtarget)

##### `impl StructuralPartialEq for RelocationTarget`

### `CompressionFormat`

```rust
enum CompressionFormat {
    None,
    Unknown,
    Zlib,
    Zstandard,
}
```

A data compression format.

#### Variants

- **`None`**

  The data is uncompressed.

- **`Unknown`**

  The data is compressed, but the compression format is unknown.

- **`Zlib`**

  ZLIB/DEFLATE.
  
  Used for ELF compression and GNU compressed debug information.

- **`Zstandard`**

  Zstandard.
  
  Used for ELF compression.

#### Trait Implementations

##### `impl Clone for CompressionFormat`

- <span id="compressionformat-clone"></span>`fn clone(&self) -> CompressionFormat` — [`CompressionFormat`](#compressionformat)

##### `impl Copy for CompressionFormat`

##### `impl Debug for CompressionFormat`

- <span id="compressionformat-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for CompressionFormat`

##### `impl<K> Equivalent for CompressionFormat`

- <span id="compressionformat-equivalent"></span>`fn equivalent(&self, key: &K) -> bool`

##### `impl Hash for CompressionFormat`

- <span id="compressionformat-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for CompressionFormat`

- <span id="compressionformat-partialeq-eq"></span>`fn eq(&self, other: &CompressionFormat) -> bool` — [`CompressionFormat`](#compressionformat)

##### `impl StructuralPartialEq for CompressionFormat`

## Traits

### `Endian`

```rust
trait Endian: Debug + Default + Clone + Copy + PartialEq + Eq + 'static { ... }
```

A trait for using an endianness specification.

Provides methods for converting between the specified endianness and
the native endianness of the target machine.

This trait does not require that the endianness is known at compile time.

#### Required Methods

- `fn from_big_endian(big_endian: bool) -> Option<Self>`

  Construct a specification for the endianness of some values.

- `fn is_big_endian(self) -> bool`

  Return true for big endian byte order.

#### Provided Methods

- `fn from_little_endian(little_endian: bool) -> Option<Self>`

  Construct a specification for the endianness of some values.

- `fn is_little_endian(self) -> bool`

  Return true for little endian byte order.

- `fn read_u16(self, n: u16) -> u16`

  Converts an unsigned 16 bit integer to native endian.

- `fn read_u32(self, n: u32) -> u32`

  Converts an unsigned 32 bit integer to native endian.

- `fn read_u64(self, n: u64) -> u64`

  Converts an unsigned 64 bit integer to native endian.

- `fn read_i16(self, n: i16) -> i16`

  Converts a signed 16 bit integer to native endian.

- `fn read_i32(self, n: i32) -> i32`

  Converts a signed 32 bit integer to native endian.

- `fn read_i64(self, n: i64) -> i64`

  Converts a signed 64 bit integer to native endian.

- `fn read_u16_bytes(self, n: [u8; 2]) -> u16`

  Converts an unaligned unsigned 16 bit integer to native endian.

- `fn read_u32_bytes(self, n: [u8; 4]) -> u32`

  Converts an unaligned unsigned 32 bit integer to native endian.

- `fn read_u64_bytes(self, n: [u8; 8]) -> u64`

  Converts an unaligned unsigned 64 bit integer to native endian.

- `fn read_i16_bytes(self, n: [u8; 2]) -> i16`

  Converts an unaligned signed 16 bit integer to native endian.

- `fn read_i32_bytes(self, n: [u8; 4]) -> i32`

  Converts an unaligned signed 32 bit integer to native endian.

- `fn read_i64_bytes(self, n: [u8; 8]) -> i64`

  Converts an unaligned signed 64 bit integer to native endian.

- `fn write_u16(self, n: u16) -> u16`

  Converts an unsigned 16 bit integer from native endian.

- `fn write_u32(self, n: u32) -> u32`

  Converts an unsigned 32 bit integer from native endian.

- `fn write_u64(self, n: u64) -> u64`

  Converts an unsigned 64 bit integer from native endian.

- `fn write_i16(self, n: i16) -> i16`

  Converts a signed 16 bit integer from native endian.

- `fn write_i32(self, n: i32) -> i32`

  Converts a signed 32 bit integer from native endian.

- `fn write_i64(self, n: i64) -> i64`

  Converts a signed 64 bit integer from native endian.

- `fn write_u16_bytes(self, n: u16) -> [u8; 2]`

  Converts an unaligned unsigned 16 bit integer from native endian.

- `fn write_u32_bytes(self, n: u32) -> [u8; 4]`

  Converts an unaligned unsigned 32 bit integer from native endian.

- `fn write_u64_bytes(self, n: u64) -> [u8; 8]`

  Converts an unaligned unsigned 64 bit integer from native endian.

- `fn write_i16_bytes(self, n: i16) -> [u8; 2]`

  Converts an unaligned signed 16 bit integer from native endian.

- `fn write_i32_bytes(self, n: i32) -> [u8; 4]`

  Converts an unaligned signed 32 bit integer from native endian.

- `fn write_i64_bytes(self, n: i64) -> [u8; 8]`

  Converts an unaligned signed 64 bit integer from native endian.

#### Implementors

- [`BigEndian`](#bigendian)
- [`Endianness`](#endianness)
- [`LittleEndian`](#littleendian)

### `Pod`

```rust
trait Pod: Copy + 'static { ... }
```

A trait for types that can safely be converted from and to byte slices.

# Safety
A type that is `Pod` must:
- be `#[repr(C)]` or `#[repr(transparent)]`
- have no invalid byte values
- have no padding

#### Implementors

- [`AixFileHeader`](archive/index.md#aixfileheader)
- [`AixHeader`](archive/index.md#aixheader)
- [`AixMemberOffset`](archive/index.md#aixmemberoffset)
- [`AnonObjectHeaderBigobj`](pe/index.md#anonobjectheaderbigobj)
- [`AnonObjectHeaderV2`](pe/index.md#anonobjectheaderv2)
- [`AnonObjectHeader`](pe/index.md#anonobjectheader)
- [`AuxHeader32`](xcoff/index.md#auxheader32)
- [`AuxHeader64`](xcoff/index.md#auxheader64)
- [`BlockAux32`](xcoff/index.md#blockaux32)
- [`BlockAux64`](xcoff/index.md#blockaux64)
- [`BuildToolVersion`](macho/index.md#buildtoolversion)
- [`BuildVersionCommand`](macho/index.md#buildversioncommand)
- [`CompressionHeader32`](elf/index.md#compressionheader32)
- [`CompressionHeader64`](elf/index.md#compressionheader64)
- [`CsectAux32`](xcoff/index.md#csectaux32)
- [`CsectAux64`](xcoff/index.md#csectaux64)
- [`DataInCodeEntry`](macho/index.md#dataincodeentry)
- [`DwarfAux32`](xcoff/index.md#dwarfaux32)
- [`DwarfAux64`](xcoff/index.md#dwarfaux64)
- [`DyldCacheHeader`](macho/index.md#dyldcacheheader)
- [`DyldCacheImageInfo`](macho/index.md#dyldcacheimageinfo)
- [`DyldCacheMappingAndSlideInfo`](macho/index.md#dyldcachemappingandslideinfo)
- [`DyldCacheMappingInfo`](macho/index.md#dyldcachemappinginfo)
- [`DyldCacheSlideInfo2`](macho/index.md#dyldcacheslideinfo2)
- [`DyldCacheSlideInfo3`](macho/index.md#dyldcacheslideinfo3)
- [`DyldCacheSlideInfo5`](macho/index.md#dyldcacheslideinfo5)
- [`DyldInfoCommand`](macho/index.md#dyldinfocommand)
- [`DyldSubCacheEntryV1`](macho/index.md#dyldsubcacheentryv1)
- [`DyldSubCacheEntryV2`](macho/index.md#dyldsubcacheentryv2)
- [`DylibCommand`](macho/index.md#dylibcommand)
- [`DylibModule32`](macho/index.md#dylibmodule32)
- [`DylibModule64`](macho/index.md#dylibmodule64)
- [`DylibReference`](macho/index.md#dylibreference)
- [`DylibTableOfContents`](macho/index.md#dylibtableofcontents)
- [`Dylib`](macho/index.md#dylib)
- [`DylinkerCommand`](macho/index.md#dylinkercommand)
- [`Dyn32`](elf/index.md#dyn32)
- [`Dyn64`](elf/index.md#dyn64)
- [`DysymtabCommand`](macho/index.md#dysymtabcommand)
- [`EncryptionInfoCommand32`](macho/index.md#encryptioninfocommand32)
- [`EncryptionInfoCommand64`](macho/index.md#encryptioninfocommand64)
- [`EntryPointCommand`](macho/index.md#entrypointcommand)
- [`ExpAux`](xcoff/index.md#expaux)
- [`FatArch32`](macho/index.md#fatarch32)
- [`FatArch64`](macho/index.md#fatarch64)
- [`FatHeader`](macho/index.md#fatheader)
- [`FileAux32`](xcoff/index.md#fileaux32)
- [`FileAux64`](xcoff/index.md#fileaux64)
- [`FileHeader32`](elf/index.md#fileheader32)
- [`FileHeader32`](xcoff/index.md#fileheader32)
- [`FileHeader64`](elf/index.md#fileheader64)
- [`FileHeader64`](xcoff/index.md#fileheader64)
- [`FilesetEntryCommand`](macho/index.md#filesetentrycommand)
- [`FunAux32`](xcoff/index.md#funaux32)
- [`FunAux64`](xcoff/index.md#funaux64)
- [`FvmfileCommand`](macho/index.md#fvmfilecommand)
- [`FvmlibCommand`](macho/index.md#fvmlibcommand)
- [`Fvmlib`](macho/index.md#fvmlib)
- [`GnuHashHeader`](elf/index.md#gnuhashheader)
- [`Guid`](pe/index.md#guid)
- [`HashHeader`](elf/index.md#hashheader)
- [`Header`](archive/index.md#header)
- [`I16Bytes`](#i16bytes)
- [`I32Bytes`](#i32bytes)
- [`I64Bytes`](#i64bytes)
- [`IdentCommand`](macho/index.md#identcommand)
- [`ImageAlpha64RuntimeFunctionEntry`](pe/index.md#imagealpha64runtimefunctionentry)
- [`ImageAlphaRuntimeFunctionEntry`](pe/index.md#imagealpharuntimefunctionentry)
- [`ImageArchitectureEntry`](pe/index.md#imagearchitectureentry)
- [`ImageArchiveMemberHeader`](pe/index.md#imagearchivememberheader)
- [`ImageArm64RuntimeFunctionEntry`](pe/index.md#imagearm64runtimefunctionentry)
- [`ImageArmRuntimeFunctionEntry`](pe/index.md#imagearmruntimefunctionentry)
- [`ImageAuxSymbolCrc`](pe/index.md#imageauxsymbolcrc)
- [`ImageAuxSymbolFunctionBeginEnd`](pe/index.md#imageauxsymbolfunctionbeginend)
- [`ImageAuxSymbolFunction`](pe/index.md#imageauxsymbolfunction)
- [`ImageAuxSymbolSection`](pe/index.md#imageauxsymbolsection)
- [`ImageAuxSymbolTokenDef`](pe/index.md#imageauxsymboltokendef)
- [`ImageAuxSymbolWeak`](pe/index.md#imageauxsymbolweak)
- [`ImageBaseRelocation`](pe/index.md#imagebaserelocation)
- [`ImageBoundForwarderRef`](pe/index.md#imageboundforwarderref)
- [`ImageBoundImportDescriptor`](pe/index.md#imageboundimportdescriptor)
- [`ImageCoffSymbolsHeader`](pe/index.md#imagecoffsymbolsheader)
- [`ImageCor20Header`](pe/index.md#imagecor20header)
- [`ImageDataDirectory`](pe/index.md#imagedatadirectory)
- [`ImageDebugDirectory`](pe/index.md#imagedebugdirectory)
- [`ImageDebugMisc`](pe/index.md#imagedebugmisc)
- [`ImageDelayloadDescriptor`](pe/index.md#imagedelayloaddescriptor)
- [`ImageDosHeader`](pe/index.md#imagedosheader)
- [`ImageDynamicRelocation32V2`](pe/index.md#imagedynamicrelocation32v2)
- [`ImageDynamicRelocation32`](pe/index.md#imagedynamicrelocation32)
- [`ImageDynamicRelocation64V2`](pe/index.md#imagedynamicrelocation64v2)
- [`ImageDynamicRelocation64`](pe/index.md#imagedynamicrelocation64)
- [`ImageDynamicRelocationTable`](pe/index.md#imagedynamicrelocationtable)
- [`ImageEnclaveConfig32`](pe/index.md#imageenclaveconfig32)
- [`ImageEnclaveConfig64`](pe/index.md#imageenclaveconfig64)
- [`ImageEnclaveImport`](pe/index.md#imageenclaveimport)
- [`ImageEpilogueDynamicRelocationHeader`](pe/index.md#imageepiloguedynamicrelocationheader)
- [`ImageExportDirectory`](pe/index.md#imageexportdirectory)
- [`ImageFileHeader`](pe/index.md#imagefileheader)
- [`ImageFunctionEntry64`](pe/index.md#imagefunctionentry64)
- [`ImageFunctionEntry`](pe/index.md#imagefunctionentry)
- [`ImageHotPatchBase`](pe/index.md#imagehotpatchbase)
- [`ImageHotPatchHashes`](pe/index.md#imagehotpatchhashes)
- [`ImageHotPatchInfo`](pe/index.md#imagehotpatchinfo)
- [`ImageImportByName`](pe/index.md#imageimportbyname)
- [`ImageImportDescriptor`](pe/index.md#imageimportdescriptor)
- [`ImageLinenumber`](pe/index.md#imagelinenumber)
- [`ImageLoadConfigCodeIntegrity`](pe/index.md#imageloadconfigcodeintegrity)
- [`ImageLoadConfigDirectory32`](pe/index.md#imageloadconfigdirectory32)
- [`ImageLoadConfigDirectory64`](pe/index.md#imageloadconfigdirectory64)
- [`ImageNtHeaders32`](pe/index.md#imagentheaders32)
- [`ImageNtHeaders64`](pe/index.md#imagentheaders64)
- [`ImageOptionalHeader32`](pe/index.md#imageoptionalheader32)
- [`ImageOptionalHeader64`](pe/index.md#imageoptionalheader64)
- [`ImageOs2Header`](pe/index.md#imageos2header)
- [`ImagePrologueDynamicRelocationHeader`](pe/index.md#imageprologuedynamicrelocationheader)
- [`ImageRelocation`](pe/index.md#imagerelocation)
- [`ImageResourceDataEntry`](pe/index.md#imageresourcedataentry)
- [`ImageResourceDirStringU`](pe/index.md#imageresourcedirstringu)
- [`ImageResourceDirectoryEntry`](pe/index.md#imageresourcedirectoryentry)
- [`ImageResourceDirectoryString`](pe/index.md#imageresourcedirectorystring)
- [`ImageResourceDirectory`](pe/index.md#imageresourcedirectory)
- [`ImageRomHeaders`](pe/index.md#imageromheaders)
- [`ImageRomOptionalHeader`](pe/index.md#imageromoptionalheader)
- [`ImageRuntimeFunctionEntry`](pe/index.md#imageruntimefunctionentry)
- [`ImageSectionHeader`](pe/index.md#imagesectionheader)
- [`ImageSeparateDebugHeader`](pe/index.md#imageseparatedebugheader)
- [`ImageSymbolBytes`](pe/index.md#imagesymbolbytes)
- [`ImageSymbolExBytes`](pe/index.md#imagesymbolexbytes)
- [`ImageSymbolEx`](pe/index.md#imagesymbolex)
- [`ImageSymbol`](pe/index.md#imagesymbol)
- [`ImageThunkData32`](pe/index.md#imagethunkdata32)
- [`ImageThunkData64`](pe/index.md#imagethunkdata64)
- [`ImageTlsDirectory32`](pe/index.md#imagetlsdirectory32)
- [`ImageTlsDirectory64`](pe/index.md#imagetlsdirectory64)
- [`ImageVxdHeader`](pe/index.md#imagevxdheader)
- [`ImportObjectHeader`](pe/index.md#importobjectheader)
- [`LcStr`](macho/index.md#lcstr)
- [`LinkeditDataCommand`](macho/index.md#linkeditdatacommand)
- [`LinkerOptionCommand`](macho/index.md#linkeroptioncommand)
- [`LoadCommand`](macho/index.md#loadcommand)
- [`MachHeader32`](macho/index.md#machheader32)
- [`MachHeader64`](macho/index.md#machheader64)
- [`MaskedRichHeaderEntry`](pe/index.md#maskedrichheaderentry)
- [`Nlist32`](macho/index.md#nlist32)
- [`Nlist64`](macho/index.md#nlist64)
- [`NonPagedDebugInfo`](pe/index.md#nonpageddebuginfo)
- [`NoteCommand`](macho/index.md#notecommand)
- [`NoteHeader32`](elf/index.md#noteheader32)
- [`NoteHeader64`](elf/index.md#noteheader64)
- [`PrebindCksumCommand`](macho/index.md#prebindcksumcommand)
- [`PreboundDylibCommand`](macho/index.md#prebounddylibcommand)
- [`ProgramHeader32`](elf/index.md#programheader32)
- [`ProgramHeader64`](elf/index.md#programheader64)
- [`Rel32`](elf/index.md#rel32)
- [`Rel32`](xcoff/index.md#rel32)
- [`Rel64`](elf/index.md#rel64)
- [`Rel64`](xcoff/index.md#rel64)
- [`Rela32`](elf/index.md#rela32)
- [`Rela64`](elf/index.md#rela64)
- [`Relocation`](macho/index.md#relocation)
- [`Relr32`](elf/index.md#relr32)
- [`Relr64`](elf/index.md#relr64)
- [`RoutinesCommand32`](macho/index.md#routinescommand32)
- [`RoutinesCommand64`](macho/index.md#routinescommand64)
- [`RpathCommand`](macho/index.md#rpathcommand)
- [`Section32`](macho/index.md#section32)
- [`Section64`](macho/index.md#section64)
- [`SectionHeader32`](elf/index.md#sectionheader32)
- [`SectionHeader32`](xcoff/index.md#sectionheader32)
- [`SectionHeader64`](elf/index.md#sectionheader64)
- [`SectionHeader64`](xcoff/index.md#sectionheader64)
- [`SegmentCommand32`](macho/index.md#segmentcommand32)
- [`SegmentCommand64`](macho/index.md#segmentcommand64)
- [`SourceVersionCommand`](macho/index.md#sourceversioncommand)
- [`StatAux`](xcoff/index.md#stataux)
- [`SubClientCommand`](macho/index.md#subclientcommand)
- [`SubFrameworkCommand`](macho/index.md#subframeworkcommand)
- [`SubLibraryCommand`](macho/index.md#sublibrarycommand)
- [`SubUmbrellaCommand`](macho/index.md#subumbrellacommand)
- [`Sym32`](elf/index.md#sym32)
- [`Sym64`](elf/index.md#sym64)
- [`Symbol32`](xcoff/index.md#symbol32)
- [`Symbol64`](xcoff/index.md#symbol64)
- [`SymbolBytes`](xcoff/index.md#symbolbytes)
- [`Syminfo32`](elf/index.md#syminfo32)
- [`Syminfo64`](elf/index.md#syminfo64)
- [`SymsegCommand`](macho/index.md#symsegcommand)
- [`SymtabCommand`](macho/index.md#symtabcommand)
- [`ThreadCommand`](macho/index.md#threadcommand)
- [`TwolevelHint`](macho/index.md#twolevelhint)
- [`TwolevelHintsCommand`](macho/index.md#twolevelhintscommand)
- [`U16Bytes`](#u16bytes)
- [`U32Bytes`](#u32bytes)
- [`U64Bytes`](#u64bytes)
- [`UuidCommand`](macho/index.md#uuidcommand)
- [`Verdaux`](elf/index.md#verdaux)
- [`Verdef`](elf/index.md#verdef)
- [`Vernaux`](elf/index.md#vernaux)
- [`Verneed`](elf/index.md#verneed)
- [`VersionMinCommand`](macho/index.md#versionmincommand)
- [`Versym`](elf/index.md#versym)
- `[T; N]`
- `u16`
- `u32`
- `u64`
- `u8`

### `ReadError<T>`

```rust
trait ReadError<T> { ... }
```

#### Required Methods

- `fn read_error(self, error: &'static str) -> Result<T>`

#### Implementors

- `Option<T>`
- `result::Result<T, ()>`
- `result::Result<T, Error>`
- `wasmparser::Result<T>`

### `SymbolMapEntry`

```rust
trait SymbolMapEntry { ... }
```

An entry in a [`SymbolMap`](#symbolmap).

#### Required Methods

- `fn address(&self) -> u64`

  The symbol address.

#### Implementors

- [`ObjectMapEntry`](#objectmapentry)
- [`SymbolMapName`](#symbolmapname)

## Functions

### `from_bytes`

```rust
fn from_bytes<T: Pod>(data: &[u8]) -> result::Result<(&T, &[u8]), ()>
```

Cast the head of a byte slice to a `Pod` type.

Returns the type and the tail of the byte slice.

Returns an error if the byte slice is too short or the alignment is invalid.

### `from_bytes_mut`

```rust
fn from_bytes_mut<T: Pod>(data: &mut [u8]) -> result::Result<(&mut T, &mut [u8]), ()>
```

Cast the head of a mutable byte slice to a `Pod` type.

Returns the type and the tail of the byte slice.

Returns an error if the byte slice is too short or the alignment is invalid.

### `slice_from_bytes`

```rust
fn slice_from_bytes<T: Pod>(data: &[u8], count: usize) -> result::Result<(&[T], &[u8]), ()>
```

Cast the head of a byte slice to a slice of a `Pod` type.

Returns the type slice and the tail of the byte slice.

Returns an error if the byte slice is too short or the alignment is invalid.

### `slice_from_bytes_mut`

```rust
fn slice_from_bytes_mut<T: Pod>(data: &mut [u8], count: usize) -> result::Result<(&mut [T], &mut [u8]), ()>
```

Cast the head of a mutable byte slice to a slice of a `Pod` type.

Returns the type slice and the tail of the byte slice.

Returns an error if the byte slice is too short or the alignment is invalid.

### `slice_from_all_bytes`

```rust
fn slice_from_all_bytes<T: Pod>(data: &[u8]) -> result::Result<&[T], ()>
```

Cast all of a byte slice to a slice of a `Pod` type.

Returns the type slice.

Returns an error if the size of the byte slice is not an exact multiple
of the type size, or the alignment is invalid.

### `slice_from_all_bytes_mut`

```rust
fn slice_from_all_bytes_mut<T: Pod>(data: &mut [u8]) -> result::Result<&mut [T], ()>
```

Cast all of a byte slice to a slice of a `Pod` type.

Returns the type slice.

Returns an error if the size of the byte slice is not an exact multiple
of the type size, or the alignment is invalid.

### `bytes_of`

```rust
fn bytes_of<T: Pod>(val: &T) -> &[u8]
```

Cast a `Pod` type to a byte slice.

### `bytes_of_mut`

```rust
fn bytes_of_mut<T: Pod>(val: &mut T) -> &mut [u8]
```

Cast a `Pod` type to a mutable byte slice.

### `bytes_of_slice`

```rust
fn bytes_of_slice<T: Pod>(val: &[T]) -> &[u8]
```

Cast a slice of a `Pod` type to a byte slice.

### `bytes_of_slice_mut`

```rust
fn bytes_of_slice_mut<T: Pod>(val: &mut [T]) -> &mut [u8]
```

Cast a slice of a `Pod` type to a mutable byte slice.

## Type Aliases

### `NativeEndian`

```rust
type NativeEndian = LittleEndian;
```

The native endianness for the target platform.

### `U16<E>`

```rust
type U16<E> = U16Bytes<E>;
```

A `u16` value with an externally specified endianness of type `E`.

### `U32<E>`

```rust
type U32<E> = U32Bytes<E>;
```

A `u32` value with an externally specified endianness of type `E`.

### `U64<E>`

```rust
type U64<E> = U64Bytes<E>;
```

A `u64` value with an externally specified endianness of type `E`.

### `I16<E>`

```rust
type I16<E> = I16Bytes<E>;
```

An `i16` value with an externally specified endianness of type `E`.

### `I32<E>`

```rust
type I32<E> = I32Bytes<E>;
```

An `i32` value with an externally specified endianness of type `E`.

### `I64<E>`

```rust
type I64<E> = I64Bytes<E>;
```

An `i64` value with an externally specified endianness of type `E`.

### `Result<T>`

```rust
type Result<T> = result::Result<T, ()>;
```

### `Result<T>`

```rust
type Result<T> = result::Result<T, Error>;
```

The result type used within the read module.

### `NativeFile<'data, R>`

```rust
type NativeFile<'data, R> = elf::ElfFile64<'data, crate::endian::Endianness, R>;
```

The native executable file for the target platform.

## Macros

### `unsafe_impl_endian_pod!`

### `unsafe_impl_pod!`

