*[libm](../../index.md) / [math](../index.md) / [expm1f](index.md)*

---

# Module `expm1f`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`expm1f`](#expm1f) | fn | Exponential, base *e*, of x-1 (f32) |
| [`LN2_HI`](#ln2-hi) | const |  |
| [`LN2_LO`](#ln2-lo) | const |  |
| [`INV_LN2`](#inv-ln2) | const |  |
| [`Q1`](#q1) | const |  |
| [`Q2`](#q2) | const |  |

## Functions

### `expm1f`

```rust
fn expm1f(x: f32) -> f32
```

Exponential, base *e*, of x-1 (f32)

Calculates the exponential of `x` and subtract 1, that is, *e* raised
to the power `x` minus 1 (where *e* is the base of the natural
system of logarithms, approximately 2.71828).
The result is accurate even for small values of `x`,
where using `exp(x)-1` would lose many significant digits.

## Constants

### `LN2_HI`
```rust
const LN2_HI: f32 = 0.693138123f32;
```

### `LN2_LO`
```rust
const LN2_LO: f32 = 9.05800061E-6f32;
```

### `INV_LN2`
```rust
const INV_LN2: f32 = 1.44269502f32;
```

### `Q1`
```rust
const Q1: f32 = -0.0333332121f32;
```

### `Q2`
```rust
const Q2: f32 = 0.00158071704f32;
```

