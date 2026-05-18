*[miniz_oxide](../../index.md) / [deflate](../index.md) / [zlib](index.md)*

---

# Module `zlib`

## Contents

- [Functions](#functions)
  - [`add_fcheck`](#add-fcheck)
  - [`zlib_level_from_flags`](#zlib-level-from-flags)
  - [`header_from_level`](#header-from-level)
  - [`header_from_flags`](#header-from-flags)
- [Constants](#constants)
  - [`DEFAULT_CM`](#default-cm)
  - [`_DEFAULT_CINFO`](#default-cinfo)
  - [`_DEFAULT_FDICT`](#default-fdict)
  - [`_DEFAULT_CMF`](#default-cmf)
  - [`_MIN_CMF`](#min-cmf)
  - [`FCHECK_DIVISOR`](#fcheck-divisor)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`add_fcheck`](#add-fcheck) | fn | Generate FCHECK from CMF and FLG (without FCKECH )so that they are correct according to the specification, i.e (CMF*256 + FCHK) % 31 = 0. |
| [`zlib_level_from_flags`](#zlib-level-from-flags) | fn |  |
| [`header_from_level`](#header-from-level) | fn | Get the zlib header for the level using the default window size and no dictionary. |
| [`header_from_flags`](#header-from-flags) | fn | Create a zlib header from the given compression flags. |
| [`DEFAULT_CM`](#default-cm) | const |  |
| [`_DEFAULT_CINFO`](#default-cinfo) | const |  |
| [`_DEFAULT_FDICT`](#default-fdict) | const |  |
| [`_DEFAULT_CMF`](#default-cmf) | const |  |
| [`_MIN_CMF`](#min-cmf) | const |  |
| [`FCHECK_DIVISOR`](#fcheck-divisor) | const | The 16-bit value consisting of CMF and FLG must be divisible by this to be valid. |

## Functions

### `add_fcheck`

```rust
fn add_fcheck(cmf: u8, flg: u8) -> u8
```

Generate FCHECK from CMF and FLG (without FCKECH )so that they are correct according to the
specification, i.e (CMF*256 + FCHK) % 31 = 0.
Returns flg with the FCHKECK bits added (any existing FCHECK bits are ignored).

### `zlib_level_from_flags`

```rust
const fn zlib_level_from_flags(flags: u32) -> u8
```

### `header_from_level`

```rust
fn header_from_level(level: u8, window_bits: u8) -> [u8; 2]
```

Get the zlib header for the level using the default window size and no
dictionary.

### `header_from_flags`

```rust
fn header_from_flags(flags: u32, window_bits: u8) -> [u8; 2]
```

Create a zlib header from the given compression flags.
Only level is considered.

## Constants

### `DEFAULT_CM`
```rust
const DEFAULT_CM: u8 = 8u8;
```

### `_DEFAULT_CINFO`
```rust
const _DEFAULT_CINFO: u8 = 112u8;
```

### `_DEFAULT_FDICT`
```rust
const _DEFAULT_FDICT: u8 = 0u8;
```

### `_DEFAULT_CMF`
```rust
const _DEFAULT_CMF: u8 = 120u8;
```

### `_MIN_CMF`
```rust
const _MIN_CMF: u8 = 8u8;
```

### `FCHECK_DIVISOR`
```rust
const FCHECK_DIVISOR: u8 = 31u8;
```

The 16-bit value consisting of CMF and FLG must be divisible by this to be valid.

