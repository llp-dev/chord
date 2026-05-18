*[libm](../../index.md) / [math](../index.md) / [rem_pio2](index.md)*

---

# Module `rem_pio2`

## Contents

- [Functions](#functions)
  - [`rem_pio2`](#rem-pio2)
- [Constants](#constants)
  - [`EPS`](#eps)
  - [`TO_INT`](#to-int)
  - [`INV_PIO2`](#inv-pio2)
  - [`PIO2_1`](#pio2-1)
  - [`PIO2_1T`](#pio2-1t)
  - [`PIO2_2`](#pio2-2)
  - [`PIO2_2T`](#pio2-2t)
  - [`PIO2_3`](#pio2-3)
  - [`PIO2_3T`](#pio2-3t)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`rem_pio2`](#rem-pio2) | fn |  |
| [`EPS`](#eps) | const |  |
| [`TO_INT`](#to-int) | const |  |
| [`INV_PIO2`](#inv-pio2) | const | 53 bits of 2/pi |
| [`PIO2_1`](#pio2-1) | const | first 33 bits of pi/2 |
| [`PIO2_1T`](#pio2-1t) | const | pi/2 - PIO2_1 |
| [`PIO2_2`](#pio2-2) | const | second 33 bits of pi/2 |
| [`PIO2_2T`](#pio2-2t) | const | pi/2 - (PIO2_1+PIO2_2) |
| [`PIO2_3`](#pio2-3) | const | third 33 bits of pi/2 |
| [`PIO2_3T`](#pio2-3t) | const | pi/2 - (PIO2_1+PIO2_2+PIO2_3) |

## Functions

### `rem_pio2`

```rust
fn rem_pio2(x: f64) -> (i32, f64, f64)
```

## Constants

### `EPS`
```rust
const EPS: f64 = 2.2204460492503131E-16f64;
```

### `TO_INT`
```rust
const TO_INT: f64 = 6755399441055744f64;
```

### `INV_PIO2`
```rust
const INV_PIO2: f64 = 0.63661977236758138f64;
```

53 bits of 2/pi

### `PIO2_1`
```rust
const PIO2_1: f64 = 1.5707963267341256f64;
```

first 33 bits of pi/2

### `PIO2_1T`
```rust
const PIO2_1T: f64 = 6.0771005065061922E-11f64;
```

pi/2 - PIO2_1

### `PIO2_2`
```rust
const PIO2_2: f64 = 6.077100506303966E-11f64;
```

second 33 bits of pi/2

### `PIO2_2T`
```rust
const PIO2_2T: f64 = 2.0222662487959506E-21f64;
```

pi/2 - (PIO2_1+PIO2_2)

### `PIO2_3`
```rust
const PIO2_3: f64 = 2.0222662487111665E-21f64;
```

third 33 bits of pi/2

### `PIO2_3T`
```rust
const PIO2_3T: f64 = 8.4784276603688995E-32f64;
```

pi/2 - (PIO2_1+PIO2_2+PIO2_3)

