*[simdutf8](../../index.md) / [implementation](../index.md) / [helpers](index.md)*

---

# Module `helpers`

## Contents

- [Structs](#structs)
  - [`Utf8CheckAlgorithm`](#utf8checkalgorithm)
  - [`TempSimdChunkA16`](#tempsimdchunka16)
  - [`TempSimdChunkA32`](#tempsimdchunka32)
  - [`SimdU8Value`](#simdu8value)
- [Functions](#functions)
  - [`validate_utf8_at_offset`](#validate-utf8-at-offset)
  - [`get_compat_error`](#get-compat-error)
  - [`memcpy_unaligned_nonoverlapping_inline_opt_lt_64`](#memcpy-unaligned-nonoverlapping-inline-opt-lt-64)
- [Type Aliases](#type-aliases)
  - [`Utf8ErrorCompat`](#utf8errorcompat)
- [Constants](#constants)
  - [`SIMD_CHUNK_SIZE`](#simd-chunk-size)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Utf8CheckAlgorithm`](#utf8checkalgorithm) | struct |  |
| [`TempSimdChunkA16`](#tempsimdchunka16) | struct |  |
| [`TempSimdChunkA32`](#tempsimdchunka32) | struct |  |
| [`SimdU8Value`](#simdu8value) | struct |  |
| [`validate_utf8_at_offset`](#validate-utf8-at-offset) | fn |  |
| [`get_compat_error`](#get-compat-error) | fn |  |
| [`memcpy_unaligned_nonoverlapping_inline_opt_lt_64`](#memcpy-unaligned-nonoverlapping-inline-opt-lt-64) | fn |  |
| [`Utf8ErrorCompat`](#utf8errorcompat) | type |  |
| [`SIMD_CHUNK_SIZE`](#simd-chunk-size) | const |  |

## Structs

### `Utf8CheckAlgorithm<T>`

```rust
struct Utf8CheckAlgorithm<T> {
    prev: T,
    incomplete: T,
    error: T,
}
```

#### Implementations

- <span id="crateimplementationhelpersutf8checkalgorithm-must-be-2-3-continuation"></span>`unsafe fn must_be_2_3_continuation(prev2: crate::implementation::helpers::SimdU8Value<core::arch::x86_64::__m256i>, prev3: crate::implementation::helpers::SimdU8Value<core::arch::x86_64::__m256i>) -> crate::implementation::helpers::SimdU8Value<core::arch::x86_64::__m256i>` — [`SimdU8Value`](#simdu8value)

### `TempSimdChunkA16`

```rust
struct TempSimdChunkA16([u8; 64]);
```

#### Implementations

- <span id="tempsimdchunka16-new"></span>`const fn new() -> Self`

### `TempSimdChunkA32`

```rust
struct TempSimdChunkA32([u8; 64]);
```

#### Implementations

- <span id="tempsimdchunka32-new"></span>`const fn new() -> Self`

### `SimdU8Value<T>`

```rust
struct SimdU8Value<T>(T)
where
    T: Copy;
```

#### Implementations

- <span id="crateimplementationhelperssimdu8value-from-32-cut-off-leading"></span>`unsafe fn from_32_cut_off_leading(v0: u8, v1: u8, v2: u8, v3: u8, v4: u8, v5: u8, v6: u8, v7: u8, v8: u8, v9: u8, v10: u8, v11: u8, v12: u8, v13: u8, v14: u8, v15: u8, v16: u8, v17: u8, v18: u8, v19: u8, v20: u8, v21: u8, v22: u8, v23: u8, v24: u8, v25: u8, v26: u8, v27: u8, v28: u8, v29: u8, v30: u8, v31: u8) -> Self`

- <span id="crateimplementationhelperssimdu8value-repeat-16"></span>`unsafe fn repeat_16(v0: u8, v1: u8, v2: u8, v3: u8, v4: u8, v5: u8, v6: u8, v7: u8, v8: u8, v9: u8, v10: u8, v11: u8, v12: u8, v13: u8, v14: u8, v15: u8) -> Self`

- <span id="crateimplementationhelperssimdu8value-load-from"></span>`unsafe fn load_from(ptr: *const u8) -> Self`

- <span id="crateimplementationhelperssimdu8value-lookup-16"></span>`unsafe fn lookup_16(self, v0: u8, v1: u8, v2: u8, v3: u8, v4: u8, v5: u8, v6: u8, v7: u8, v8: u8, v9: u8, v10: u8, v11: u8, v12: u8, v13: u8, v14: u8, v15: u8) -> Self`

- <span id="crateimplementationhelperssimdu8value-splat"></span>`unsafe fn splat(val: u8) -> Self`

- <span id="crateimplementationhelperssimdu8value-splat0"></span>`unsafe fn splat0() -> Self`

- <span id="crateimplementationhelperssimdu8value-or"></span>`unsafe fn or(self, b: Self) -> Self`

- <span id="crateimplementationhelperssimdu8value-and"></span>`unsafe fn and(self, b: Self) -> Self`

- <span id="crateimplementationhelperssimdu8value-xor"></span>`unsafe fn xor(self, b: Self) -> Self`

- <span id="crateimplementationhelperssimdu8value-saturating-sub"></span>`unsafe fn saturating_sub(self, b: Self) -> Self`

- <span id="crateimplementationhelperssimdu8value-shr4"></span>`unsafe fn shr4(self) -> Self`

- <span id="crateimplementationhelperssimdu8value-prev1"></span>`unsafe fn prev1(self, prev: Self) -> Self`

- <span id="crateimplementationhelperssimdu8value-prev2"></span>`unsafe fn prev2(self, prev: Self) -> Self`

- <span id="crateimplementationhelperssimdu8value-prev3"></span>`unsafe fn prev3(self, prev: Self) -> Self`

- <span id="crateimplementationhelperssimdu8value-signed-gt"></span>`unsafe fn signed_gt(self, other: Self) -> Self`

- <span id="crateimplementationhelperssimdu8value-any-bit-set"></span>`unsafe fn any_bit_set(self) -> bool`

- <span id="crateimplementationhelperssimdu8value-is-ascii"></span>`unsafe fn is_ascii(self) -> bool`

#### Trait Implementations

##### `impl<T> Clone for SimdU8Value<T>`

- <span id="simdu8value-clone"></span>`fn clone(&self) -> SimdU8Value<T>` — [`SimdU8Value`](#simdu8value)

##### `impl<T> Copy for SimdU8Value<T>`

## Functions

### `validate_utf8_at_offset`

```rust
fn validate_utf8_at_offset(input: &[u8], offset: usize) -> Result<(), crate::compat::Utf8Error>
```

### `get_compat_error`

```rust
fn get_compat_error(input: &[u8], failing_block_pos: usize) -> crate::compat::Utf8Error
```

### `memcpy_unaligned_nonoverlapping_inline_opt_lt_64`

```rust
unsafe fn memcpy_unaligned_nonoverlapping_inline_opt_lt_64(src: *const u8, dest: *mut u8, len: usize)
```

## Type Aliases

### `Utf8ErrorCompat`

```rust
type Utf8ErrorCompat = crate::compat::Utf8Error;
```

## Constants

### `SIMD_CHUNK_SIZE`
```rust
const SIMD_CHUNK_SIZE: usize = 64usize;
```

