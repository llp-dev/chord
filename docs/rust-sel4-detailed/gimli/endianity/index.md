*[gimli](../index.md) / [endianity](index.md)*

---

# Module `endianity`

Types for compile-time and run-time endianity.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`LittleEndian`](#littleendian) | struct | Little endian byte order. |
| [`BigEndian`](#bigendian) | struct | Big endian byte order. |
| [`RunTimeEndian`](#runtimeendian) | enum | Byte order that is selectable at runtime. |
| [`Endianity`](#endianity) | trait | A trait describing the endianity of some buffer. |
| [`NativeEndian`](#nativeendian) | type | The native endianity for the target platform. |

## Structs

### `LittleEndian`

```rust
struct LittleEndian;
```

Little endian byte order.

#### Trait Implementations

##### `impl Clone for LittleEndian`

- <span id="littleendian-clone"></span>`fn clone(&self) -> LittleEndian` — [`LittleEndian`](../index.md#littleendian)

##### `impl Copy for LittleEndian`

##### `impl Debug for LittleEndian`

- <span id="littleendian-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for LittleEndian`

- <span id="littleendian-default"></span>`fn default() -> LittleEndian` — [`LittleEndian`](../index.md#littleendian)

##### `impl Endianity for LittleEndian`

- <span id="littleendian-endianity-is-big-endian"></span>`fn is_big_endian(self) -> bool`

##### `impl Eq for LittleEndian`

##### `impl Hash for LittleEndian`

- <span id="littleendian-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for LittleEndian`

- <span id="littleendian-partialeq-eq"></span>`fn eq(&self, other: &LittleEndian) -> bool` — [`LittleEndian`](../index.md#littleendian)

##### `impl StructuralPartialEq for LittleEndian`

### `BigEndian`

```rust
struct BigEndian;
```

Big endian byte order.

#### Trait Implementations

##### `impl Clone for BigEndian`

- <span id="bigendian-clone"></span>`fn clone(&self) -> BigEndian` — [`BigEndian`](../index.md#bigendian)

##### `impl Copy for BigEndian`

##### `impl Debug for BigEndian`

- <span id="bigendian-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for BigEndian`

- <span id="bigendian-default"></span>`fn default() -> BigEndian` — [`BigEndian`](../index.md#bigendian)

##### `impl Endianity for BigEndian`

- <span id="bigendian-endianity-is-big-endian"></span>`fn is_big_endian(self) -> bool`

##### `impl Eq for BigEndian`

##### `impl Hash for BigEndian`

- <span id="bigendian-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for BigEndian`

- <span id="bigendian-partialeq-eq"></span>`fn eq(&self, other: &BigEndian) -> bool` — [`BigEndian`](../index.md#bigendian)

##### `impl StructuralPartialEq for BigEndian`

## Enums

### `RunTimeEndian`

```rust
enum RunTimeEndian {
    Little,
    Big,
}
```

Byte order that is selectable at runtime.

#### Variants

- **`Little`**

  Little endian byte order.

- **`Big`**

  Big endian byte order.

#### Trait Implementations

##### `impl Clone for RunTimeEndian`

- <span id="runtimeendian-clone"></span>`fn clone(&self) -> RunTimeEndian` — [`RunTimeEndian`](../index.md#runtimeendian)

##### `impl Copy for RunTimeEndian`

##### `impl Debug for RunTimeEndian`

- <span id="runtimeendian-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl Default for RunTimeEndian`

- <span id="runtimeendian-default"></span>`fn default() -> RunTimeEndian` — [`RunTimeEndian`](../index.md#runtimeendian)

##### `impl Endianity for RunTimeEndian`

- <span id="runtimeendian-endianity-is-big-endian"></span>`fn is_big_endian(self) -> bool`

##### `impl Eq for RunTimeEndian`

##### `impl Hash for RunTimeEndian`

- <span id="runtimeendian-hash"></span>`fn hash<__H: hash::Hasher>(&self, state: &mut __H)`

##### `impl PartialEq for RunTimeEndian`

- <span id="runtimeendian-partialeq-eq"></span>`fn eq(&self, other: &RunTimeEndian) -> bool` — [`RunTimeEndian`](../index.md#runtimeendian)

##### `impl StructuralPartialEq for RunTimeEndian`

## Traits

### `Endianity`

```rust
trait Endianity: Debug + Default + Clone + Copy + PartialEq + Eq { ... }
```

A trait describing the endianity of some buffer.

#### Required Methods

- `fn is_big_endian(self) -> bool`

  Return true for big endian byte order.

#### Provided Methods

- `fn is_little_endian(self) -> bool`

  Return true for little endian byte order.

- `fn read_u16(self, buf: &[u8]) -> u16`

  Reads an unsigned 16 bit integer from `buf`.

- `fn read_u32(self, buf: &[u8]) -> u32`

  Reads an unsigned 32 bit integer from `buf`.

- `fn read_u64(self, buf: &[u8]) -> u64`

  Reads an unsigned 64 bit integer from `buf`.

- `fn read_u128(self, buf: &[u8]) -> u128`

  Reads an unsigned 128 bit integer from `buf`.

- `fn read_uint(&mut self, buf: &[u8]) -> u64`

  Read an unsigned n-bytes integer u64.

- `fn read_i16(self, buf: &[u8]) -> i16`

  Reads a signed 16 bit integer from `buf`.

- `fn read_i32(self, buf: &[u8]) -> i32`

  Reads a signed 32 bit integer from `buf`.

- `fn read_i64(self, buf: &[u8]) -> i64`

  Reads a signed 64 bit integer from `buf`.

- `fn read_f32(self, buf: &[u8]) -> f32`

  Reads a 32 bit floating point number from `buf`.

- `fn read_f64(self, buf: &[u8]) -> f64`

  Reads a 32 bit floating point number from `buf`.

- `fn write_u16(self, buf: &mut [u8], n: u16)`

  Writes an unsigned 16 bit integer `n` to `buf`.

- `fn write_u32(self, buf: &mut [u8], n: u32)`

  Writes an unsigned 32 bit integer `n` to `buf`.

- `fn write_u64(self, buf: &mut [u8], n: u64)`

  Writes an unsigned 64 bit integer `n` to `buf`.

- `fn write_u128(self, buf: &mut [u8], n: u128)`

  Writes an unsigned 128 bit integer `n` to `buf`.

#### Implementors

- [`BigEndian`](../index.md#bigendian)
- [`LittleEndian`](../index.md#littleendian)
- [`RunTimeEndian`](../index.md#runtimeendian)

## Type Aliases

### `NativeEndian`

```rust
type NativeEndian = LittleEndian;
```

The native endianity for the target platform.

