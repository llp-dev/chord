*[ring](../../index.md) / [cpu](../index.md) / [intel](index.md)*

---

# Module `intel`

## Contents

- [Modules](#modules)
  - [`abi_assumptions`](#abi-assumptions)
- [Structs](#structs)
  - [`Feature`](#feature)
- [Functions](#functions)
  - [`init_global_shared_with_assembly`](#init-global-shared-with-assembly)
- [Constants](#constants)
  - [`ADX`](#adx)
  - [`BMI1`](#bmi1)
  - [`BMI2`](#bmi2)
  - [`FXSR`](#fxsr)
  - [`PCLMULQDQ`](#pclmulqdq)
  - [`SSSE3`](#ssse3)
  - [`SSE41`](#sse41)
  - [`MOVBE`](#movbe)
  - [`AES`](#aes)
  - [`AVX`](#avx)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`abi_assumptions`](#abi-assumptions) | mod |  |
| [`Feature`](#feature) | struct |  |
| [`init_global_shared_with_assembly`](#init-global-shared-with-assembly) | fn |  |
| [`ADX`](#adx) | const |  |
| [`BMI1`](#bmi1) | const |  |
| [`BMI2`](#bmi2) | const |  |
| [`FXSR`](#fxsr) | const |  |
| [`PCLMULQDQ`](#pclmulqdq) | const |  |
| [`SSSE3`](#ssse3) | const |  |
| [`SSE41`](#sse41) | const |  |
| [`MOVBE`](#movbe) | const |  |
| [`AES`](#aes) | const |  |
| [`AVX`](#avx) | const |  |

## Modules

- [`abi_assumptions`](abi_assumptions/index.md)

## Structs

### `Feature`

```rust
struct Feature {
    word: usize,
    mask: u32,
}
```

#### Implementations

- <span id="feature-available"></span>`fn available(&self, _: super::Features) -> bool` — [`Features`](../index.md#features)

## Functions

### `init_global_shared_with_assembly`

```rust
unsafe fn init_global_shared_with_assembly()
```

## Constants

### `ADX`
```rust
const ADX: Feature;
```

### `BMI1`
```rust
const BMI1: Feature;
```

### `BMI2`
```rust
const BMI2: Feature;
```

### `FXSR`
```rust
const FXSR: Feature;
```

### `PCLMULQDQ`
```rust
const PCLMULQDQ: Feature;
```

### `SSSE3`
```rust
const SSSE3: Feature;
```

### `SSE41`
```rust
const SSE41: Feature;
```

### `MOVBE`
```rust
const MOVBE: Feature;
```

### `AES`
```rust
const AES: Feature;
```

### `AVX`
```rust
const AVX: Feature;
```

