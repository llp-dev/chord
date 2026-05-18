*[libm](../../index.md) / [math](../index.md) / [pow](index.md)*

---

# Module `pow`

## Contents

- [Functions](#functions)
  - [`pow`](#pow)
- [Constants](#constants)
  - [`BP`](#bp)
  - [`DP_H`](#dp-h)
  - [`DP_L`](#dp-l)
  - [`TWO53`](#two53)
  - [`HUGE`](#huge)
  - [`TINY`](#tiny)
  - [`L1`](#l1)
  - [`L2`](#l2)
  - [`L3`](#l3)
  - [`L4`](#l4)
  - [`L5`](#l5)
  - [`L6`](#l6)
  - [`P1`](#p1)
  - [`P2`](#p2)
  - [`P3`](#p3)
  - [`P4`](#p4)
  - [`P5`](#p5)
  - [`LG2`](#lg2)
  - [`LG2_H`](#lg2-h)
  - [`LG2_L`](#lg2-l)
  - [`OVT`](#ovt)
  - [`CP`](#cp)
  - [`CP_H`](#cp-h)
  - [`CP_L`](#cp-l)
  - [`IVLN2`](#ivln2)
  - [`IVLN2_H`](#ivln2-h)
  - [`IVLN2_L`](#ivln2-l)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`pow`](#pow) | fn | Returns `x` to the power of `y` (f64). |
| [`BP`](#bp) | const |  |
| [`DP_H`](#dp-h) | const |  |
| [`DP_L`](#dp-l) | const |  |
| [`TWO53`](#two53) | const |  |
| [`HUGE`](#huge) | const |  |
| [`TINY`](#tiny) | const |  |
| [`L1`](#l1) | const |  |
| [`L2`](#l2) | const |  |
| [`L3`](#l3) | const |  |
| [`L4`](#l4) | const |  |
| [`L5`](#l5) | const |  |
| [`L6`](#l6) | const |  |
| [`P1`](#p1) | const |  |
| [`P2`](#p2) | const |  |
| [`P3`](#p3) | const |  |
| [`P4`](#p4) | const |  |
| [`P5`](#p5) | const |  |
| [`LG2`](#lg2) | const |  |
| [`LG2_H`](#lg2-h) | const |  |
| [`LG2_L`](#lg2-l) | const |  |
| [`OVT`](#ovt) | const |  |
| [`CP`](#cp) | const |  |
| [`CP_H`](#cp-h) | const |  |
| [`CP_L`](#cp-l) | const |  |
| [`IVLN2`](#ivln2) | const |  |
| [`IVLN2_H`](#ivln2-h) | const |  |
| [`IVLN2_L`](#ivln2-l) | const |  |

## Functions

### `pow`

```rust
fn pow(x: f64, y: f64) -> f64
```

Returns `x` to the power of `y` (f64).

## Constants

### `BP`
```rust
const BP: [f64; 2];
```

### `DP_H`
```rust
const DP_H: [f64; 2];
```

### `DP_L`
```rust
const DP_L: [f64; 2];
```

### `TWO53`
```rust
const TWO53: f64 = 9007199254740992f64;
```

### `HUGE`
```rust
const HUGE: f64 = 1.0000000000000001E+300f64;
```

### `TINY`
```rust
const TINY: f64 = 1.0E-300f64;
```

### `L1`
```rust
const L1: f64 = 0.59999999999999465f64;
```

### `L2`
```rust
const L2: f64 = 0.42857142857855018f64;
```

### `L3`
```rust
const L3: f64 = 0.33333332981837743f64;
```

### `L4`
```rust
const L4: f64 = 0.27272812380853401f64;
```

### `L5`
```rust
const L5: f64 = 0.23066074577556175f64;
```

### `L6`
```rust
const L6: f64 = 0.20697501780033842f64;
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

### `LG2`
```rust
const LG2: f64 = 0.69314718055994529f64;
```

### `LG2_H`
```rust
const LG2_H: f64 = 0.69314718246459961f64;
```

### `LG2_L`
```rust
const LG2_L: f64 = -1.904654299957768E-9f64;
```

### `OVT`
```rust
const OVT: f64 = 8.0085662595372941E-17f64;
```

### `CP`
```rust
const CP: f64 = 0.96179669392597555f64;
```

### `CP_H`
```rust
const CP_H: f64 = 0.96179670095443726f64;
```

### `CP_L`
```rust
const CP_L: f64 = -7.0284616509527583E-9f64;
```

### `IVLN2`
```rust
const IVLN2: f64 = 1.4426950408889634f64;
```

### `IVLN2_H`
```rust
const IVLN2_H: f64 = 1.4426950216293335f64;
```

### `IVLN2_L`
```rust
const IVLN2_L: f64 = 1.9259629911266175E-8f64;
```

