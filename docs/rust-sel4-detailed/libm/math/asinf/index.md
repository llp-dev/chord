*[libm](../../index.md) / [math](../index.md) / [asinf](index.md)*

---

# Module `asinf`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`r`](#r) | fn |  |
| [`asinf`](#asinf) | fn | Arcsine (f32) |
| [`PIO2`](#pio2) | const |  |
| [`P_S0`](#p-s0) | const |  |
| [`P_S1`](#p-s1) | const |  |
| [`P_S2`](#p-s2) | const |  |
| [`Q_S1`](#q-s1) | const |  |

## Functions

### `r`

```rust
fn r(z: f32) -> f32
```

### `asinf`

```rust
fn asinf(x: f32) -> f32
```

Arcsine (f32)

Computes the inverse sine (arc sine) of the argument `x`.
Arguments to asin must be in the range -1 to 1.
Returns values in radians, in the range of -pi/2 to pi/2.

## Constants

### `PIO2`
```rust
const PIO2: f64 = 1.5707963267948966f64;
```

### `P_S0`
```rust
const P_S0: f32 = 0.166665867f32;
```

### `P_S1`
```rust
const P_S1: f32 = -0.0427434221f32;
```

### `P_S2`
```rust
const P_S2: f32 = -0.008656363f32;
```

### `Q_S1`
```rust
const Q_S1: f32 = -0.706629634f32;
```

