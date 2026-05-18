*[num_bigint](../../index.md) / [biguint](../index.md) / [addition](index.md)*

---

# Module `addition`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`adc`](#adc) | fn |  |
| [`__add2`](#add2) | fn | Two argument addition of raw slices, `a += b`, returning the carry. |
| [`add2`](#add2) | fn | Two argument addition of raw slices: a += b |

## Functions

### `adc`

```rust
fn adc(carry: u8, a: u64, b: u64, out: &mut u64) -> u8
```

### `__add2`

```rust
fn __add2(a: &mut [u64], b: &[u64]) -> u64
```

Two argument addition of raw slices, `a += b`, returning the carry.

This is used when the data `Vec` might need to resize to push a non-zero carry, so we perform
the addition first hoping that it will fit.

The caller _must_ ensure that `a` is at least as long as `b`.

### `add2`

```rust
fn add2(a: &mut [u64], b: &[u64])
```

Two argument addition of raw slices:
a += b

The caller _must_ ensure that a is big enough to store the result - typically this means
resizing a to max(a.len(), b.len()) + 1, to fit a possible carry.

