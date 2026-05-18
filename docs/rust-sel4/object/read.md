**object > read**

# Module: read

## Contents

**Modules**

- [`archive`](#archive) - Support for archive files.
- [`coff`](#coff) - Support for reading Windows COFF files.
- [`elf`](#elf) - Support for reading ELF files.
- [`macho`](#macho) - Support for reading Mach-O files.
- [`pe`](#pe) - Support for reading PE files.
- [`wasm`](#wasm) - Support for reading Wasm files.
- [`xcoff`](#xcoff) - Support for reading AIX XCOFF files.

**Structs**

- [`CodeView`](#codeview) - PDB information from the debug directory in a PE file.
- [`CompressedData`](#compresseddata) - Data that may be compressed.
- [`CompressedFileRange`](#compressedfilerange) - A range in a file that may be compressed.
- [`Error`](#error) - The error type used within the read module.
- [`Export`](#export) - An exported symbol.
- [`Import`](#import) - An imported symbol.
- [`ObjectMap`](#objectmap) - A map from addresses to symbol names and object files.
- [`ObjectMapEntry`](#objectmapentry) - A symbol in an [`ObjectMap`].
- [`ObjectMapFile`](#objectmapfile) - An object file name in an [`ObjectMap`].
- [`Relocation`](#relocation) - A relocation entry.
- [`RelocationMap`](#relocationmap) - A map from section offsets to relocation information.
- [`SectionIndex`](#sectionindex) - The index used to identify a section in a file.
- [`SymbolIndex`](#symbolindex) - The index used to identify a symbol in a symbol table.
- [`SymbolMap`](#symbolmap) - A map from addresses to symbol information.
- [`SymbolMapName`](#symbolmapname) - The type used for entries in a [`SymbolMap`] that maps from addresses to names.

**Enums**

- [`CompressionFormat`](#compressionformat) - A data compression format.
- [`FileKind`](#filekind) - A file format kind.
- [`ObjectKind`](#objectkind) - An object kind.
- [`RelocationTarget`](#relocationtarget) - The target referenced by a [`Relocation`].
- [`SymbolSection`](#symbolsection) - The section where an [`ObjectSymbol`] is defined.

**Traits**

- [`SymbolMapEntry`](#symbolmapentry) - An entry in a [`SymbolMap`].

**Type Aliases**

- [`NativeFile`](#nativefile) - The native executable file for the target platform.
- [`Result`](#result) - The result type used within the read module.

---

## object::read::CodeView

*Struct*

PDB information from the debug directory in a PE file.

**Generic Parameters:**
- 'data

**Methods:**

- `fn path(self: &Self) -> &'data [u8]` - The path to the PDB as stored in CodeView.
- `fn age(self: &Self) -> u32` - The age of the PDB.
- `fn guid(self: &Self) -> [u8; 16]` - The GUID of the PDB.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> CodeView<'data>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &CodeView<'data>) -> bool`



## object::read::CompressedData

*Struct*

Data that may be compressed.

Returned by [`ObjectSection::compressed_data`].

**Generic Parameters:**
- 'data

**Fields:**
- `format: CompressionFormat` - The data compression format.
- `data: &'data [u8]` - The compressed data.
- `uncompressed_size: u64` - The uncompressed data size.

**Methods:**

- `fn none(data: &'data [u8]) -> Self` - Data that is uncompressed.
- `fn decompress(self: Self) -> Result<Cow<'data, [u8]>>` - Return the uncompressed data.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &CompressedData<'data>) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> CompressedData<'data>`



## object::read::CompressedFileRange

*Struct*

A range in a file that may be compressed.

Returned by [`ObjectSection::compressed_file_range`].

**Fields:**
- `format: CompressionFormat` - The data compression format.
- `offset: u64` - The file offset of the compressed data.
- `compressed_size: u64` - The compressed data size.
- `uncompressed_size: u64` - The uncompressed data size.

**Methods:**

- `fn none(range: Option<(u64, u64)>) -> Self` - Data that is uncompressed.
- `fn data<'data, R>(self: Self, file: R) -> Result<CompressedData<'data>>` - Convert to [`CompressedData`] by reading from the file.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> CompressedFileRange`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &CompressedFileRange) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## object::read::CompressionFormat

*Enum*

A data compression format.

**Variants:**
- `None` - The data is uncompressed.
- `Unknown` - The data is compressed, but the compression format is unknown.
- `Zlib` - ZLIB/DEFLATE.
- `Zstandard` - Zstandard.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &CompressionFormat) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> CompressionFormat`



## object::read::Error

*Struct*

The error type used within the read module.

**Tuple Struct**: `()`

**Traits:** Eq, Error, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Error) -> bool`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Clone**
  - `fn clone(self: &Self) -> Error`



## object::read::Export

*Struct*

An exported symbol.

Returned by [`Object::exports`].

**Generic Parameters:**
- 'data

**Methods:**

- `fn name(self: &Self) -> &'data [u8]` - The symbol name.
- `fn address(self: &Self) -> u64` - The virtual address of the symbol.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Export<'data>) -> bool`
- **Clone**
  - `fn clone(self: &Self) -> Export<'data>`



## object::read::FileKind

*Enum*

A file format kind.

**Variants:**
- `Archive` - A Unix archive.
- `Coff` - A COFF object file.
- `CoffBig` - A COFF bigobj object file.
- `CoffImport` - A Windows short import file.
- `DyldCache` - A dyld cache file containing Mach-O images.
- `Elf32` - A 32-bit ELF file.
- `Elf64` - A 64-bit ELF file.
- `MachO32` - A 32-bit Mach-O file.
- `MachO64` - A 64-bit Mach-O file.
- `MachOFat32` - A 32-bit Mach-O fat binary.
- `MachOFat64` - A 64-bit Mach-O fat binary.
- `Pe32` - A 32-bit PE file.
- `Pe64` - A 64-bit PE file.
- `Wasm` - A Wasm file.
- `Xcoff32` - A 32-bit XCOFF file.
- `Xcoff64` - A 64-bit XCOFF file.

**Methods:**

- `fn parse<'data, R>(data: R) -> Result<FileKind>` - Determine a file kind by parsing the start of the file.
- `fn parse_at<'data, R>(data: R, offset: u64) -> Result<FileKind>` - Determine a file kind by parsing at the given offset.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> FileKind`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &FileKind) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## object::read::Import

*Struct*

An imported symbol.

Returned by [`Object::imports`].

**Generic Parameters:**
- 'data

**Methods:**

- `fn name(self: &Self) -> &'data [u8]` - The symbol name.
- `fn library(self: &Self) -> &'data [u8]` - The name of the library to import the symbol from.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> Import<'data>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &Import<'data>) -> bool`



## object::read::NativeFile

*Type Alias*: `elf::ElfFile64<'data, crate::endian::Endianness, R>`

The native executable file for the target platform.



## object::read::ObjectKind

*Enum*

An object kind.

Returned by [`Object::kind`].

**Variants:**
- `Unknown` - The object kind is unknown.
- `Relocatable` - Relocatable object.
- `Executable` - Executable.
- `Dynamic` - Dynamic shared object.
- `Core` - Core.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &ObjectKind) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> ObjectKind`



## object::read::ObjectMap

*Struct*

A map from addresses to symbol names and object files.

This is derived from STAB entries in Mach-O files.

Returned by [`Object::object_map`].

**Generic Parameters:**
- 'data

**Methods:**

- `fn get(self: &Self, address: u64) -> Option<&ObjectMapEntry<'data>>` - Get the entry containing the given address.
- `fn symbols(self: &Self) -> &[ObjectMapEntry<'data>]` - Get all symbols in the map.
- `fn objects(self: &Self) -> &[ObjectMapFile<'data>]` - Get all objects in the map.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> ObjectMap<'data>`
- **Clone**
  - `fn clone(self: &Self) -> ObjectMap<'data>`



## object::read::ObjectMapEntry

*Struct*

A symbol in an [`ObjectMap`].

**Generic Parameters:**
- 'data

**Methods:**

- `fn address(self: &Self) -> u64` - Get the symbol address.
- `fn size(self: &Self) -> u64` - Get the symbol size.
- `fn name(self: &Self) -> &'data [u8]` - Get the symbol name.
- `fn object_index(self: &Self) -> usize` - Get the index of the object file name.
- `fn object<'a>(self: &Self, map: &'a ObjectMap<'data>) -> &'a ObjectMapFile<'data>` - Get the object file name.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Default**
  - `fn default() -> ObjectMapEntry<'data>`
- **PartialEq**
  - `fn eq(self: &Self, other: &ObjectMapEntry<'data>) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **SymbolMapEntry**
  - `fn address(self: &Self) -> u64`
- **Clone**
  - `fn clone(self: &Self) -> ObjectMapEntry<'data>`



## object::read::ObjectMapFile

*Struct*

An object file name in an [`ObjectMap`].

**Generic Parameters:**
- 'data

**Methods:**

- `fn path(self: &Self) -> &'data [u8]` - Get the path to the file containing the object.
- `fn member(self: &Self) -> Option<&'data [u8]>` - If the file is an archive, get the name of the member containing the object.

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> ObjectMapFile<'data>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &ObjectMapFile<'data>) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## object::read::Relocation

*Struct*

A relocation entry.

Returned by [`Object::dynamic_relocations`] or [`ObjectSection::relocations`].

**Methods:**

- `fn kind(self: &Self) -> RelocationKind` - The operation used to calculate the result of the relocation.
- `fn encoding(self: &Self) -> RelocationEncoding` - Information about how the result of the relocation operation is encoded in the place.
- `fn size(self: &Self) -> u8` - The size in bits of the place of the relocation.
- `fn target(self: &Self) -> RelocationTarget` - The target of the relocation.
- `fn addend(self: &Self) -> i64` - The addend to use in the relocation calculation.
- `fn set_addend(self: & mut Self, addend: i64)` - Set the addend to use in the relocation calculation.
- `fn has_implicit_addend(self: &Self) -> bool` - Returns true if there is an implicit addend stored in the data at the offset
- `fn flags(self: &Self) -> RelocationFlags` - Relocation flags that are specific to each file format.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::read::RelocationMap

*Struct*

A map from section offsets to relocation information.

This can be used to apply relocations to a value at a given section offset.
This is intended for use with DWARF in relocatable object files, and only
supports relocations that are used in DWARF.

Returned by [`ObjectSection::relocation_map`].

**Tuple Struct**: `()`

**Methods:**

- `fn new<'data, 'file, T>(file: &'file T, section: &<T as >::Section) -> Result<Self>` - Construct a new relocation map for a section.
- `fn add<'data, 'file, T>(self: & mut Self, file: &'file T, offset: u64, relocation: Relocation) -> Result<()>` - Add a single relocation to the map.
- `fn relocate(self: &Self, offset: u64, value: u64) -> u64` - Relocate a value that was read from the section at the given offset.

**Trait Implementations:**

- **Default**
  - `fn default() -> RelocationMap`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::read::RelocationTarget

*Enum*

The target referenced by a [`Relocation`].

**Variants:**
- `Symbol(SymbolIndex)` - The target is a symbol.
- `Section(SectionIndex)` - The target is a section.
- `Absolute` - The offset is an absolute address.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &RelocationTarget) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> RelocationTarget`



## object::read::Result

*Type Alias*: `result::Result<T, Error>`

The result type used within the read module.



## object::read::SectionIndex

*Struct*

The index used to identify a section in a file.

**Tuple Struct**: `(usize)`

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &SectionIndex) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> SectionIndex`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`



## object::read::SymbolIndex

*Struct*

The index used to identify a symbol in a symbol table.

**Tuple Struct**: `(usize)`

**Traits:** Eq, Copy

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> SymbolIndex`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &SymbolIndex) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`



## object::read::SymbolMap

*Struct*

A map from addresses to symbol information.

The symbol information depends on the chosen entry type, such as [`SymbolMapName`].

Returned by [`Object::symbol_map`].

**Generic Parameters:**
- T

**Methods:**

- `fn new(symbols: Vec<T>) -> Self` - Construct a new symbol map.
- `fn get(self: &Self, address: u64) -> Option<&T>` - Get the symbol before the given address.
- `fn symbols(self: &Self) -> &[T]` - Get all symbols in the map.

**Trait Implementations:**

- **Default**
  - `fn default() -> SymbolMap<T>`
- **Clone**
  - `fn clone(self: &Self) -> SymbolMap<T>`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::read::SymbolMapEntry

*Trait*

An entry in a [`SymbolMap`].

**Methods:**

- `address`: The symbol address.



## object::read::SymbolMapName

*Struct*

The type used for entries in a [`SymbolMap`] that maps from addresses to names.

**Generic Parameters:**
- 'data

**Methods:**

- `fn new(address: u64, name: &'data str) -> Self` - Construct a `SymbolMapName`.
- `fn address(self: &Self) -> u64` - The symbol address.
- `fn name(self: &Self) -> &'data str` - The symbol name.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &SymbolMapName<'data>) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **SymbolMapEntry**
  - `fn address(self: &Self) -> u64`
- **Clone**
  - `fn clone(self: &Self) -> SymbolMapName<'data>`



## object::read::SymbolSection

*Enum*

The section where an [`ObjectSymbol`] is defined.

**Variants:**
- `Unknown` - The section is unknown.
- `None` - The section is not applicable for this symbol (such as file symbols).
- `Undefined` - The symbol is undefined.
- `Absolute` - The symbol has an absolute value.
- `Common` - The symbol is a zero-initialized symbol that will be combined with duplicate definitions.
- `Section(SectionIndex)` - The symbol is defined in the given section.

**Methods:**

- `fn index(self: Self) -> Option<SectionIndex>` - Returns the section index for the section where the symbol is defined.

**Traits:** Copy, Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **PartialEq**
  - `fn eq(self: &Self, other: &SymbolSection) -> bool`
- **Hash**
  - `fn hash<__H>(self: &Self, state: & mut __H)`
- **Clone**
  - `fn clone(self: &Self) -> SymbolSection`



## Module: archive

Support for archive files.

## Example
 ```no_run
use object::{Object, ObjectSection};
use std::error::Error;
use std::fs;

/// Reads an archive and displays the name of each member.
fn main() -> Result<(), Box<dyn Error>> {
#   #[cfg(feature = "std")] {
    let data = fs::read("path/to/binary")?;
    let file = object::read::archive::ArchiveFile::parse(&*data)?;
    for member in file.members() {
        let member = member?;
        println!("{}", String::from_utf8_lossy(member.name()));
    }
#   }
    Ok(())
}
```



## Module: coff

Support for reading Windows COFF files.

Traits are used to abstract over the difference between COFF object files
and COFF bigobj files. The primary trait for this is [`CoffHeader`].

## High level API

[`CoffFile`] implements the [`Object`](crate::read::Object) trait for
COFF files. [`CoffFile`] is parameterised by [`CoffHeader`].
The default parameter allows reading regular COFF object files,
while the type alias [`CoffBigFile`] allows reading COFF bigobj files.

[`ImportFile`] allows reading COFF short imports that are used in import
libraries. Currently these are not integrated with the unified read API.

## Low level API

The [`CoffHeader`] trait can be directly used to parse both COFF
object files (which start with [`pe::ImageFileHeader`]) and COFF bigobj
files (which start with [`pe::AnonObjectHeaderBigobj`]).

### Example for low level API
 ```no_run
use object::pe;
use object::read::coff::{CoffHeader, ImageSymbol as _};
use std::error::Error;
use std::fs;

/// Reads a file and displays the name of each section and symbol.
fn main() -> Result<(), Box<dyn Error>> {
#   #[cfg(feature = "std")] {
    let data = fs::read("path/to/binary")?;
    let mut offset = 0;
    let header = pe::ImageFileHeader::parse(&*data, &mut offset)?;
    let sections = header.sections(&*data, offset)?;
    let symbols = header.symbols(&*data)?;
    for section in sections.iter() {
        println!("{}", String::from_utf8_lossy(section.name(symbols.strings())?));
    }
    for (_index, symbol) in symbols.iter() {
        println!("{}", String::from_utf8_lossy(symbol.name(symbols.strings())?));
    }
#   }
    Ok(())
}
```



## Module: elf

Support for reading ELF files.

Traits are used to abstract over the difference between 32-bit and 64-bit ELF.
The primary trait for this is [`FileHeader`].

## High level API

[`ElfFile`] implements the [`Object`](crate::read::Object) trait for ELF files.
[`ElfFile`] is parameterised by [`FileHeader`] to allow reading both 32-bit and
64-bit ELF. There are type aliases for these parameters ([`ElfFile32`] and
[`ElfFile64`]).

## Low level API

The [`FileHeader`] trait can be directly used to parse both [`elf::FileHeader32`]
and [`elf::FileHeader64`].

### Example for low level API
 ```no_run
use object::elf;
use object::read::elf::{FileHeader, Sym};
use std::error::Error;
use std::fs;

/// Reads a file and displays the name of each symbol.
fn main() -> Result<(), Box<dyn Error>> {
#   #[cfg(feature = "std")] {
    let data = fs::read("path/to/binary")?;
    let elf = elf::FileHeader64::<object::Endianness>::parse(&*data)?;
    let endian = elf.endian()?;
    let sections = elf.sections(endian, &*data)?;
    let symbols = sections.symbols(endian, &*data, elf::SHT_SYMTAB)?;
    for symbol in symbols.iter() {
        let name = symbol.name(endian, symbols.strings())?;
        println!("{}", String::from_utf8_lossy(name));
    }
#   }
    Ok(())
}
```



## Module: macho

Support for reading Mach-O files.

Traits are used to abstract over the difference between 32-bit and 64-bit Mach-O
files. The primary trait for this is [`MachHeader`].

## High level API

[`MachOFile`] implements the [`Object`](crate::read::Object) trait for Mach-O files.
[`MachOFile`] is parameterised by [`MachHeader`] to allow reading both 32-bit and
64-bit Mach-O files. There are type aliases for these parameters ([`MachOFile32`] and
[`MachOFile64`]).

## Low level API

The [`MachHeader`] trait can be directly used to parse both [`macho::MachHeader32`]
and [`macho::MachHeader64`]. Additionally, [`FatHeader`] and the [`FatArch`] trait
can be used to iterate images in multi-architecture binaries, and [`DyldCache`] can
be used to locate images in a dyld shared cache.

### Example for low level API
 ```no_run
use object::macho;
use object::read::macho::{MachHeader, Nlist};
use std::error::Error;
use std::fs;

/// Reads a file and displays the name of each symbol.
fn main() -> Result<(), Box<dyn Error>> {
#   #[cfg(feature = "std")] {
    let data = fs::read("path/to/binary")?;
    let header = macho::MachHeader64::<object::Endianness>::parse(&*data, 0)?;
    let endian = header.endian()?;
    let mut commands = header.load_commands(endian, &*data, 0)?;
    while let Some(command) = commands.next()? {
        if let Some(symtab_command) = command.symtab()? {
            let symbols = symtab_command.symbols::<macho::MachHeader64<_>, _>(endian, &*data)?;
            for symbol in symbols.iter() {
                let name = symbol.name(endian, symbols.strings())?;
                println!("{}", String::from_utf8_lossy(name));
            }
        }
    }
#   }
    Ok(())
}
```



## Module: pe

Support for reading PE files.

Traits are used to abstract over the difference between PE32 and PE32+.
The primary trait for this is [`ImageNtHeaders`].

## High level API

[`PeFile`] implements the [`Object`](crate::read::Object) trait for
PE files. [`PeFile`] is parameterised by [`ImageNtHeaders`] to allow
reading both PE32 and PE32+. There are type aliases for these parameters
([`PeFile32`] and [`PeFile64`]).

## Low level API

The [`ImageNtHeaders`] trait can be directly used to parse both
[`pe::ImageNtHeaders32`] and [`pe::ImageNtHeaders64`].

### Example for low level API
 ```no_run
use object::pe;
use object::read::pe::ImageNtHeaders;
use std::error::Error;
use std::fs;

/// Reads a file and displays the name of each section.
fn main() -> Result<(), Box<dyn Error>> {
#   #[cfg(feature = "std")] {
    let data = fs::read("path/to/binary")?;
    let dos_header = pe::ImageDosHeader::parse(&*data)?;
    let mut offset = dos_header.nt_headers_offset().into();
    let (nt_headers, data_directories) = pe::ImageNtHeaders64::parse(&*data, &mut offset)?;
    let sections = nt_headers.sections(&*data, offset)?;
    let symbols = nt_headers.symbols(&*data)?;
    for section in sections.iter() {
        println!("{}", String::from_utf8_lossy(section.name(symbols.strings())?));
    }
#   }
    Ok(())
}
```



## Module: wasm

Support for reading Wasm files.

[`WasmFile`] implements the [`Object`] trait for Wasm files.



## Module: xcoff

Support for reading AIX XCOFF files.

Traits are used to abstract over the difference between 32-bit and 64-bit XCOFF.
The primary trait for this is [`FileHeader`].

## High level API

[`XcoffFile`] implements the [`Object`](crate::read::Object) trait for XCOFF files.
[`XcoffFile`] is parameterised by [`FileHeader`] to allow reading both 32-bit and
64-bit XCOFF. There are type aliases for these parameters ([`XcoffFile32`] and
[`XcoffFile64`]).

## Low level API

The [`FileHeader`] trait can be directly used to parse both [`xcoff::FileHeader32`]
and [`xcoff::FileHeader64`].

### Example for low level API
 ```no_run
use object::xcoff;
use object::read::xcoff::{FileHeader, SectionHeader, Symbol};
use std::error::Error;
use std::fs;

/// Reads a file and displays the name of each section and symbol.
fn main() -> Result<(), Box<dyn Error>> {
#   #[cfg(feature = "std")] {
    let data = fs::read("path/to/binary")?;
    let mut offset = 0;
    let header = xcoff::FileHeader64::parse(&*data, &mut offset)?;
    let aux_header = header.aux_header(&*data, &mut offset)?;
    let sections = header.sections(&*data, &mut offset)?;
    let symbols = header.symbols(&*data)?;
    for section in sections.iter() {
        println!("{}", String::from_utf8_lossy(section.name()));
    }
    for (_index, symbol) in symbols.iter() {
        println!("{}", String::from_utf8_lossy(symbol.name(symbols.strings())?));
    }
#   }
    Ok(())
}
```



