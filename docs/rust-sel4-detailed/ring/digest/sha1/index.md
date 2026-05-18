*[ring](../../index.md) / [digest](../index.md) / [sha1](index.md)*

---

# Module `sha1`

## Contents

- [Functions](#functions)
  - [`parity`](#parity)
  - [`block_data_order`](#block-data-order)
  - [`block_data_order_`](#block-data-order)
  - [`step3`](#step3)
  - [`rotl`](#rotl)
- [Type Aliases](#type-aliases)
  - [`W32`](#w32)
  - [`State`](#state)
- [Constants](#constants)
  - [`BLOCK_LEN`](#block-len)
  - [`CHAINING_LEN`](#chaining-len)
  - [`OUTPUT_LEN`](#output-len)
  - [`CHAINING_WORDS`](#chaining-words)
  - [`ROUNDS`](#rounds)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`parity`](#parity) | fn |  |
| [`block_data_order`](#block-data-order) | fn |  |
| [`block_data_order_`](#block-data-order) | fn |  |
| [`step3`](#step3) | fn |  |
| [`rotl`](#rotl) | fn |  |
| [`W32`](#w32) | type |  |
| [`State`](#state) | type |  |
| [`BLOCK_LEN`](#block-len) | const |  |
| [`CHAINING_LEN`](#chaining-len) | const |  |
| [`OUTPUT_LEN`](#output-len) | const |  |
| [`CHAINING_WORDS`](#chaining-words) | const |  |
| [`ROUNDS`](#rounds) | const |  |

## Functions

### `parity`

```rust
fn parity(x: core::num::Wrapping<u32>, y: core::num::Wrapping<u32>, z: core::num::Wrapping<u32>) -> core::num::Wrapping<u32>
```

### `block_data_order`

```rust
fn block_data_order(state: &mut super::State, data: *const u8, num: usize)
```

### `block_data_order_`

```rust
fn block_data_order_(H: [core::num::Wrapping<u32>; 5], M: &[[<core::num::Wrapping<u32> as Word>::InputBytes; 16]]) -> [core::num::Wrapping<u32>; 5]
```

### `step3`

```rust
fn step3(a: core::num::Wrapping<u32>, b: core::num::Wrapping<u32>, c: core::num::Wrapping<u32>, d: core::num::Wrapping<u32>, e: core::num::Wrapping<u32>, W: &[core::num::Wrapping<u32>; 80], t: usize, k: core::num::Wrapping<u32>, f: impl Fn(core::num::Wrapping<u32>, core::num::Wrapping<u32>, core::num::Wrapping<u32>) -> core::num::Wrapping<u32>) -> (core::num::Wrapping<u32>, core::num::Wrapping<u32>, core::num::Wrapping<u32>, core::num::Wrapping<u32>, core::num::Wrapping<u32>)
```

### `rotl`

```rust
fn rotl(x: core::num::Wrapping<u32>, n: u32) -> core::num::Wrapping<u32>
```

## Type Aliases

### `W32`

```rust
type W32 = core::num::Wrapping<u32>;
```

### `State`

```rust
type State = [core::num::Wrapping<u32>; 5];
```

## Constants

### `BLOCK_LEN`
```rust
const BLOCK_LEN: usize = 64usize;
```

### `CHAINING_LEN`
```rust
const CHAINING_LEN: usize = 20usize;
```

### `OUTPUT_LEN`
```rust
const OUTPUT_LEN: usize = 20usize;
```

### `CHAINING_WORDS`
```rust
const CHAINING_WORDS: usize = 5usize;
```

### `ROUNDS`
```rust
const ROUNDS: usize = 80usize;
```

