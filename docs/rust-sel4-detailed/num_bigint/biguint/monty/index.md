*[num_bigint](../../index.md) / [biguint](../index.md) / [monty](index.md)*

---

# Module `monty`

## Contents

- [Structs](#structs)
  - [`MontyReducer`](#montyreducer)
- [Functions](#functions)
  - [`inv_mod_alt`](#inv-mod-alt)
  - [`montgomery`](#montgomery)
  - [`add_mul_vvw`](#add-mul-vvw)
  - [`sub_vv`](#sub-vv)
  - [`add_ww`](#add-ww)
  - [`mul_add_www`](#mul-add-www)
  - [`monty_modpow`](#monty-modpow)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`MontyReducer`](#montyreducer) | struct |  |
| [`inv_mod_alt`](#inv-mod-alt) | fn |  |
| [`montgomery`](#montgomery) | fn | Computes z mod m = x * y * 2 ** (-n*_W) mod m assuming k = -1/m mod 2**_W See Gueron, "Efficient Software Implementations of Modular Exponentiation". |
| [`add_mul_vvw`](#add-mul-vvw) | fn |  |
| [`sub_vv`](#sub-vv) | fn | The resulting carry c is either 0 or 1. |
| [`add_ww`](#add-ww) | fn | z1<<_W + z0 = x+y+c, with c == 0 or 1 |
| [`mul_add_www`](#mul-add-www) | fn | z1 << _W + z0 = x * y + c |
| [`monty_modpow`](#monty-modpow) | fn | Calculates x ** y mod m using a fixed, 4-bit window. |

## Structs

### `MontyReducer`

```rust
struct MontyReducer {
    n0inv: u64,
}
```

#### Implementations

- <span id="montyreducer-new"></span>`fn new(n: &BigUint) -> Self` — [`BigUint`](../index.md#biguint)

## Functions

### `inv_mod_alt`

```rust
fn inv_mod_alt(b: u64) -> u64
```

### `montgomery`

```rust
fn montgomery(x: &crate::biguint::BigUint, y: &crate::biguint::BigUint, m: &crate::biguint::BigUint, k: u64, n: usize) -> crate::biguint::BigUint
```

Computes z mod m = x * y * 2 ** (-n*_W) mod m
assuming k = -1/m mod 2**_W
See Gueron, "Efficient Software Implementations of Modular Exponentiation".
<https://eprint.iacr.org/2011/239.pdf>
In the terminology of that paper, this is an "Almost Montgomery Multiplication":
x and y are required to satisfy 0 <= z < 2**(n*_W) and then the result
z is guaranteed to satisfy 0 <= z < 2**(n*_W), but it may not be < m.

### `add_mul_vvw`

```rust
fn add_mul_vvw(z: &mut [u64], x: &[u64], y: u64) -> u64
```

### `sub_vv`

```rust
fn sub_vv(z: &mut [u64], x: &[u64], y: &[u64]) -> u64
```

The resulting carry c is either 0 or 1.

### `add_ww`

```rust
fn add_ww(x: u64, y: u64, c: u64) -> (u64, u64)
```

z1<<_W + z0 = x+y+c, with c == 0 or 1

### `mul_add_www`

```rust
fn mul_add_www(x: u64, y: u64, c: u64) -> (u64, u64)
```

z1 << _W + z0 = x * y + c

### `monty_modpow`

```rust
fn monty_modpow(x: &crate::biguint::BigUint, y: &crate::biguint::BigUint, m: &crate::biguint::BigUint) -> crate::biguint::BigUint
```

Calculates x ** y mod m using a fixed, 4-bit window.

