*[libm](../../index.md) / [math](../index.md) / [acos](index.md)*

---

# Module `acos`

## Contents

- [Functions](#functions)
  - [`r`](#r)
  - [`acos`](#acos)
- [Constants](#constants)
  - [`PIO2_HI`](#pio2-hi)
  - [`PIO2_LO`](#pio2-lo)
  - [`PS0`](#ps0)
  - [`PS1`](#ps1)
  - [`PS2`](#ps2)
  - [`PS3`](#ps3)
  - [`PS4`](#ps4)
  - [`PS5`](#ps5)
  - [`QS1`](#qs1)
  - [`QS2`](#qs2)
  - [`QS3`](#qs3)
  - [`QS4`](#qs4)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`r`](#r) | fn |  |
| [`acos`](#acos) | fn | Arccosine (f64) |
| [`PIO2_HI`](#pio2-hi) | const |  |
| [`PIO2_LO`](#pio2-lo) | const |  |
| [`PS0`](#ps0) | const |  |
| [`PS1`](#ps1) | const |  |
| [`PS2`](#ps2) | const |  |
| [`PS3`](#ps3) | const |  |
| [`PS4`](#ps4) | const |  |
| [`PS5`](#ps5) | const |  |
| [`QS1`](#qs1) | const |  |
| [`QS2`](#qs2) | const |  |
| [`QS3`](#qs3) | const |  |
| [`QS4`](#qs4) | const |  |

## Functions

### `r`

```rust
fn r(z: f64) -> f64
```

### `acos`

```rust
fn acos(x: f64) -> f64
```

Arccosine (f64)

Computes the inverse cosine (arc cosine) of the input value.
Arguments must be in the range -1 to 1.
Returns values in radians, in the range of 0 to pi.

## Constants

### `PIO2_HI`
```rust
const PIO2_HI: f64 = 1.5707963267948966f64;
```

### `PIO2_LO`
```rust
const PIO2_LO: f64 = 6.123233995736766E-17f64;
```

### `PS0`
```rust
const PS0: f64 = 0.16666666666666666f64;
```

### `PS1`
```rust
const PS1: f64 = -0.32556581862240092f64;
```

### `PS2`
```rust
const PS2: f64 = 0.20121253213486293f64;
```

### `PS3`
```rust
const PS3: f64 = -0.040055534500679411f64;
```

### `PS4`
```rust
const PS4: f64 = 7.9153499428981453E-4f64;
```

### `PS5`
```rust
const PS5: f64 = 3.4793310759602117E-5f64;
```

### `QS1`
```rust
const QS1: f64 = -2.4033949117344142f64;
```

### `QS2`
```rust
const QS2: f64 = 2.0209457602335057f64;
```

### `QS3`
```rust
const QS3: f64 = -0.68828397160545329f64;
```

### `QS4`
```rust
const QS4: f64 = 0.077038150555901935f64;
```

