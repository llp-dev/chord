*[wasmparser](../../../index.md) / [readers](../../index.md) / [core](../index.md) / [names](index.md)*

---

# Module `names`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Naming`](#naming) | struct | Represents a name for an index from the names section. |
| [`IndirectNaming`](#indirectnaming) | struct | Represents an indirect name in the names custom section. |
| [`Name`](#name) | enum | Represents a name read from the names custom section. |
| [`NameMap`](#namemap) | type | Represents a name map from the names custom section. |
| [`IndirectNameMap`](#indirectnamemap) | type | Represents a reader for indirect names from the names custom section. |
| [`NameSectionReader`](#namesectionreader) | type | A reader for the name custom section of a WebAssembly module. |

## Structs

### `Naming<'a>`

```rust
struct Naming<'a> {
    pub index: u32,
    pub name: &'a str,
}
```

Represents a name for an index from the names section.

#### Fields

- **`index`**: `u32`

  The index being named.

- **`name`**: `&'a str`

  The name for the index.

#### Trait Implementations

##### `impl Clone for Naming<'a>`

- <span id="naming-clone"></span>`fn clone(&self) -> Naming<'a>` — [`Naming`](../index.md#naming)

##### `impl Copy for Naming<'a>`

##### `impl Debug for Naming<'a>`

- <span id="naming-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl FromReader for Naming<'a>`

- <span id="naming-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader), [`Result`](../../../binary_reader/index.md#result)

### `IndirectNaming<'a>`

```rust
struct IndirectNaming<'a> {
    pub index: u32,
    pub names: NameMap<'a>,
}
```

Represents an indirect name in the names custom section.

#### Fields

- **`index`**: `u32`

  The indirect index of the name.

- **`names`**: `NameMap<'a>`

  The map of names within the `index` prior.

#### Trait Implementations

##### `impl Clone for IndirectNaming<'a>`

- <span id="indirectnaming-clone"></span>`fn clone(&self) -> IndirectNaming<'a>` — [`IndirectNaming`](../index.md#indirectnaming)

##### `impl Debug for IndirectNaming<'a>`

- <span id="indirectnaming-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl FromReader for IndirectNaming<'a>`

- <span id="indirectnaming-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader), [`Result`](../../../binary_reader/index.md#result)

## Enums

### `Name<'a>`

```rust
enum Name<'a> {
    Module {
        name: &'a str,
        name_range: core::ops::Range<usize>,
    },
    Function(NameMap<'a>),
    Local(IndirectNameMap<'a>),
    Label(IndirectNameMap<'a>),
    Type(NameMap<'a>),
    Table(NameMap<'a>),
    Memory(NameMap<'a>),
    Global(NameMap<'a>),
    Element(NameMap<'a>),
    Data(NameMap<'a>),
    Field(IndirectNameMap<'a>),
    Tag(NameMap<'a>),
    Unknown {
        ty: u8,
        data: &'a [u8],
        range: core::ops::Range<usize>,
    },
}
```

Represents a name read from the names custom section.

#### Variants

- **`Module`**

  The name is for the module.

- **`Function`**

  The name is for the functions.

- **`Local`**

  The name is for the function locals.

- **`Label`**

  The name is for the function labels.

- **`Type`**

  The name is for the types.

- **`Table`**

  The name is for the tables.

- **`Memory`**

  The name is for the memories.

- **`Global`**

  The name is for the globals.

- **`Element`**

  The name is for the element segments.

- **`Data`**

  The name is for the data segments.

- **`Field`**

  The name is for fields.

- **`Tag`**

  The name is for tags.

- **`Unknown`**

  An unknown [name subsection](https://webassembly.github.io/spec/core/appendix/custom.html#subsections).

#### Trait Implementations

##### `impl Clone for Name<'a>`

- <span id="name-clone"></span>`fn clone(&self) -> Name<'a>` — [`Name`](../index.md#name)

##### `impl Subsection for Name<'a>`

- <span id="name-subsection-from-reader"></span>`fn from_reader(id: u8, reader: BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader), [`Result`](../../../binary_reader/index.md#result)

## Type Aliases

### `NameMap<'a>`

```rust
type NameMap<'a> = crate::SectionLimited<'a, Naming<'a>>;
```

Represents a name map from the names custom section.

### `IndirectNameMap<'a>`

```rust
type IndirectNameMap<'a> = crate::SectionLimited<'a, IndirectNaming<'a>>;
```

Represents a reader for indirect names from the names custom section.

### `NameSectionReader<'a>`

```rust
type NameSectionReader<'a> = crate::Subsections<'a, Name<'a>>;
```

A reader for the name custom section of a WebAssembly module.

