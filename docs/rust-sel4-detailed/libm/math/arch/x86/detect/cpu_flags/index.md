*[libm](../../../../../index.md) / [math](../../../../index.md) / [arch](../../../index.md) / [x86](../../index.md) / [detect](../index.md) / [cpu_flags](index.md)*

---

# Module `cpu_flags`

CPU features that get cached (doesn't correlate to anything on the CPU).

## Contents

- [Constants](#constants)
  - [`SSE3`](#sse3)
  - [`F16C`](#f16c)
  - [`SSE`](#sse)
  - [`SSE2`](#sse2)
  - [`ERMSB`](#ermsb)
  - [`MOVRS`](#movrs)
  - [`FMA`](#fma)
  - [`FMA4`](#fma4)
  - [`AVX512FP16`](#avx512fp16)
  - [`AVX512BF16`](#avx512bf16)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SSE3`](#sse3) | const |  |
| [`F16C`](#f16c) | const |  |
| [`SSE`](#sse) | const |  |
| [`SSE2`](#sse2) | const |  |
| [`ERMSB`](#ermsb) | const |  |
| [`MOVRS`](#movrs) | const |  |
| [`FMA`](#fma) | const |  |
| [`FMA4`](#fma4) | const |  |
| [`AVX512FP16`](#avx512fp16) | const |  |
| [`AVX512BF16`](#avx512bf16) | const |  |

## Constants

### `SSE3`
```rust
const SSE3: u32 = 1u32;
```

### `F16C`
```rust
const F16C: u32 = 2u32;
```

### `SSE`
```rust
const SSE: u32 = 4u32;
```

### `SSE2`
```rust
const SSE2: u32 = 8u32;
```

### `ERMSB`
```rust
const ERMSB: u32 = 16u32;
```

### `MOVRS`
```rust
const MOVRS: u32 = 32u32;
```

### `FMA`
```rust
const FMA: u32 = 64u32;
```

### `FMA4`
```rust
const FMA4: u32 = 128u32;
```

### `AVX512FP16`
```rust
const AVX512FP16: u32 = 256u32;
```

### `AVX512BF16`
```rust
const AVX512BF16: u32 = 512u32;
```

