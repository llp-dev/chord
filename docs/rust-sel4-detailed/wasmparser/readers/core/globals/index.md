*[wasmparser](../../../index.md) / [readers](../../index.md) / [core](../index.md) / [globals](index.md)*

---

# Module `globals`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Global`](#global) | struct | Represents a core WebAssembly global. |
| [`GlobalSectionReader`](#globalsectionreader) | type | A reader for the global section of a WebAssembly module. |

## Structs

### `Global<'a>`

```rust
struct Global<'a> {
    pub ty: crate::GlobalType,
    pub init_expr: crate::ConstExpr<'a>,
}
```

Represents a core WebAssembly global.

#### Fields

- **`ty`**: `crate::GlobalType`

  The global's type.

- **`init_expr`**: `crate::ConstExpr<'a>`

  The global's initialization expression.

#### Trait Implementations

##### `impl Clone for Global<'a>`

- <span id="global-clone"></span>`fn clone(&self) -> Global<'a>` — [`Global`](../index.md#global)

##### `impl Debug for Global<'a>`

- <span id="global-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl FromReader for Global<'a>`

- <span id="global-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader), [`Result`](../../../binary_reader/index.md#result)

## Type Aliases

### `GlobalSectionReader<'a>`

```rust
type GlobalSectionReader<'a> = crate::SectionLimited<'a, Global<'a>>;
```

A reader for the global section of a WebAssembly module.

