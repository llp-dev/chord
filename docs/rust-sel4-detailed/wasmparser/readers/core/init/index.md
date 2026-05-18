*[wasmparser](../../../index.md) / [readers](../../index.md) / [core](../index.md) / [init](index.md)*

---

# Module `init`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`ConstExpr`](#constexpr) | struct | Represents an initialization expression. |

## Structs

### `ConstExpr<'a>`

```rust
struct ConstExpr<'a> {
    reader: crate::BinaryReader<'a>,
}
```

Represents an initialization expression.

#### Implementations

- <span id="constexpr-new"></span>`fn new(reader: BinaryReader<'a>) -> ConstExpr<'a>` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader), [`ConstExpr`](../index.md#constexpr)

  Constructs a new `ConstExpr` from the given data and offset.

- <span id="constexpr-get-binary-reader"></span>`fn get_binary_reader(&self) -> BinaryReader<'a>` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader)

  Gets a binary reader for the initialization expression.

- <span id="constexpr-get-operators-reader"></span>`fn get_operators_reader(&self) -> OperatorsReader<'a>` — [`OperatorsReader`](../index.md#operatorsreader)

  Gets an operators parser for the initialization expression.

#### Trait Implementations

##### `impl Clone for ConstExpr<'a>`

- <span id="constexpr-clone"></span>`fn clone(&self) -> ConstExpr<'a>` — [`ConstExpr`](../index.md#constexpr)

##### `impl Debug for ConstExpr<'_>`

- <span id="constexpr-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Eq for ConstExpr<'_>`

##### `impl FromReader for ConstExpr<'a>`

- <span id="constexpr-fromreader-from-reader"></span>`fn from_reader(reader: &mut BinaryReader<'a>) -> Result<Self>` — [`BinaryReader`](../../../binary_reader/index.md#binaryreader), [`Result`](../../../binary_reader/index.md#result)

##### `impl PartialEq for ConstExpr<'_>`

- <span id="constexpr-partialeq-eq"></span>`fn eq(&self, other: &Self) -> bool`

