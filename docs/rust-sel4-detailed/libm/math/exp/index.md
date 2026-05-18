*[libm](../../index.md) / [math](../index.md) / [exp](index.md)*

---

# Module `exp`

## Contents

- [Functions](#functions)
  - [`exp`](#exp)
- [Constants](#constants)
  - [`HALF`](#half)
  - [`LN2HI`](#ln2hi)
  - [`LN2LO`](#ln2lo)
  - [`INVLN2`](#invln2)
  - [`P1`](#p1)
  - [`P2`](#p2)
  - [`P3`](#p3)
  - [`P4`](#p4)
  - [`P5`](#p5)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`exp`](#exp) | fn | Exponential, base *e* (f64) |
| [`HALF`](#half) | const |  |
| [`LN2HI`](#ln2hi) | const |  |
| [`LN2LO`](#ln2lo) | const |  |
| [`INVLN2`](#invln2) | const |  |
| [`P1`](#p1) | const |  |
| [`P2`](#p2) | const |  |
| [`P3`](#p3) | const |  |
| [`P4`](#p4) | const |  |
| [`P5`](#p5) | const |  |

## Functions

### `exp`

```rust
fn exp(x: f64) -> f64
```

Exponential, base *e* (f64)

Calculate the exponential of `x`, that is, *e* raised to the power `x`
(where *e* is the base of the natural system of logarithms, approximately 2.71828).

## Constants

### `HALF`
```rust
const HALF: [f64; 2];
```

### `LN2HI`
```rust
const LN2HI: f64 = 0.69314718036912382f64;
```

### `LN2LO`
```rust
const LN2LO: f64 = 1.9082149292705877E-10f64;
```

### `INVLN2`
```rust
const INVLN2: f64 = 1.4426950408889634f64;
```

### `P1`
```rust
const P1: f64 = 0.16666666666666602f64;
```

### `P2`
```rust
const P2: f64 = -0.0027777777777015593f64;
```

### `P3`
```rust
const P3: f64 = 6.6137563214379344E-5f64;
```

### `P4`
```rust
const P4: f64 = -1.6533902205465252E-6f64;
```

### `P5`
```rust
const P5: f64 = 4.1381367970572385E-8f64;
```

