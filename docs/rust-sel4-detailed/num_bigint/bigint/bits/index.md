*[num_bigint](../../index.md) / [bigint](../index.md) / [bits](index.md)*

---

# Module `bits`

## Contents

- [Functions](#functions)
  - [`negate_carry`](#negate-carry)
  - [`bitand_pos_neg`](#bitand-pos-neg)
  - [`bitand_neg_pos`](#bitand-neg-pos)
  - [`bitand_neg_neg`](#bitand-neg-neg)
  - [`bitor_pos_neg`](#bitor-pos-neg)
  - [`bitor_neg_pos`](#bitor-neg-pos)
  - [`bitor_neg_neg`](#bitor-neg-neg)
  - [`bitxor_pos_neg`](#bitxor-pos-neg)
  - [`bitxor_neg_pos`](#bitxor-neg-pos)
  - [`bitxor_neg_neg`](#bitxor-neg-neg)
  - [`set_negative_bit`](#set-negative-bit)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`negate_carry`](#negate-carry) | fn |  |
| [`bitand_pos_neg`](#bitand-pos-neg) | fn |  |
| [`bitand_neg_pos`](#bitand-neg-pos) | fn |  |
| [`bitand_neg_neg`](#bitand-neg-neg) | fn |  |
| [`bitor_pos_neg`](#bitor-pos-neg) | fn |  |
| [`bitor_neg_pos`](#bitor-neg-pos) | fn |  |
| [`bitor_neg_neg`](#bitor-neg-neg) | fn |  |
| [`bitxor_pos_neg`](#bitxor-pos-neg) | fn |  |
| [`bitxor_neg_pos`](#bitxor-neg-pos) | fn |  |
| [`bitxor_neg_neg`](#bitxor-neg-neg) | fn |  |
| [`set_negative_bit`](#set-negative-bit) | fn |  |

## Functions

### `negate_carry`

```rust
fn negate_carry(a: u64, acc: &mut u128) -> u64
```

### `bitand_pos_neg`

```rust
fn bitand_pos_neg(a: &mut [u64], b: &[u64])
```

### `bitand_neg_pos`

```rust
fn bitand_neg_pos(a: &mut alloc::vec::Vec<u64>, b: &[u64])
```

### `bitand_neg_neg`

```rust
fn bitand_neg_neg(a: &mut alloc::vec::Vec<u64>, b: &[u64])
```

### `bitor_pos_neg`

```rust
fn bitor_pos_neg(a: &mut alloc::vec::Vec<u64>, b: &[u64])
```

### `bitor_neg_pos`

```rust
fn bitor_neg_pos(a: &mut [u64], b: &[u64])
```

### `bitor_neg_neg`

```rust
fn bitor_neg_neg(a: &mut alloc::vec::Vec<u64>, b: &[u64])
```

### `bitxor_pos_neg`

```rust
fn bitxor_pos_neg(a: &mut alloc::vec::Vec<u64>, b: &[u64])
```

### `bitxor_neg_pos`

```rust
fn bitxor_neg_pos(a: &mut alloc::vec::Vec<u64>, b: &[u64])
```

### `bitxor_neg_neg`

```rust
fn bitxor_neg_neg(a: &mut alloc::vec::Vec<u64>, b: &[u64])
```

### `set_negative_bit`

```rust
fn set_negative_bit(x: &mut super::BigInt, bit: u64, value: bool)
```

