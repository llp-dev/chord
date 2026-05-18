*[num_bigint](../../index.md) / [biguint](../index.md) / [power](index.md)*

---

# Module `power`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`modpow`](#modpow) | fn |  |
| [`plain_modpow`](#plain-modpow) | fn |  |
| [`pow_impl!`](#pow-impl) | macro |  |

## Functions

### `modpow`

```rust
fn modpow(x: &super::BigUint, exponent: &super::BigUint, modulus: &super::BigUint) -> super::BigUint
```

### `plain_modpow`

```rust
fn plain_modpow(base: &super::BigUint, exp_data: &[u64], modulus: &super::BigUint) -> super::BigUint
```

## Macros

### `pow_impl!`

