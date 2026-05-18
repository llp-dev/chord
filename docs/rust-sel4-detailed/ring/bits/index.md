*[ring](../index.md) / [bits](index.md)*

---

# Module `bits`

Bit lengths.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`BitLength`](#bitlength) | struct | The length of something, in bits. |
| [`FromUsizeBytes`](#fromusizebytes) | trait |  |

## Structs

### `BitLength<T>`

```rust
struct BitLength<T>(T);
```

The length of something, in bits.

This can represent a bit length that isn't a whole number of bytes.

#### Implementations

- <span id="bitlength-as-bits"></span>`fn as_bits(self) -> T`

  The number of bits this bit length represents, as a `usize`.

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for BitLength<T>`

- <span id="bitlength-clone"></span>`fn clone(&self) -> BitLength<T>` — [`BitLength`](#bitlength)

##### `impl<T: marker::Copy> Copy for BitLength<T>`

##### `impl<T: fmt::Debug> Debug for BitLength<T>`

- <span id="bitlength-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<T: cmp::Eq> Eq for BitLength<T>`

##### `impl FromUsizeBytes for BitLength<usize>`

- <span id="bitlength-fromusizebytes-from-usize-bytes"></span>`fn from_usize_bytes(bytes: usize) -> Result<Self, error::Unspecified>` — [`Unspecified`](../error/index.md#unspecified)

##### `impl<T: cmp::PartialEq> PartialEq for BitLength<T>`

- <span id="bitlength-partialeq-eq"></span>`fn eq(&self, other: &BitLength<T>) -> bool` — [`BitLength`](#bitlength)

##### `impl<T: cmp::PartialOrd> PartialOrd for BitLength<T>`

- <span id="bitlength-partialord-partial-cmp"></span>`fn partial_cmp(&self, other: &BitLength<T>) -> option::Option<cmp::Ordering>` — [`BitLength`](#bitlength)

##### `impl<T> StructuralPartialEq for BitLength<T>`

## Traits

### `FromUsizeBytes`

```rust
trait FromUsizeBytes: Sized { ... }
```

#### Required Methods

- `fn from_usize_bytes(bytes: usize) -> Result<Self, error::Unspecified>`

  Constructs a `BitLength` from the given length in bytes.

#### Implementors

- [`BitLength`](#bitlength)

