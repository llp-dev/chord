**object > read > traits**

# Module: read::traits

## Contents

**Structs**

- [`NoDynamicRelocationIterator`](#nodynamicrelocationiterator) - An iterator for files that don't have dynamic relocations.

**Traits**

- [`Object`](#object) - An object file.
- [`ObjectComdat`](#objectcomdat) - A COMDAT section group in an [`Object`].
- [`ObjectSection`](#objectsection) - A section in an [`Object`].
- [`ObjectSegment`](#objectsegment) - A loadable segment in an [`Object`].
- [`ObjectSymbol`](#objectsymbol) - A symbol table entry in an [`Object`].
- [`ObjectSymbolTable`](#objectsymboltable) - A symbol table in an [`Object`].

---

## object::read::traits::NoDynamicRelocationIterator

*Struct*

An iterator for files that don't have dynamic relocations.

**Unit Struct**

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



## object::read::traits::Object

*Trait*

An object file.

This is the primary trait for the unified read API.

**Methods:**

- `Segment`: A loadable segment in the object file.
- `SegmentIterator`: An iterator for the loadable segments in the object file.
- `Section`: A section in the object file.
- `SectionIterator`: An iterator for the sections in the object file.
- `Comdat`: A COMDAT section group in the object file.
- `ComdatIterator`: An iterator for the COMDAT section groups in the object file.
- `Symbol`: A symbol in the object file.
- `SymbolIterator`: An iterator for symbols in the object file.
- `SymbolTable`: A symbol table in the object file.
- `DynamicRelocationIterator`: An iterator for the dynamic relocations in the file.
- `architecture`: Get the architecture type of the file.
- `sub_architecture`: Get the sub-architecture type of the file if known.
- `endianness`: Get the endianness of the file.
- `is_little_endian`: Return true if the file is little endian, false if it is big endian.
- `is_64`: Return true if the file can contain 64-bit addresses.
- `kind`: Return the kind of this object.
- `segments`: Get an iterator for the loadable segments in the file.
- `section_by_name`: Get the section named `section_name`, if such a section exists.
- `section_by_name_bytes`: Like [`Self::section_by_name`], but allows names that are not UTF-8.
- `section_by_index`: Get the section at the given index.
- `sections`: Get an iterator for the sections in the file.
- `comdats`: Get an iterator for the COMDAT section groups in the file.
- `symbol_table`: Get the debugging symbol table, if any.
- `symbol_by_index`: Get the debugging symbol at the given index.
- `symbols`: Get an iterator for the debugging symbols in the file.
- `symbol_by_name`: Get the symbol named `symbol_name`, if the symbol exists.
- `symbol_by_name_bytes`: Like [`Self::symbol_by_name`], but allows names that are not UTF-8.
- `dynamic_symbol_table`: Get the dynamic linking symbol table, if any.
- `dynamic_symbols`: Get an iterator for the dynamic linking symbols in the file.
- `dynamic_relocations`: Get the dynamic relocations for this file.
- `symbol_map`: Construct a map from addresses to symbol names.
- `object_map`: Construct a map from addresses to symbol names and object file names.
- `imports`: Get the imported symbols.
- `exports`: Get the exported symbols that expose both a name and an address.
- `has_debug_symbols`: Return true if the file contains DWARF debug information sections, false if not.
- `mach_uuid`: The UUID from a Mach-O [`LC_UUID`](crate::macho::LC_UUID) load command.
- `build_id`: The build ID from an ELF [`NT_GNU_BUILD_ID`](crate::elf::NT_GNU_BUILD_ID) note.
- `gnu_debuglink`: The filename and CRC from a `.gnu_debuglink` section.
- `gnu_debugaltlink`: The filename and build ID from a `.gnu_debugaltlink` section.
- `pdb_info`: The filename and GUID from the PE CodeView section.
- `relative_address_base`: Get the base address used for relative virtual addresses.
- `entry`: Get the virtual address of the entry point of the binary.
- `flags`: File flags that are specific to each file format.



## object::read::traits::ObjectComdat

*Trait*

A COMDAT section group in an [`Object`].

This trait is part of the unified read API.

**Methods:**

- `SectionIterator`: An iterator for the sections in the section group.
- `kind`: Returns the COMDAT selection kind.
- `symbol`: Returns the index of the symbol used for the name of COMDAT section group.
- `name_bytes`: Returns the name of the COMDAT section group.
- `name`: Returns the name of the COMDAT section group.
- `sections`: Get the sections in this section group.



## object::read::traits::ObjectSection

*Trait*

A section in an [`Object`].

This trait is part of the unified read API.

**Methods:**

- `RelocationIterator`: An iterator for the relocations for a section.
- `index`: Returns the section index.
- `address`: Returns the address of the section.
- `size`: Returns the size of the section in memory.
- `align`: Returns the alignment of the section in memory.
- `file_range`: Returns offset and size of on-disk segment (if any).
- `data`: Returns the raw contents of the section.
- `data_range`: Return the raw contents of the section data in the given range.
- `compressed_file_range`: Returns the potentially compressed file range of the section,
- `compressed_data`: Returns the potentially compressed contents of the section,
- `uncompressed_data`: Returns the uncompressed contents of the section.
- `name_bytes`: Returns the name of the section.
- `name`: Returns the name of the section.
- `segment_name_bytes`: Returns the name of the segment for this section.
- `segment_name`: Returns the name of the segment for this section.
- `kind`: Return the kind of this section.
- `relocations`: Get the relocations for this section.
- `relocation_map`: Construct a relocation map for this section.
- `flags`: Section flags that are specific to each file format.



## object::read::traits::ObjectSegment

*Trait*

A loadable segment in an [`Object`].

This trait is part of the unified read API.

**Methods:**

- `address`: Returns the virtual address of the segment.
- `size`: Returns the size of the segment in memory.
- `align`: Returns the alignment of the segment in memory.
- `file_range`: Returns the offset and size of the segment in the file.
- `data`: Returns a reference to the file contents of the segment.
- `data_range`: Return the segment data in the given range.
- `name_bytes`: Returns the name of the segment.
- `name`: Returns the name of the segment.
- `flags`: Return the flags of segment.



## object::read::traits::ObjectSymbol

*Trait*

A symbol table entry in an [`Object`].

This trait is part of the unified read API.

**Methods:**

- `index`: The index of the symbol.
- `name_bytes`: The name of the symbol.
- `name`: The name of the symbol.
- `address`: The address of the symbol. May be zero if the address is unknown.
- `size`: The size of the symbol. May be zero if the size is unknown.
- `kind`: Return the kind of this symbol.
- `section`: Returns the section where the symbol is defined.
- `section_index`: Returns the section index for the section containing this symbol.
- `is_undefined`: Return true if the symbol is undefined.
- `is_definition`: Return true if the symbol is a definition of a function or data object
- `is_common`: Return true if the symbol is common data.
- `is_weak`: Return true if the symbol is weak.
- `scope`: Returns the symbol scope.
- `is_global`: Return true if the symbol visible outside of the compilation unit.
- `is_local`: Return true if the symbol is only visible within the compilation unit.
- `flags`: Symbol flags that are specific to each file format.



## object::read::traits::ObjectSymbolTable

*Trait*

A symbol table in an [`Object`].

This trait is part of the unified read API.

**Methods:**

- `Symbol`: A symbol table entry.
- `SymbolIterator`: An iterator for the symbols in a symbol table.
- `symbols`: Get an iterator for the symbols in the table.
- `symbol_by_index`: Get the symbol at the given index.



