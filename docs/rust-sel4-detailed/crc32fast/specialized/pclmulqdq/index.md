*[crc32fast](../../index.md) / [specialized](../index.md) / [pclmulqdq](index.md)*

---

# Module `pclmulqdq`

Specialized checksum code for the x86 CPU architecture, based on the efficient algorithm described
in the following whitepaper:

Gopal, V., Ozturk, E., Guilford, J., Wolrich, G., Feghali, W., Dixon, M., & Karakoyunlu, D. (2009).
_Fast CRC computation for generic polynomials using PCLMULQDQ instruction_. Intel.
(Mirror link: <https://fossies.org/linux/zlib-ng/doc/crc-pclmulqdq.pdf>, accessed 2024-05-20)

Throughout the code, this work is referred to as "the paper".

## Contents

- [Structs](#structs)
  - [`State`](#state)
- [Functions](#functions)
  - [`calculate`](#calculate)
  - [`reduce128`](#reduce128)
  - [`get`](#get)
- [Constants](#constants)
  - [`K1`](#k1)
  - [`K2`](#k2)
  - [`K3`](#k3)
  - [`K4`](#k4)
  - [`K5`](#k5)
  - [`P_X`](#p-x)
  - [`U_PRIME`](#u-prime)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`State`](#state) | struct |  |
| [`calculate`](#calculate) | fn |  |
| [`reduce128`](#reduce128) | fn |  |
| [`get`](#get) | fn |  |
| [`K1`](#k1) | const |  |
| [`K2`](#k2) | const |  |
| [`K3`](#k3) | const |  |
| [`K4`](#k4) | const |  |
| [`K5`](#k5) | const |  |
| [`P_X`](#p-x) | const |  |
| [`U_PRIME`](#u-prime) | const |  |

## Structs

### `State`

```rust
struct State {
    state: u32,
}
```

#### Implementations

- <span id="state-new"></span>`fn new(state: u32) -> Option<Self>`

- <span id="state-update"></span>`fn update(&mut self, buf: &[u8])`

- <span id="state-finalize"></span>`fn finalize(self) -> u32`

- <span id="state-reset"></span>`fn reset(&mut self)`

- <span id="state-combine"></span>`fn combine(&mut self, other: u32, amount: u64)`

#### Trait Implementations

##### `impl Clone for State`

- <span id="state-clone"></span>`fn clone(&self) -> State` — [`State`](#state)

## Functions

### `calculate`

```rust
unsafe fn calculate(crc: u32, data: &[u8]) -> u32
```

### `reduce128`

```rust
unsafe fn reduce128(a: arch::__m128i, b: arch::__m128i, keys: arch::__m128i) -> arch::__m128i
```

### `get`

```rust
unsafe fn get(a: &mut &[u8]) -> arch::__m128i
```

## Constants

### `K1`
```rust
const K1: i64 = 5_708_721_108i64;
```

### `K2`
```rust
const K2: i64 = 7_631_803_798i64;
```

### `K3`
```rust
const K3: i64 = 6_259_578_832i64;
```

### `K4`
```rust
const K4: i64 = 3_433_693_342i64;
```

### `K5`
```rust
const K5: i64 = 5_969_371_428i64;
```

### `P_X`
```rust
const P_X: i64 = 7_976_584_769i64;
```

### `U_PRIME`
```rust
const U_PRIME: i64 = 8_439_010_881i64;
```

