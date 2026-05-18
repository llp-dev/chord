*[num_bigint](../../index.md) / [bigint](../index.md) / [power](index.md)*

---

# Module `power`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`powsign`](#powsign) | fn | Help function for pow |
| [`modpow`](#modpow) | fn |  |
| [`pow_impl!`](#pow-impl) | macro |  |

## Functions

### `powsign`

```rust
fn powsign<T: Integer>(sign: super::Sign, other: &T) -> super::Sign
```

Help function for pow

Computes the effect of the exponent on the sign.

### `modpow`

```rust
fn modpow(x: &super::BigInt, exponent: &super::BigInt, modulus: &super::BigInt) -> super::BigInt
```

## Macros

### `pow_impl!`

