**object > read > macho > exports_trie**

# Module: read::macho::exports_trie

## Contents

**Structs**

- [`ExportSymbol`](#exportsymbol) - Exported symbol information.
- [`ExportsTrieIterator`](#exportstrieiterator) - Iterator over the exports trie.

**Enums**

- [`ExportData`](#exportdata) - Terminal data for an exports trie node.

---

## object::read::macho::exports_trie::ExportData

*Enum*

Terminal data for an exports trie node.

**Generic Parameters:**
- 'data

**Variants:**
- `Regular{ address: u64 }` - A regular export.
- `Reexport{ dylib_ordinal: u64, import_name: &'data [u8] }` - A re-exported symbol.
- `StubAndResolver{ stub_address: u64, resolver_address: u64 }` - A stub-and-resolver symbol.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::read::macho::exports_trie::ExportSymbol

*Struct*

Exported symbol information.

**Generic Parameters:**
- 'data

**Methods:**

- `fn name(self: &Self) -> &[u8]` - The name of the exported symbol.
- `fn flags(self: &Self) -> u8` - The flags for the exported symbol.
- `fn data(self: &Self) -> &ExportData<'data>` - The terminal data for the exported symbol.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`



## object::read::macho::exports_trie::ExportsTrieIterator

*Struct*

Iterator over the exports trie.

**Generic Parameters:**
- 'data

**Methods:**

- `fn next(self: & mut Self) -> Result<Option<ExportSymbol<'data>>>` - Returns the next exported symbol, if any.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Iterator**
  - `fn next(self: & mut Self) -> Option<<Self as >::Item>`



