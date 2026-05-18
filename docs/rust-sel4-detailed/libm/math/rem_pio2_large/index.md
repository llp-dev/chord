*[libm](../../index.md) / [math](../index.md) / [rem_pio2_large](index.md)*

---

# Module `rem_pio2_large`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`rem_pio2_large`](#rem-pio2-large) | fn | Return the last three digits of N with y = x - N*pi/2 so that \|y\| < pi/2. |
| [`INIT_JK`](#init-jk) | const |  |
| [`IPIO2`](#ipio2) | const |  |
| [`PIO2`](#pio2) | const |  |

## Functions

### `rem_pio2_large`

```rust
fn rem_pio2_large(x: &[f64], y: &mut [f64], e0: i32, prec: usize) -> i32
```

Return the last three digits of N with y = x - N*pi/2
so that |y| < pi/2.

The method is to compute the integer (mod 8) and fraction parts of
(2/pi)*x without doing the full multiplication. In general we
skip the part of the product that are known to be a huge integer (
more accurately, = 0 mod 8 ). Thus the number of operations are
independent of the exponent of the input.

## Constants

### `INIT_JK`
```rust
const INIT_JK: [usize; 4];
```

### `IPIO2`
```rust
const IPIO2: [i32; 690];
```

### `PIO2`
```rust
const PIO2: [f64; 8];
```

