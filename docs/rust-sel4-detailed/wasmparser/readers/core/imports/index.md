*[wasmparser](../../../index.md) / [readers](../../index.md) / [core](../index.md) / [imports](index.md)*

---

# Module `imports`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Import`](#import) | struct | Represents an import in a WebAssembly module. |
| [`TypeRef`](#typeref) | enum | Represents a reference to a type definition in a WebAssembly module. |
| [`ImportSectionReader`](#importsectionreader) | type | A reader for the import section of a WebAssembly module. |

## Structs

### `Import<'a>`

```rust
struct Import<'a> {
    pub module: &'a str,
    pub name: &'a str,
    pub ty: TypeRef,
}
```

Represents an import in a WebAssembly module.

#### Fields

- **`module`**: `&'a str`

  The module being imported from.

- **`name`**: `&'a str`

  The name of the imported item.

- **`ty`**: `TypeRef`

  The type of the imported item.

#### Trait Implementations

##### `impl Clone for Import<'a>`

- <span id="import-clone"></span>`fn clone(&self) -> Import<'a>` — [`Import`](../index.md#import)

##### `impl Copy for Import<'a>`

##### `impl Debug for Import<'a>`

- <span id="import-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for Import<'a>`

##### `impl FromReader for Import<'a>`

- <span id="import-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader), [`Result`](../../../binary_reader/index.md#result)

##### `impl PartialEq for Import<'a>`

- <span id="import-partialeq-eq"></span>`fn eq(&self, other: &Import<'a>) -> bool` — [`Import`](../index.md#import)

##### `impl StructuralPartialEq for Import<'a>`

## Enums

### `TypeRef`

```rust
enum TypeRef {
    Func(u32),
    Table(crate::TableType),
    Memory(crate::MemoryType),
    Global(crate::GlobalType),
    Tag(crate::TagType),
    FuncExact(u32),
}
```

Represents a reference to a type definition in a WebAssembly module.

#### Variants

- **`Func`**

  The type is a function.

- **`Table`**

  The type is a table.

- **`Memory`**

  The type is a memory.

- **`Global`**

  The type is a global.

- **`Tag`**

  The type is a tag.
  
  This variant is only used for the exception handling proposal.
  
  The value is an index in the types index space.

- **`FuncExact`**

  The type is a function.

#### Trait Implementations

##### `impl Clone for TypeRef`

- <span id="typeref-clone"></span>`fn clone(&self) -> TypeRef` — [`TypeRef`](../index.md#typeref)

##### `impl Copy for TypeRef`

##### `impl Debug for TypeRef`

- <span id="typeref-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for TypeRef`

##### `impl FromReader for TypeRef`

- <span id="typeref-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader), [`Result`](../../../binary_reader/index.md#result)

##### `impl PartialEq for TypeRef`

- <span id="typeref-partialeq-eq"></span>`fn eq(&self, other: &TypeRef) -> bool` — [`TypeRef`](../index.md#typeref)

##### `impl StructuralPartialEq for TypeRef`

## Type Aliases

### `ImportSectionReader<'a>`

```rust
type ImportSectionReader<'a> = crate::SectionLimited<'a, Import<'a>>;
```

A reader for the import section of a WebAssembly module.

