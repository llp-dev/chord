*[libm](../../index.md) / [math](../index.md) / [asin](index.md)*

---

# Module `asin`

## Contents

- [Functions](#functions)
  - [`comp_r`](#comp-r)
  - [`asin`](#asin)
- [Constants](#constants)
  - [`PIO2_HI`](#pio2-hi)
  - [`PIO2_LO`](#pio2-lo)
  - [`P_S0`](#p-s0)
  - [`P_S1`](#p-s1)
  - [`P_S2`](#p-s2)
  - [`P_S3`](#p-s3)
  - [`P_S4`](#p-s4)
  - [`P_S5`](#p-s5)
  - [`Q_S1`](#q-s1)
  - [`Q_S2`](#q-s2)
  - [`Q_S3`](#q-s3)
  - [`Q_S4`](#q-s4)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`comp_r`](#comp-r) | fn |  |
| [`asin`](#asin) | fn | Arcsine (f64) |
| [`PIO2_HI`](#pio2-hi) | const |  |
| [`PIO2_LO`](#pio2-lo) | const |  |
| [`P_S0`](#p-s0) | const |  |
| [`P_S1`](#p-s1) | const |  |
| [`P_S2`](#p-s2) | const |  |
| [`P_S3`](#p-s3) | const |  |
| [`P_S4`](#p-s4) | const |  |
| [`P_S5`](#p-s5) | const |  |
| [`Q_S1`](#q-s1) | const |  |
| [`Q_S2`](#q-s2) | const |  |
| [`Q_S3`](#q-s3) | const |  |
| [`Q_S4`](#q-s4) | const |  |

## Functions

### `comp_r`

```rust
fn comp_r(z: f64) -> f64
```

### `asin`

```rust
fn asin(x: f64) -> f64
```

Arcsine (f64)

Computes the inverse sine (arc sine) of the argument `x`.
Arguments to asin must be in the range -1 to 1.
Returns values in radians, in the range of -pi/2 to pi/2.

## Constants

### `PIO2_HI`
```rust
const PIO2_HI: f64 = 1.5707963267948966f64;
```

### `PIO2_LO`
```rust
const PIO2_LO: f64 = 6.123233995736766E-17f64;
```

### `P_S0`
```rust
const P_S0: f64 = 0.16666666666666666f64;
```

### `P_S1`
```rust
const P_S1: f64 = -0.32556581862240092f64;
```

### `P_S2`
```rust
const P_S2: f64 = 0.20121253213486293f64;
```

### `P_S3`
```rust
const P_S3: f64 = -0.040055534500679411f64;
```

### `P_S4`
```rust
const P_S4: f64 = 7.9153499428981453E-4f64;
```

### `P_S5`
```rust
const P_S5: f64 = 3.4793310759602117E-5f64;
```

### `Q_S1`
```rust
const Q_S1: f64 = -2.4033949117344142f64;
```

### `Q_S2`
```rust
const Q_S2: f64 = 2.0209457602335057f64;
```

### `Q_S3`
```rust
const Q_S3: f64 = -0.68828397160545329f64;
```

### `Q_S4`
```rust
const Q_S4: f64 = 0.077038150555901935f64;
```

