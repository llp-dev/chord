**object**

# Module: object

## Contents

**Modules**

- [`archive`](#archive) - Archive definitions.
- [`build`](#build) - Interface for building object files.
- [`elf`](#elf) - ELF definitions.
- [`endian`](#endian) - Types for compile-time and run-time endianness.
- [`macho`](#macho) - Mach-O definitions.
- [`pe`](#pe) - PE/COFF definitions.
- [`pod`](#pod) - Tools for converting file format structures to and from bytes.
- [`read`](#read) - Interface for reading object files.
- [`write`](#write) - Interface for writing object files.
- [`xcoff`](#xcoff) - XCOFF definitions

---

## Module: archive

Archive definitions.

These definitions are independent of read/write support, although we do implement
some traits useful for those.



## Module: build

Interface for building object files.

This module provides common types and traits used in the builders.

The submodules define the builders for each file format.



## Module: elf

ELF definitions.

These definitions are independent of read/write support, although we do implement
some traits useful for those.

This module is the equivalent of /usr/include/elf.h, and is based heavily on it.



## Module: endian

Types for compile-time and run-time endianness.



## Module: macho

Mach-O definitions.

These definitions are independent of read/write support, although we do implement
some traits useful for those.

This module is based heavily on header files from MacOSX11.1.sdk.



## Module: pe

PE/COFF definitions.

These definitions are independent of read/write support, although we do implement
some traits useful for those.

This module is based heavily on "winnt.h" (10.0.17763.0).



## Module: pod

Tools for converting file format structures to and from bytes.

This module should be replaced once rust provides safe transmutes.



## Module: read

Interface for reading object files.

## Unified read API

The [`Object`] trait provides a unified read API for accessing common features of
object files, such as sections and symbols. There is an implementation of this
trait for [`File`], which allows reading any file format, as well as implementations
for each file format:
[`ElfFile`](elf::ElfFile), [`MachOFile`](macho::MachOFile), [`CoffFile`](coff::CoffFile),
[`PeFile`](pe::PeFile), [`WasmFile`](wasm::WasmFile), [`XcoffFile`](xcoff::XcoffFile).

## Low level read API

The submodules for each file format define helpers that operate on the raw structs.
These can be used instead of the unified API, or in conjunction with it to access
details that are not available via the unified API.

See the [submodules](#modules) for examples of the low level read API.

## Naming Convention

Types that form part of the unified API for a file format are prefixed with the
name of the file format.

## Example for unified read API
 ```no_run
use object::{Object, ObjectSection};
use std::error::Error;
use std::fs;

/// Reads a file and displays the name of each section.
fn main() -> Result<(), Box<dyn Error>> {
#   #[cfg(all(feature = "read", feature = "std"))] {
    let data = fs::read("path/to/binary")?;
    let file = object::File::parse(&*data)?;
    for section in file.sections() {
        println!("{}", section.name()?);
    }
#   }
    Ok(())
}
```



## Module: write

Interface for writing object files.

This module provides a unified write API for relocatable object files
using [`Object`]. This does not support writing executable files.
This supports the following file formats: COFF, ELF, Mach-O, and XCOFF.

The submodules define helpers for writing the raw structs. These support
writing both relocatable and executable files. There are writers for
the following file formats: [COFF](coff::Writer), [ELF](elf::Writer),
and [PE](pe::Writer).



## Module: xcoff

XCOFF definitions

These definitions are independent of read/write support, although we do implement
some traits useful for those.

This module is the equivalent of /usr/include/xcoff.h, and is based heavily on it.



