*[hash32](../index.md) / [murmur3](index.md)*

---

# Module `murmur3`

## Contents

- [Structs](#structs)
  - [`Hasher`](#hasher)
  - [`State`](#state)
  - [`Buffer`](#buffer)
- [Enums](#enums)
  - [`Index`](#index)
- [Functions](#functions)
  - [`pre_mix`](#pre-mix)
- [Constants](#constants)
  - [`C1`](#c1)
  - [`C2`](#c2)
  - [`R1`](#r1)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Hasher`](#hasher) | struct | 32-bit MurmurHash3 hasher |
| [`State`](#state) | struct |  |
| [`Buffer`](#buffer) | struct |  |
| [`Index`](#index) | enum |  |
| [`pre_mix`](#pre-mix) | fn |  |
| [`C1`](#c1) | const |  |
| [`C2`](#c2) | const |  |
| [`R1`](#r1) | const |  |

## Structs

### `Hasher`

```rust
struct Hasher {
    buf: Buffer,
    index: Index,
    processed: u32,
    state: State,
}
```

32-bit MurmurHash3 hasher

#### Implementations

- <span id="hasher-push"></span>`fn push(&mut self, buf: &[u8])`

#### Trait Implementations

##### `impl Default for Hasher`

- <span id="hasher-default"></span>`fn default() -> Self`

##### `impl Hasher for Hasher`

- <span id="hasher-hasher-finish32"></span>`fn finish32(&self) -> u32`

### `State`

```rust
struct State(u32);
```

#### Implementations

- <span id="state-process-block"></span>`fn process_block(&mut self, block: &MaybeUninit<[u8; 4]>)`

### `Buffer`

```rust
struct Buffer {
    bytes: core::mem::MaybeUninit<[u8; 4]>,
}
```

#### Trait Implementations

##### `impl Clone for Buffer`

- <span id="buffer-clone"></span>`fn clone(&self) -> Buffer` — [`Buffer`](#buffer)

##### `impl Copy for Buffer`

## Enums

### `Index`

```rust
enum Index {
    _0,
    _1,
    _2,
    _3,
}
```

#### Implementations

- <span id="index-usize"></span>`fn usize(&self) -> usize`

#### Trait Implementations

##### `impl Clone for Index`

- <span id="index-clone"></span>`fn clone(&self) -> Index` — [`Index`](#index)

##### `impl Copy for Index`

##### `impl PartialEq for Index`

- <span id="index-partialeq-eq"></span>`fn eq(&self, other: &Index) -> bool` — [`Index`](#index)

##### `impl StructuralPartialEq for Index`

## Functions

### `pre_mix`

```rust
fn pre_mix(block: u32) -> u32
```

## Constants

### `C1`
```rust
const C1: u32 = 3_432_918_353u32;
```

### `C2`
```rust
const C2: u32 = 461_845_907u32;
```

### `R1`
```rust
const R1: u32 = 15u32;
```

