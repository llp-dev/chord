**object > read > elf > hash**

# Module: read::elf::hash

## Contents

**Structs**

- [`GnuHashTable`](#gnuhashtable) - A GNU symbol hash table in an ELF file.
- [`HashTable`](#hashtable) - A SysV symbol hash table in an ELF file.

---

## object::read::elf::hash::GnuHashTable

*Struct*

A GNU symbol hash table in an ELF file.

Returned by [`SectionHeader::gnu_hash`](super::SectionHeader::gnu_hash).

**Generic Parameters:**
- 'data
- Elf

**Methods:**

- `fn parse(endian: <Elf as >::Endian, data: &'data [u8]) -> Result<Self>` - Parse a GNU hash table.
- `fn symbol_base(self: &Self) -> u32` - Return the symbol table index of the first symbol in the hash table.
- `fn symbol_table_length(self: &Self, endian: <Elf as >::Endian) -> Option<u32>` - Determine the symbol table length by finding the last entry in the hash table.
- `fn find<R>(self: &Self, endian: <Elf as >::Endian, name: &[u8], hash: u32, version: Option<&Version>, symbols: &SymbolTable<'data, Elf, R>, versions: &VersionTable<'data, Elf>) -> Option<(SymbolIndex, &'data <Elf as >::Sym)>` - Use the hash table to find the symbol table entry with the given name, hash, and version.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::read::elf::hash::HashTable

*Struct*

A SysV symbol hash table in an ELF file.

Returned by [`SectionHeader::hash`](super::SectionHeader::hash).

**Generic Parameters:**
- 'data
- Elf

**Methods:**

- `fn parse(endian: <Elf as >::Endian, data: &'data [u8]) -> Result<Self>` - Parse a SysV hash table.
- `fn symbol_table_length(self: &Self) -> u32` - Return the symbol table length.
- `fn find<R>(self: &Self, endian: <Elf as >::Endian, name: &[u8], hash: u32, version: Option<&Version>, symbols: &SymbolTable<'data, Elf, R>, versions: &VersionTable<'data, Elf>) -> Option<(SymbolIndex, &'data <Elf as >::Sym)>` - Use the hash table to find the symbol table entry with the given name, hash and version.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



