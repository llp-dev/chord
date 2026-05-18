# object

# `object`

The `object` crate provides a unified interface to working with object files
across platforms. It supports reading relocatable object files and executable files,
and writing relocatable object files and some executable files.

## Raw struct definitions

Raw structs are defined for: [ELF](elf), [Mach-O](macho), [PE/COFF](pe),
[XCOFF](xcoff), [archive].
Types and traits for zerocopy support are defined in the [`pod`] and [`endian`] modules.

## Unified read API

The [`read`] module provides a unified read API using the [`read::Object`] trait.
There is an implementation of this trait for [`read::File`], which allows reading any
file format, as well as implementations for each file format.

## Low level read API

The [`read#modules`] submodules define helpers that operate on the raw structs.
These can be used instead of the unified API, or in conjunction with it to access
details that are not available via the unified API.

## Unified write API

The [`mod@write`] module provides a unified write API for relocatable object files
using [`write::Object`]. This does not support writing executable files.

## Low level write API

The [`mod@write#modules`] submodules define helpers for writing the raw structs.

## Build API

The [`mod@build`] submodules define helpers for building object files, either from
scratch or by modifying existing files.

## Shared definitions

The crate provides a number of definitions that are used by both the read and write
APIs. These are defined at the top level module, but none of these are the main entry
points of the crate.

## Modules

### [`object`](object.md)

*10 modules*

### [`archive`](archive.md)

*4 constants, 4 structs*

### [`build`](build.md)

*1 module*

### [`build::bytes`](build/bytes.md)

*2 structs*

### [`build::elf`](build/elf.md)

*18 structs, 4 enums, 9 type aliases*

### [`build::error`](build/error.md)

*1 struct, 1 type alias*

### [`build::table`](build/table.md)

*2 traits, 3 structs*

### [`build::table::id_private`](build/table/id_private.md)

*1 trait*

### [`common`](common.md)

*15 enums*

### [`elf`](elf.md)

*2946 constants, 30 structs, 4 functions*

### [`endian`](endian.md)

*1 enum, 1 trait, 7 type aliases, 8 structs*

### [`macho`](macho.md)

*1 enum, 3 functions, 490 constants, 67 structs*

### [`pe`](pe.md)

*706 constants, 77 structs*

### [`pod`](pod.md)

*1 trait, 10 functions*

### [`read`](read.md)

*1 trait, 15 structs, 2 type aliases, 5 enums, 7 modules*

### [`read::any`](read/any.md)

*1 enum, 12 structs*

### [`read::archive`](read/archive.md)

*1 enum, 6 structs*

### [`read::coff::comdat`](read/coff/comdat.md)

*3 structs, 3 type aliases*

### [`read::coff::file`](read/coff/file.md)

*1 function, 1 struct, 1 trait, 1 type alias*

### [`read::coff::import`](read/coff/import.md)

*2 enums, 2 structs*

### [`read::coff::relocation`](read/coff/relocation.md)

*1 struct, 1 type alias*

### [`read::coff::section`](read/coff/section.md)

*4 type aliases, 5 structs*

### [`read::coff::symbol`](read/coff/symbol.md)

*1 trait, 3 type aliases, 5 structs*

### [`read::elf::attributes`](read/elf/attributes.md)

*7 structs*

### [`read::elf::comdat`](read/elf/comdat.md)

*3 structs, 6 type aliases*

### [`read::elf::compression`](read/elf/compression.md)

*1 trait*

### [`read::elf::dynamic`](read/elf/dynamic.md)

*1 trait*

### [`read::elf::file`](read/elf/file.md)

*1 struct, 1 trait, 2 type aliases*

### [`read::elf::hash`](read/elf/hash.md)

*2 structs*

### [`read::elf::note`](read/elf/note.md)

*1 trait, 4 structs*

### [`read::elf::relocation`](read/elf/relocation.md)

*3 traits, 4 type aliases, 6 structs*

### [`read::elf::section`](read/elf/section.md)

*1 trait, 3 structs, 4 type aliases*

### [`read::elf::segment`](read/elf/segment.md)

*1 trait, 2 structs, 4 type aliases*

### [`read::elf::symbol`](read/elf/symbol.md)

*1 trait, 4 structs, 6 type aliases*

### [`read::elf::version`](read/elf/version.md)

*7 structs*

### [`read::macho::dyld_cache`](read/macho/dyld_cache.md)

*3 enums, 8 structs*

### [`read::macho::exports_trie`](read/macho/exports_trie.md)

*1 enum, 2 structs*

### [`read::macho::fat`](read/macho/fat.md)

*1 struct, 1 trait, 2 type aliases*

### [`read::macho::file`](read/macho/file.md)

*1 trait, 4 structs, 8 type aliases*

### [`read::macho::function_starts`](read/macho/function_starts.md)

*1 struct*

### [`read::macho::load_command`](read/macho/load_command.md)

*1 enum, 2 structs*

### [`read::macho::relocation`](read/macho/relocation.md)

*1 struct, 2 type aliases*

### [`read::macho::section`](read/macho/section.md)

*1 trait, 2 structs, 4 type aliases*

### [`read::macho::segment`](read/macho/segment.md)

*1 trait, 2 structs, 4 type aliases*

### [`read::macho::symbol`](read/macho/symbol.md)

*1 trait, 4 structs, 6 type aliases*

### [`read::pe::data_directory`](read/pe/data_directory.md)

*1 struct*

### [`read::pe::export`](read/pe/export.md)

*1 enum, 2 structs*

### [`read::pe::file`](read/pe/file.md)

*1 function, 2 traits, 4 structs, 8 type aliases*

### [`read::pe::import`](read/pe/import.md)

*1 enum, 1 trait, 5 structs*

### [`read::pe::relocation`](read/pe/relocation.md)

*3 structs*

### [`read::pe::resource`](read/pe/resource.md)

*2 enums, 3 structs*

### [`read::pe::rich`](read/pe/rich.md)

*2 structs*

### [`read::pe::section`](read/pe/section.md)

*5 structs, 8 type aliases*

### [`read::private`](read/private.md)

*1 trait*

### [`read::read_cache`](read/read_cache.md)

*1 trait, 2 structs*

### [`read::read_ref`](read/read_ref.md)

*1 trait*

### [`read::traits`](read/traits.md)

*1 struct, 6 traits*

### [`read::util`](read/util.md)

*2 structs*

### [`read::wasm`](read/wasm.md)

*12 structs*

### [`read::xcoff::comdat`](read/xcoff/comdat.md)

*3 structs, 6 type aliases*

### [`read::xcoff::file`](read/xcoff/file.md)

*1 struct, 2 traits, 2 type aliases*

### [`read::xcoff::relocation`](read/xcoff/relocation.md)

*1 struct, 1 trait, 2 type aliases*

### [`read::xcoff::section`](read/xcoff/section.md)

*1 trait, 3 structs, 4 type aliases*

### [`read::xcoff::segment`](read/xcoff/segment.md)

*2 structs, 4 type aliases*

### [`read::xcoff::symbol`](read/xcoff/symbol.md)

*3 traits, 5 structs, 6 type aliases*

### [`write`](write.md)

*1 type alias, 3 modules, 4 enums, 9 structs*

### [`write::coff::object`](write/coff/object.md)

*1 enum*

### [`write::coff::writer`](write/coff/writer.md)

*1 enum, 7 structs*

### [`write::elf::writer`](write/elf/writer.md)

*13 structs*

### [`write::macho`](write/macho.md)

*1 struct*

### [`write::pe`](write/pe.md)

*4 structs*

### [`write::string`](write/string.md)

*1 struct*

### [`write::util`](write/util.md)

*1 struct, 1 trait*

### [`xcoff`](xcoff.md)

*154 constants, 23 structs*

