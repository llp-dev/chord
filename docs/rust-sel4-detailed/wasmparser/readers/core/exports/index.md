*[wasmparser](../../../index.md) / [readers](../../index.md) / [core](../index.md) / [exports](index.md)*

---

# Module `exports`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Export`](#export) | struct | Represents an export in a WebAssembly module. |
| [`ExternalKind`](#externalkind) | enum | External types as defined [here]. |
| [`ExportSectionReader`](#exportsectionreader) | type | A reader for the export section of a WebAssembly module. |

## Structs

### `Export<'a>`

```rust
struct Export<'a> {
    pub name: &'a str,
    pub kind: ExternalKind,
    pub index: u32,
}
```

Represents an export in a WebAssembly module.

#### Fields

- **`name`**: `&'a str`

  The name of the exported item.

- **`kind`**: `ExternalKind`

  The kind of the export.

- **`index`**: `u32`

  The index of the exported item.

#### Trait Implementations

##### `impl Clone for Export<'a>`

- <span id="export-clone"></span>`fn clone(&self) -> Export<'a>` — [`Export`](../index.md#export)

##### `impl Copy for Export<'a>`

##### `impl Debug for Export<'a>`

- <span id="export-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Export<'a>`

##### `impl FromReader for Export<'a>`

- <span id="export-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader), [`Result`](../../../binary_reader/index.md#result)

##### `impl PartialEq for Export<'a>`

- <span id="export-partialeq-eq"></span>`fn eq(&self, other: &Export<'a>) -> bool` — [`Export`](../index.md#export)

##### `impl StructuralPartialEq for Export<'a>`

## Enums

### `ExternalKind`

```rust
enum ExternalKind {
    Func,
    Table,
    Memory,
    Global,
    Tag,
    FuncExact,
}
```

External types as defined [here].


#### Variants

- **`Func`**

  The external kind is a function.

- **`Table`**

  The external kind if a table.

- **`Memory`**

  The external kind is a memory.

- **`Global`**

  The external kind is a global.

- **`Tag`**

  The external kind is a tag.

- **`FuncExact`**

  The external kind is a function with the exact type.

#### Trait Implementations

##### `impl Clone for ExternalKind`

- <span id="externalkind-clone"></span>`fn clone(&self) -> ExternalKind` — [`ExternalKind`](../index.md#externalkind)

##### `impl Copy for ExternalKind`

##### `impl Debug for ExternalKind`

- <span id="externalkind-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ExternalKind`

##### `impl FromReader for ExternalKind`

- <span id="externalkind-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader), [`Result`](../../../binary_reader/index.md#result)

##### `impl PartialEq for ExternalKind`

- <span id="externalkind-partialeq-eq"></span>`fn eq(&self, other: &ExternalKind) -> bool` — [`ExternalKind`](../index.md#externalkind)

##### `impl StructuralPartialEq for ExternalKind`

## Type Aliases

### `ExportSectionReader<'a>`

```rust
type ExportSectionReader<'a> = crate::SectionLimited<'a, Export<'a>>;
```

A reader for the export section of a WebAssembly module.

