*[serde_core](../../index.md) / [ser](../index.md) / [impls](index.md)*

---

# Module `impls`

## Contents

- [Functions](#functions)
  - [`format_u8`](#format-u8)
- [Constants](#constants)
  - [`DEC_DIGITS_LUT`](#dec-digits-lut)
- [Macros](#macros)
  - [`primitive_impl!`](#primitive-impl)
  - [`array_impls!`](#array-impls)
  - [`seq_impl!`](#seq-impl)
  - [`tuple_impls!`](#tuple-impls)
  - [`tuple_impl_body!`](#tuple-impl-body)
  - [`map_impl!`](#map-impl)
  - [`deref_impl!`](#deref-impl)
  - [`nonzero_integers!`](#nonzero-integers)
  - [`serialize_display_bounded_length!`](#serialize-display-bounded-length)
  - [`atomic_impl!`](#atomic-impl)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`format_u8`](#format-u8) | fn |  |
| [`DEC_DIGITS_LUT`](#dec-digits-lut) | const |  |
| [`primitive_impl!`](#primitive-impl) | macro |  |
| [`array_impls!`](#array-impls) | macro |  |
| [`seq_impl!`](#seq-impl) | macro |  |
| [`tuple_impls!`](#tuple-impls) | macro |  |
| [`tuple_impl_body!`](#tuple-impl-body) | macro |  |
| [`map_impl!`](#map-impl) | macro |  |
| [`deref_impl!`](#deref-impl) | macro |  |
| [`nonzero_integers!`](#nonzero-integers) | macro |  |
| [`serialize_display_bounded_length!`](#serialize-display-bounded-length) | macro | Serialize a value that implements `Display` as a string, when that string is statically known to never have more than a constant `MAX_LEN` bytes. |
| [`atomic_impl!`](#atomic-impl) | macro |  |

## Functions

### `format_u8`

```rust
fn format_u8(n: u8, out: &mut [u8]) -> usize
```

## Constants

### `DEC_DIGITS_LUT`
```rust
const DEC_DIGITS_LUT: &[u8];
```

## Macros

### `primitive_impl!`

### `array_impls!`

### `seq_impl!`

### `tuple_impls!`

### `tuple_impl_body!`

### `map_impl!`

### `deref_impl!`

### `nonzero_integers!`

### `serialize_display_bounded_length!`

Serialize a value that implements `Display` as a string, when that string is
statically known to never have more than a constant `MAX_LEN` bytes.

Panics if the `Display` impl tries to write more than `MAX_LEN` bytes.

### `atomic_impl!`

