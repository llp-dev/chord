*[libm](../../index.md) / [math](../index.md) / [rem_pio2f](index.md)*

---

# Module `rem_pio2f`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`rem_pio2f`](#rem-pio2f) | fn | Return the remainder of x rem pi/2 in *y |
| [`TOINT`](#toint) | const |  |
| [`INV_PIO2`](#inv-pio2) | const | 53 bits of 2/pi |
| [`PIO2_1`](#pio2-1) | const | first 25 bits of pi/2 |
| [`PIO2_1T`](#pio2-1t) | const | pi/2 - pio2_1 |

## Functions

### `rem_pio2f`

```rust
fn rem_pio2f(x: f32) -> (i32, f64)
```

Return the remainder of x rem pi/2 in *y

use double precision for everything except passing x
use __rem_pio2_large() for large x

## Constants

### `TOINT`
```rust
const TOINT: f64 = 6755399441055744f64;
```

### `INV_PIO2`
```rust
const INV_PIO2: f64 = 0.63661977236758138f64;
```

53 bits of 2/pi

### `PIO2_1`
```rust
const PIO2_1: f64 = 1.5707963109016418f64;
```

first 25 bits of pi/2

### `PIO2_1T`
```rust
const PIO2_1T: f64 = 1.5893254773528196E-8f64;
```

pi/2 - pio2_1

