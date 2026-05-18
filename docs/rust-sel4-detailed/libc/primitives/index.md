*[libc](../index.md) / [primitives](index.md)*

---

# Module `primitives`

This module contains type aliases for C's platform-specific types
and fixed-width integer types.

The platform-specific types definitions were taken from rust-lang/rust in
library/core/src/ffi/primitives.rs

The fixed-width integer aliases are deprecated: use the Rust types instead.

## Contents

- [Type Aliases](#type-aliases)
  - [`c_schar`](#c-schar)
  - [`c_uchar`](#c-uchar)
  - [`c_short`](#c-short)
  - [`c_ushort`](#c-ushort)
  - [`c_longlong`](#c-longlong)
  - [`c_ulonglong`](#c-ulonglong)
  - [`c_float`](#c-float)
  - [`c_double`](#c-double)
  - [`c_char`](#c-char)
  - [`c_int`](#c-int)
  - [`c_uint`](#c-uint)
  - [`c_long`](#c-long)
  - [`c_ulong`](#c-ulong)
  - [`int8_t`](#int8-t)
  - [`int16_t`](#int16-t)
  - [`int32_t`](#int32-t)
  - [`int64_t`](#int64-t)
  - [`uint8_t`](#uint8-t)
  - [`uint16_t`](#uint16-t)
  - [`uint32_t`](#uint32-t)
  - [`uint64_t`](#uint64-t)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`c_schar`](#c-schar) | type |  |
| [`c_uchar`](#c-uchar) | type |  |
| [`c_short`](#c-short) | type |  |
| [`c_ushort`](#c-ushort) | type |  |
| [`c_longlong`](#c-longlong) | type |  |
| [`c_ulonglong`](#c-ulonglong) | type |  |
| [`c_float`](#c-float) | type |  |
| [`c_double`](#c-double) | type |  |
| [`c_char`](#c-char) | type |  |
| [`c_int`](#c-int) | type |  |
| [`c_uint`](#c-uint) | type |  |
| [`c_long`](#c-long) | type |  |
| [`c_ulong`](#c-ulong) | type |  |
| [`int8_t`](#int8-t) | type |  |
| [`int16_t`](#int16-t) | type |  |
| [`int32_t`](#int32-t) | type |  |
| [`int64_t`](#int64-t) | type |  |
| [`uint8_t`](#uint8-t) | type |  |
| [`uint16_t`](#uint16-t) | type |  |
| [`uint32_t`](#uint32-t) | type |  |
| [`uint64_t`](#uint64-t) | type |  |

## Type Aliases

### `c_schar`

```rust
type c_schar = i8;
```

### `c_uchar`

```rust
type c_uchar = u8;
```

### `c_short`

```rust
type c_short = i16;
```

### `c_ushort`

```rust
type c_ushort = u16;
```

### `c_longlong`

```rust
type c_longlong = i64;
```

### `c_ulonglong`

```rust
type c_ulonglong = u64;
```

### `c_float`

```rust
type c_float = f32;
```

### `c_double`

```rust
type c_double = f64;
```

### `c_char`

```rust
type c_char = i8;
```

### `c_int`

```rust
type c_int = i32;
```

### `c_uint`

```rust
type c_uint = u32;
```

### `c_long`

```rust
type c_long = i64;
```

### `c_ulong`

```rust
type c_ulong = u64;
```

### `int8_t`

```rust
type int8_t = i8;
```

### `int16_t`

```rust
type int16_t = i16;
```

### `int32_t`

```rust
type int32_t = i32;
```

### `int64_t`

```rust
type int64_t = i64;
```

### `uint8_t`

```rust
type uint8_t = u8;
```

### `uint16_t`

```rust
type uint16_t = u16;
```

### `uint32_t`

```rust
type uint32_t = u32;
```

### `uint64_t`

```rust
type uint64_t = u64;
```

