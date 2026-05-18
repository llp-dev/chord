*[wasmparser](../../../index.md) / [readers](../../index.md) / [core](../index.md) / [dylink0](index.md)*

---

# Module `dylink0`

## Contents

- [Structs](#structs)
  - [`MemInfo`](#meminfo)
  - [`ExportInfo`](#exportinfo)
  - [`ImportInfo`](#importinfo)
- [Enums](#enums)
  - [`Dylink0Subsection`](#dylink0subsection)
- [Type Aliases](#type-aliases)
  - [`Dylink0SectionReader`](#dylink0sectionreader)
- [Constants](#constants)
  - [`WASM_DYLINK_MEM_INFO`](#wasm-dylink-mem-info)
  - [`WASM_DYLINK_NEEDED`](#wasm-dylink-needed)
  - [`WASM_DYLINK_EXPORT_INFO`](#wasm-dylink-export-info)
  - [`WASM_DYLINK_IMPORT_INFO`](#wasm-dylink-import-info)
  - [`WASM_DYLINK_RUNTIME_PATH`](#wasm-dylink-runtime-path)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`MemInfo`](#meminfo) | struct | Represents a `WASM_DYLINK_MEM_INFO` field |
| [`ExportInfo`](#exportinfo) | struct |  |
| [`ImportInfo`](#importinfo) | struct |  |
| [`Dylink0Subsection`](#dylink0subsection) | enum | Possible subsections of the `dylink.0` custom section. |
| [`Dylink0SectionReader`](#dylink0sectionreader) | type | Parser for the dynamic linking `dylink.0` custom section. |
| [`WASM_DYLINK_MEM_INFO`](#wasm-dylink-mem-info) | const |  |
| [`WASM_DYLINK_NEEDED`](#wasm-dylink-needed) | const |  |
| [`WASM_DYLINK_EXPORT_INFO`](#wasm-dylink-export-info) | const |  |
| [`WASM_DYLINK_IMPORT_INFO`](#wasm-dylink-import-info) | const |  |
| [`WASM_DYLINK_RUNTIME_PATH`](#wasm-dylink-runtime-path) | const |  |

## Structs

### `MemInfo`

```rust
struct MemInfo {
    pub memory_size: u32,
    pub memory_alignment: u32,
    pub table_size: u32,
    pub table_alignment: u32,
}
```

Represents a `WASM_DYLINK_MEM_INFO` field

#### Fields

- **`memory_size`**: `u32`

  Size of the memory area the loader should reserve for the module, which
  will begin at `env.__memory_base`.

- **`memory_alignment`**: `u32`

  The required alignment of the memory area, in bytes, encoded as a power
  of 2.

- **`table_size`**: `u32`

  Size of the table area the loader should reserve for the module, which
  will begin at `env.__table_base`.

- **`table_alignment`**: `u32`

  The required alignment of the table area, in elements, encoded as a
  power of 2.

#### Trait Implementations

##### `impl Clone for MemInfo`

- <span id="meminfo-clone"></span>`fn clone(&self) -> MemInfo` — [`MemInfo`](../index.md#meminfo)

##### `impl Copy for MemInfo`

##### `impl Debug for MemInfo`

- <span id="meminfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ExportInfo<'a>`

```rust
struct ExportInfo<'a> {
    pub name: &'a str,
    pub flags: crate::SymbolFlags,
}
```

#### Trait Implementations

##### `impl Debug for ExportInfo<'a>`

- <span id="exportinfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

### `ImportInfo<'a>`

```rust
struct ImportInfo<'a> {
    pub module: &'a str,
    pub field: &'a str,
    pub flags: crate::SymbolFlags,
}
```

#### Trait Implementations

##### `impl Debug for ImportInfo<'a>`

- <span id="importinfo-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Enums

### `Dylink0Subsection<'a>`

```rust
enum Dylink0Subsection<'a> {
    MemInfo(MemInfo),
    Needed(Vec<&'a str>),
    ExportInfo(Vec<ExportInfo<'a>>),
    ImportInfo(Vec<ImportInfo<'a>>),
    RuntimePath(Vec<&'a str>),
    Unknown {
        ty: u8,
        data: &'a [u8],
        range: core::ops::Range<usize>,
    },
}
```

Possible subsections of the `dylink.0` custom section.

#### Trait Implementations

##### `impl Debug for Dylink0Subsection<'a>`

- <span id="dylink0subsection-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Subsection for Dylink0Subsection<'a>`

- <span id="dylink0subsection-subsection-from-reader"></span>`fn from_reader(id: u8, reader: BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader), [`Result`](../../../binary_reader/index.md#result)

## Type Aliases

### `Dylink0SectionReader<'a>`

```rust
type Dylink0SectionReader<'a> = crate::Subsections<'a, Dylink0Subsection<'a>>;
```

Parser for the dynamic linking `dylink.0` custom section.

This format is currently defined upstream at
<https://github.com/WebAssembly/tool-conventions/blob/main/DynamicLinking.md>.

## Constants

### `WASM_DYLINK_MEM_INFO`
```rust
const WASM_DYLINK_MEM_INFO: u8 = 1u8;
```

### `WASM_DYLINK_NEEDED`
```rust
const WASM_DYLINK_NEEDED: u8 = 2u8;
```

### `WASM_DYLINK_EXPORT_INFO`
```rust
const WASM_DYLINK_EXPORT_INFO: u8 = 3u8;
```

### `WASM_DYLINK_IMPORT_INFO`
```rust
const WASM_DYLINK_IMPORT_INFO: u8 = 4u8;
```

### `WASM_DYLINK_RUNTIME_PATH`
```rust
const WASM_DYLINK_RUNTIME_PATH: u8 = 5u8;
```

