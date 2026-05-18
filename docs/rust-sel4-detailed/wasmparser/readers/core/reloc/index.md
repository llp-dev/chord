*[wasmparser](../../../index.md) / [readers](../../index.md) / [core](../index.md) / [reloc](index.md)*

---

# Module `reloc`

## Contents

- [Structs](#structs)
  - [`RelocSectionReader`](#relocsectionreader)
  - [`RelocationEntry`](#relocationentry)
- [Enums](#enums)
  - [`RelocationType`](#relocationtype)
  - [`RelocAddendKind`](#relocaddendkind)
- [Type Aliases](#type-aliases)
  - [`RelocationEntryReader`](#relocationentryreader)
- [Macros](#macros)
  - [`back_to_enum!`](#back-to-enum)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`RelocSectionReader`](#relocsectionreader) | struct | Reader for reloc.* sections as defined by <https://github.com/WebAssembly/tool-conventions/blob/main/Linking.md#relocation-sections>. |
| [`RelocationEntry`](#relocationentry) | struct | Single relocation entry within a `reloc.*` section, as defined at <https://github.com/WebAssembly/tool-conventions/blob/main/Linking.md#relocation-sections>. |
| [`RelocationType`](#relocationtype) | enum | Relocation entry type. |
| [`RelocAddendKind`](#relocaddendkind) | enum | Indicates the kind of addend that applies to a relocation entry. |
| [`RelocationEntryReader`](#relocationentryreader) | type | Reader for relocation entries within a `reloc.*` section. |
| [`back_to_enum!`](#back-to-enum) | macro |  |

## Structs

### `RelocSectionReader<'a>`

```rust
struct RelocSectionReader<'a> {
    section: u32,
    range: core::ops::Range<usize>,
    entries: crate::SectionLimited<'a, RelocationEntry>,
}
```

Reader for reloc.* sections as defined by
<https://github.com/WebAssembly/tool-conventions/blob/main/Linking.md#relocation-sections>.

#### Implementations

- <span id="relocsectionreader-new"></span>`fn new(reader: BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader), [`Result`](../../../binary_reader/index.md#result)

  Creates a new reader for a `reloc.*` section starting at

  `original_position` within the wasm file.

- <span id="relocsectionreader-section-index"></span>`fn section_index(&self) -> u32`

  Index of section to which the relocations apply.

- <span id="relocsectionreader-range"></span>`fn range(&self) -> Range<usize>`

  The byte range of the entire section.

- <span id="relocsectionreader-entries"></span>`fn entries(&self) -> SectionLimited<'a, RelocationEntry>` — [`SectionLimited`](../../../index.md#sectionlimited), [`RelocationEntry`](../index.md#relocationentry)

  The relocation entries.

#### Trait Implementations

##### `impl Clone for RelocSectionReader<'a>`

- <span id="relocsectionreader-clone"></span>`fn clone(&self) -> RelocSectionReader<'a>` — [`RelocSectionReader`](../index.md#relocsectionreader)

##### `impl Debug for RelocSectionReader<'a>`

- <span id="relocsectionreader-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `RelocationEntry`

```rust
struct RelocationEntry {
    pub ty: RelocationType,
    pub offset: u32,
    pub index: u32,
    pub addend: i64,
}
```

Single relocation entry within a `reloc.*` section, as defined at
<https://github.com/WebAssembly/tool-conventions/blob/main/Linking.md#relocation-sections>.

#### Fields

- **`ty`**: `RelocationType`

  Relocation entry type.

- **`offset`**: `u32`

  Offset in bytes from the start of the section indicated by
  `RelocSectionReader::section` targeted by this relocation.

- **`index`**: `u32`

  Index in the symbol table contained in the linking section that
  corresponds to the value at `offset`.

- **`addend`**: `i64`

  Addend to add to the address, or `0` if not applicable. The value must
  be consistent with the `self.ty.addend_kind()`.

#### Implementations

- <span id="relocationentry-relocation-range"></span>`fn relocation_range(&self) -> Range<usize>`

  Byte range relative to the start of the section indicated by

  `RelocSectionReader::section` targeted by this relocation.

#### Trait Implementations

##### `impl Clone for RelocationEntry`

- <span id="relocationentry-clone"></span>`fn clone(&self) -> RelocationEntry` — [`RelocationEntry`](../index.md#relocationentry)

##### `impl Copy for RelocationEntry`

##### `impl Debug for RelocationEntry`

- <span id="relocationentry-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for RelocationEntry`

##### `impl FromReader for RelocationEntry`

- <span id="relocationentry-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader), [`Result`](../../../binary_reader/index.md#result)

##### `impl Hash for RelocationEntry`

- <span id="relocationentry-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for RelocationEntry`

- <span id="relocationentry-partialeq-eq"></span>`fn eq(&self, other: &RelocationEntry) -> bool` — [`RelocationEntry`](../index.md#relocationentry)

##### `impl StructuralPartialEq for RelocationEntry`

## Enums

### `RelocationType`

```rust
enum RelocationType {
    FunctionIndexLeb,
    TableIndexSleb,
    TableIndexI32,
    MemoryAddrLeb,
    MemoryAddrSleb,
    MemoryAddrI32,
    TypeIndexLeb,
    GlobalIndexLeb,
    FunctionOffsetI32,
    SectionOffsetI32,
    EventIndexLeb,
    MemoryAddrRelSleb,
    TableIndexRelSleb,
    GlobalIndexI32,
    MemoryAddrLeb64,
    MemoryAddrSleb64,
    MemoryAddrI64,
    MemoryAddrRelSleb64,
    TableIndexSleb64,
    TableIndexI64,
    TableNumberLeb,
    MemoryAddrTlsSleb,
    FunctionOffsetI64,
    MemoryAddrLocrelI32,
    TableIndexRelSleb64,
    MemoryAddrTlsSleb64,
    FunctionIndexI32,
}
```

Relocation entry type. Each entry type corresponds to one of the
`R_WASM_*` constants defined at
<https://github.com/llvm/llvm-project/blob/main/llvm/include/llvm/BinaryFormat/WasmRelocs.def>
and
<https://github.com/WebAssembly/tool-conventions/blob/main/Linking.md#relocation-sections>.

#### Variants

- **`FunctionIndexLeb`**

  A function index encoded as a 5-byte varuint32. Used for the
  immediate argument of a call instruction. (since LLVM 10.0)

- **`TableIndexSleb`**

  A function table index encoded as a 5-byte varint32. Used to refer
  to the immediate argument of a i32.const instruction, e.g. taking
  the address of a function. (since LLVM 10.0)

- **`TableIndexI32`**

  A function table index encoded as a uint32, e.g. taking the address
  of a function in a static data initializer. (since LLVM 10.0)

- **`MemoryAddrLeb`**

  A linear memory index encoded as a 5-byte varuint32. Used for the
  immediate argument of a load or store instruction, e.g. directly
  loading from or storing to a C++ global. (since LLVM 10.0)

- **`MemoryAddrSleb`**

  A linear memory index encoded as a 5-byte varint32. Used for the
  immediate argument of a i32.const instruction, e.g. taking the
  address of a C++ global. (since LLVM 10.0)

- **`MemoryAddrI32`**

  A linear memory index encoded as a uint32, e.g. taking the address
  of a C++ global in a static data initializer. (since LLVM 10.0)

- **`TypeIndexLeb`**

  A type index encoded as a 5-byte varuint32, e.g. the type immediate
  in a call_indirect. (since LLVM 10.0)

- **`GlobalIndexLeb`**

  A global index encoded as a 5-byte varuint32, e.g. the index
  immediate in a get_global. (since LLVM 10.0)

- **`FunctionOffsetI32`**

  A byte offset within code section for the specific function encoded
  as a uint32. The offsets start at the actual function code excluding
  its size field. (since LLVM 10.0)

- **`SectionOffsetI32`**

  A byte offset from start of the specified section encoded as a
  uint32. (since LLVM 10.0)

- **`EventIndexLeb`**

  An event index encoded as a 5-byte varuint32. Used for the immediate
  argument of a throw and if_except instruction. (since LLVM 10.0)

- **`MemoryAddrRelSleb`**

  A memory address relative to the __memory_base wasm global. Used in
  position independent code (-fPIC) where absolute memory addresses
  are not known at link time.

- **`TableIndexRelSleb`**

  A function address (table index) relative to the __table_base wasm
  global. Used in position independent code (-fPIC) where absolute
  function addresses are not known at link time.

- **`GlobalIndexI32`**

  A global index encoded as uint32. (since LLVM 11.0)

- **`MemoryAddrLeb64`**

  The 64-bit counterpart of `MemoryAddrLeb`. A 64-bit linear memory
  index encoded as a 10-byte varuint64, Used for the immediate
  argument of a load or store instruction on a 64-bit linear memory
  array. (since LLVM 11.0)

- **`MemoryAddrSleb64`**

  The 64-bit counterpart of `MemoryAddrSleb`. A 64-bit linear memory
  index encoded as a 10-byte varint64. Used for the immediate argument
  of a i64.const instruction. (since LLVM 11.0)

- **`MemoryAddrI64`**

  The 64-bit counterpart of `MemoryAddrI32`. A 64-bit linear memory
  index encoded as a uint64, e.g. taking the 64-bit address of a C++
  global in a static data initializer. (since LLVM 11.0)

- **`MemoryAddrRelSleb64`**

  The 64-bit counterpart of `MemoryAddrRelSleb`.

- **`TableIndexSleb64`**

  The 64-bit counterpart of `TableIndexSleb`. A function table index
  encoded as a 10-byte varint64. Used to refer to the immediate
  argument of a i64.const instruction, e.g. taking the address of a
  function in Wasm64. (in LLVM 12.0)

- **`TableIndexI64`**

  The 64-bit counterpart of `TableIndexI32`. A function table index
  encoded as a uint64, e.g. taking the address of a function in a
  static data initializer. (in LLVM 12.0)

- **`TableNumberLeb`**

  A table number encoded as a 5-byte varuint32. Used for the table
  immediate argument in the table.* instructions. (in LLVM 12.0)

- **`MemoryAddrTlsSleb`**

  An offset from the __tls_base symbol encoded as a 5-byte varint32.
  Used for PIC case to avoid absolute relocation. (in LLVM 12.0)

- **`FunctionOffsetI64`**

  The 64-bit counterpart of `FunctionOffsetI32`. A byte offset within
  code section for the specific function encoded as a uint64. (in LLVM
  12.0)

- **`MemoryAddrLocrelI32`**

  A byte offset between the relocating address and a linear memory
  index encoded as a uint32. Used for pointer-relative addressing. (in
  LLVM 13.0)

- **`TableIndexRelSleb64`**

  The 64-bit counterpart of `TableIndexRelSleb`. A function table
  index encoded as a 10-byte varint64. (in LLVM 13.0)

- **`MemoryAddrTlsSleb64`**

  The 64-bit counterpart of `MemoryAddrTlsSleb`. (in LLVM 13.0)

- **`FunctionIndexI32`**

  A function index encoded as a uint32. Used in custom sections for
  function annotations (`__attribute__((annotate(<name>)))`) (in LLVM
  17.0)

#### Implementations

- <span id="relocationtype-addend-kind"></span>`const fn addend_kind(self) -> RelocAddendKind` — [`RelocAddendKind`](../index.md#relocaddendkind)

  Indicates if this relocation type has an associated `RelocEntry::addend`.

- <span id="relocationtype-extent"></span>`const fn extent(self) -> usize`

  Indicates the number of bytes that this relocation type targets.

#### Trait Implementations

##### `impl Clone for RelocationType`

- <span id="relocationtype-clone"></span>`fn clone(&self) -> RelocationType` — [`RelocationType`](../index.md#relocationtype)

##### `impl Copy for RelocationType`

##### `impl Debug for RelocationType`

- <span id="relocationtype-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for RelocationType`

##### `impl FromReader for RelocationType`

- <span id="relocationtype-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader), [`Result`](../../../binary_reader/index.md#result)

##### `impl Hash for RelocationType`

- <span id="relocationtype-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for RelocationType`

- <span id="relocationtype-partialeq-eq"></span>`fn eq(&self, other: &RelocationType) -> bool` — [`RelocationType`](../index.md#relocationtype)

##### `impl StructuralPartialEq for RelocationType`

### `RelocAddendKind`

```rust
enum RelocAddendKind {
    None,
    Addend32,
    Addend64,
}
```

Indicates the kind of addend that applies to a relocation entry.

#### Variants

- **`None`**

  Relocation entry does not include an addend.

- **`Addend32`**

  Relocation entry includes a 32-bit addend.

- **`Addend64`**

  Relocation entry includes a 64-bit addend.

#### Trait Implementations

##### `impl Clone for RelocAddendKind`

- <span id="relocaddendkind-clone"></span>`fn clone(&self) -> RelocAddendKind` — [`RelocAddendKind`](../index.md#relocaddendkind)

##### `impl Copy for RelocAddendKind`

##### `impl Debug for RelocAddendKind`

- <span id="relocaddendkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for RelocAddendKind`

##### `impl Hash for RelocAddendKind`

- <span id="relocaddendkind-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for RelocAddendKind`

- <span id="relocaddendkind-partialeq-eq"></span>`fn eq(&self, other: &RelocAddendKind) -> bool` — [`RelocAddendKind`](../index.md#relocaddendkind)

##### `impl StructuralPartialEq for RelocAddendKind`

## Type Aliases

### `RelocationEntryReader<'a>`

```rust
type RelocationEntryReader<'a> = crate::SectionLimited<'a, RelocationEntry>;
```

Reader for relocation entries within a `reloc.*` section.

## Macros

### `back_to_enum!`

