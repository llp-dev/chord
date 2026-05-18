*[ring](../index.md) / [endian](index.md)*

---

# Module `endian`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`BigEndian`](#bigendian) | struct |  |
| [`Encoding`](#encoding) | trait | An `Encoding` of a type `T` can be converted to/from its byte representation without any byte swapping or other computation. |
| [`define_endian!`](#define-endian) | macro |  |
| [`impl_endian!`](#impl-endian) | macro |  |

## Structs

### `BigEndian<T>`

```rust
struct BigEndian<T>(T);
```

#### Trait Implementations

##### `impl<T: clone::Clone> Clone for BigEndian<T>`

- <span id="bigendian-clone"></span>`fn clone(&self) -> BigEndian<T>` — [`BigEndian`](#bigendian)

##### `impl<T: marker::Copy> Copy for BigEndian<T>`

##### `impl Encoding for BigEndian<u32>`

- <span id="bigendian-encoding-const-zero"></span>`const ZERO: Self`

## Traits

### `Encoding<T>`

```rust
trait Encoding<T>: From<T> + Into<T>
where
    Self: Copy { ... }
```

An `Encoding` of a type `T` can be converted to/from its byte
representation without any byte swapping or other computation.

The `Self: Copy` constraint addresses `clippy::declare_interior_mutable_const`.

#### Associated Constants

- `const ZERO: Self`

#### Implementors

- [`BigEndian`](#bigendian)

## Macros

### `define_endian!`

### `impl_endian!`

