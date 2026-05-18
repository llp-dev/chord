*[ring](../../index.md) / [digest](../index.md) / [sha2](index.md)*

---

# Module `sha2`

## Contents

- [Traits](#traits)
  - [`Word`](#word)
  - [`Sha2`](#sha2)
- [Functions](#functions)
  - [`sha256_block_data_order`](#sha256-block-data-order)
  - [`sha512_block_data_order`](#sha512-block-data-order)
  - [`block_data_order`](#block-data-order)
  - [`ch`](#ch)
  - [`maj`](#maj)
  - [`SIGMA_0`](#sigma-0)
  - [`SIGMA_1`](#sigma-1)
  - [`sigma_0`](#sigma-0)
  - [`sigma_1`](#sigma-1)
- [Constants](#constants)
  - [`MAX_ROUNDS`](#max-rounds)
  - [`CHAINING_WORDS`](#chaining-words)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Word`](#word) | trait |  |
| [`Sha2`](#sha2) | trait | A SHA-2 input word. |
| [`sha256_block_data_order`](#sha256-block-data-order) | fn |  |
| [`sha512_block_data_order`](#sha512-block-data-order) | fn |  |
| [`block_data_order`](#block-data-order) | fn |  |
| [`ch`](#ch) | fn |  |
| [`maj`](#maj) | fn |  |
| [`SIGMA_0`](#sigma-0) | fn |  |
| [`SIGMA_1`](#sigma-1) | fn |  |
| [`sigma_0`](#sigma-0) | fn |  |
| [`sigma_1`](#sigma-1) | fn |  |
| [`MAX_ROUNDS`](#max-rounds) | const |  |
| [`CHAINING_WORDS`](#chaining-words) | const |  |

## Traits

### `Word`

```rust
trait Word: 'static + Sized + Copy + Add<Output = Self> + AddAssign + BitAnd<Output = Self> + BitOr<Output = Self> + Not<Output = Self> { ... }
```

#### Associated Types

- `type InputBytes: 1`

#### Associated Constants

- `const ZERO: Self`

#### Required Methods

- `fn from_be_bytes(input: <Self as >::InputBytes) -> Self`

- `fn rotr(self, count: u32) -> Self`

#### Implementors

- `core::num::Wrapping<u32>`
- `core::num::Wrapping<u64>`

### `Sha2`

```rust
trait Sha2: Word + BitXor<Output = Self> + Shr<usize, Output = Self> { ... }
```

A SHA-2 input word.

#### Associated Constants

- `const BIG_SIGMA_0: (u32, u32, u32)`

- `const BIG_SIGMA_1: (u32, u32, u32)`

- `const SMALL_SIGMA_0: (u32, u32, usize)`

- `const SMALL_SIGMA_1: (u32, u32, usize)`

- `const K: &'static [Self]`

#### Implementors

- `core::num::Wrapping<u32>`
- `core::num::Wrapping<u64>`

## Functions

### `sha256_block_data_order`

```rust
unsafe fn sha256_block_data_order(state: &mut super::State, data: *const u8, num: usize)
```

### `sha512_block_data_order`

```rust
unsafe fn sha512_block_data_order(state: &mut super::State, data: *const u8, num: usize)
```

### `block_data_order`

```rust
fn block_data_order<S: Sha2>(H: [S; 8], M: *const u8, num: usize) -> [S; 8]
```

### `ch`

```rust
fn ch<W: Word>(x: W, y: W, z: W) -> W
```

### `maj`

```rust
fn maj<W: Word>(x: W, y: W, z: W) -> W
```

### `SIGMA_0`

```rust
fn SIGMA_0<S: Sha2>(x: S) -> S
```

### `SIGMA_1`

```rust
fn SIGMA_1<S: Sha2>(x: S) -> S
```

### `sigma_0`

```rust
fn sigma_0<S: Sha2>(x: S) -> S
```

### `sigma_1`

```rust
fn sigma_1<S: Sha2>(x: S) -> S
```

## Constants

### `MAX_ROUNDS`
```rust
const MAX_ROUNDS: usize = 80usize;
```

### `CHAINING_WORDS`
```rust
const CHAINING_WORDS: usize = 8usize;
```

