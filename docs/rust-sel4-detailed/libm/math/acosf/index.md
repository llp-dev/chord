*[libm](../../index.md) / [math](../index.md) / [acosf](index.md)*

---

# Module `acosf`

## Contents

- [Functions](#functions)
  - [`r`](#r)
  - [`acosf`](#acosf)
- [Constants](#constants)
  - [`PIO2_HI`](#pio2-hi)
  - [`PIO2_LO`](#pio2-lo)
  - [`P_S0`](#p-s0)
  - [`P_S1`](#p-s1)
  - [`P_S2`](#p-s2)
  - [`Q_S1`](#q-s1)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`r`](#r) | fn |  |
| [`acosf`](#acosf) | fn | Arccosine (f32) |
| [`PIO2_HI`](#pio2-hi) | const |  |
| [`PIO2_LO`](#pio2-lo) | const |  |
| [`P_S0`](#p-s0) | const |  |
| [`P_S1`](#p-s1) | const |  |
| [`P_S2`](#p-s2) | const |  |
| [`Q_S1`](#q-s1) | const |  |

## Functions

### `r`

```rust
fn r(z: f32) -> f32
```

### `acosf`

```rust
fn acosf(x: f32) -> f32
```

Arccosine (f32)

Computes the inverse cosine (arc cosine) of the input value.
Arguments must be in the range -1 to 1.
Returns values in radians, in the range of 0 to pi.

## Constants

### `PIO2_HI`
```rust
const PIO2_HI: f32 = 1.57079625f32;
```

### `PIO2_LO`
```rust
const PIO2_LO: f32 = 7.54978942E-8f32;
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

