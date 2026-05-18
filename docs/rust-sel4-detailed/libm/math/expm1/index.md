*[libm](../../index.md) / [math](../index.md) / [expm1](index.md)*

---

# Module `expm1`

## Contents

- [Functions](#functions)
  - [`expm1`](#expm1)
- [Constants](#constants)
  - [`O_THRESHOLD`](#o-threshold)
  - [`LN2_HI`](#ln2-hi)
  - [`LN2_LO`](#ln2-lo)
  - [`INVLN2`](#invln2)
  - [`Q1`](#q1)
  - [`Q2`](#q2)
  - [`Q3`](#q3)
  - [`Q4`](#q4)
  - [`Q5`](#q5)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`expm1`](#expm1) | fn | Exponential, base *e*, of x-1 (f64) |
| [`O_THRESHOLD`](#o-threshold) | const |  |
| [`LN2_HI`](#ln2-hi) | const |  |
| [`LN2_LO`](#ln2-lo) | const |  |
| [`INVLN2`](#invln2) | const |  |
| [`Q1`](#q1) | const |  |
| [`Q2`](#q2) | const |  |
| [`Q3`](#q3) | const |  |
| [`Q4`](#q4) | const |  |
| [`Q5`](#q5) | const |  |

## Functions

### `expm1`

```rust
fn expm1(x: f64) -> f64
```

Exponential, base *e*, of x-1 (f64)

Calculates the exponential of `x` and subtract 1, that is, *e* raised
to the power `x` minus 1 (where *e* is the base of the natural
system of logarithms, approximately 2.71828).
The result is accurate even for small values of `x`,
where using `exp(x)-1` would lose many significant digits.

## Constants

### `O_THRESHOLD`
```rust
const O_THRESHOLD: f64 = 709.78271289338397f64;
```

### `LN2_HI`
```rust
const LN2_HI: f64 = 0.69314718036912382f64;
```

### `LN2_LO`
```rust
const LN2_LO: f64 = 1.9082149292705877E-10f64;
```

### `INVLN2`
```rust
const INVLN2: f64 = 1.4426950408889634f64;
```

### `Q1`
```rust
const Q1: f64 = -0.033333333333333132f64;
```

### `Q2`
```rust
const Q2: f64 = 0.0015873015872548146f64;
```

### `Q3`
```rust
const Q3: f64 = -7.9365075786748794E-5f64;
```

### `Q4`
```rust
const Q4: f64 = 4.0082178273293624E-6f64;
```

### `Q5`
```rust
const Q5: f64 = -2.0109921818362437E-7f64;
```

