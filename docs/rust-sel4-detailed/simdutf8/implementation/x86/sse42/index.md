*[simdutf8](../../../index.md) / [implementation](../../index.md) / [x86](../index.md) / [sse42](index.md)*

---

# Module `sse42`

Contains the x86-64/x86 SSE4.2 UTF-8 validation implementation.

## Contents

- [Structs](#structs)
  - [`SimdInput`](#simdinput)
- [Functions](#functions)
  - [`simd_prefetch`](#simd-prefetch)
  - [`validate_utf8_basic`](#validate-utf8-basic)
  - [`validate_utf8_compat`](#validate-utf8-compat)
  - [`validate_utf8_compat_simd0`](#validate-utf8-compat-simd0)
- [Type Aliases](#type-aliases)
  - [`SimdU8Value`](#simdu8value)
- [Constants](#constants)
  - [`PREFETCH`](#prefetch)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`SimdInput`](#simdinput) | struct |  |
| [`simd_prefetch`](#simd-prefetch) | fn |  |
| [`validate_utf8_basic`](#validate-utf8-basic) | fn | Validation implementation for CPUs supporting the SIMD extension (see module). |
| [`validate_utf8_compat`](#validate-utf8-compat) | fn | Validation implementation for CPUs supporting the SIMD extension (see module). |
| [`validate_utf8_compat_simd0`](#validate-utf8-compat-simd0) | fn |  |
| [`SimdU8Value`](#simdu8value) | type |  |
| [`PREFETCH`](#prefetch) | const |  |

## Structs

### `SimdInput`

```rust
struct SimdInput {
    vals: [crate::implementation::helpers::SimdU8Value<core::arch::x86_64::__m128i>; 4],
}
```

#### Implementations

- <span id="simdinput-new"></span>`unsafe fn new(ptr: &[u8]) -> Self`

- <span id="simdinput-is-ascii"></span>`unsafe fn is_ascii(&self) -> bool`

## Functions

### `simd_prefetch`

```rust
unsafe fn simd_prefetch(ptr: *const u8)
```

### `validate_utf8_basic`

```rust
unsafe fn validate_utf8_basic(input: &[u8]) -> core::result::Result<(), basic::Utf8Error>
```

Validation implementation for CPUs supporting the SIMD extension (see module).

# Errors
Returns the zero-sized [`basic::Utf8Error`](../../../basic/index.md) on failure.

# Safety
This function is inherently unsafe because it is compiled with SIMD extensions
enabled. Make sure that the CPU supports it before calling.


### `validate_utf8_compat`

```rust
unsafe fn validate_utf8_compat(input: &[u8]) -> core::result::Result<(), compat::Utf8Error>
```

Validation implementation for CPUs supporting the SIMD extension (see module).

# Errors
Returns [`compat::Utf8Error`](../../../compat/index.md) with detailed error information on failure.

# Safety
This function is inherently unsafe because it is compiled with SIMD extensions
enabled. Make sure that the CPU supports it before calling.


### `validate_utf8_compat_simd0`

```rust
unsafe fn validate_utf8_compat_simd0(input: &[u8]) -> core::result::Result<(), usize>
```

## Type Aliases

### `SimdU8Value`

```rust
type SimdU8Value = crate::implementation::helpers::SimdU8Value<core::arch::x86_64::__m128i>;
```

## Constants

### `PREFETCH`
```rust
const PREFETCH: bool = false;
```

