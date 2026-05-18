*[num_bigint](../../index.md) / [biguint](../index.md) / [shift](index.md)*

---

# Module `shift`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`biguint_shl`](#biguint-shl) | fn |  |
| [`biguint_shl2`](#biguint-shl2) | fn |  |
| [`biguint_shr`](#biguint-shr) | fn |  |
| [`biguint_shr2`](#biguint-shr2) | fn |  |
| [`impl_shift!`](#impl-shift) | macro |  |

## Functions

### `biguint_shl`

```rust
fn biguint_shl<T: PrimInt>(n: alloc::borrow::Cow<'_, super::BigUint>, shift: T) -> super::BigUint
```

### `biguint_shl2`

```rust
fn biguint_shl2(n: alloc::borrow::Cow<'_, super::BigUint>, digits: usize, shift: u8) -> super::BigUint
```

### `biguint_shr`

```rust
fn biguint_shr<T: PrimInt>(n: alloc::borrow::Cow<'_, super::BigUint>, shift: T) -> super::BigUint
```

### `biguint_shr2`

```rust
fn biguint_shr2(n: alloc::borrow::Cow<'_, super::BigUint>, digits: usize, shift: u8) -> super::BigUint
```

## Macros

### `impl_shift!`

