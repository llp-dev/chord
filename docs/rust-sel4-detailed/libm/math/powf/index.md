*[libm](../../index.md) / [math](../index.md) / [powf](index.md)*

---

# Module `powf`

## Contents

- [Functions](#functions)
  - [`powf`](#powf)
- [Constants](#constants)
  - [`BP`](#bp)
  - [`DP_H`](#dp-h)
  - [`DP_L`](#dp-l)
  - [`TWO24`](#two24)
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
| [`powf`](#powf) | fn | Returns `x` to the power of `y` (f32). |
| [`BP`](#bp) | const |  |
| [`DP_H`](#dp-h) | const |  |
| [`DP_L`](#dp-l) | const |  |
| [`TWO24`](#two24) | const |  |
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

### `powf`

```rust
fn powf(x: f32, y: f32) -> f32
```

Returns `x` to the power of `y` (f32).

## Constants

### `BP`
```rust
const BP: [f32; 2];
```

### `DP_H`
```rust
const DP_H: [f32; 2];
```

### `DP_L`
```rust
const DP_L: [f32; 2];
```

### `TWO24`
```rust
const TWO24: f32 = 16777216f32;
```

### `HUGE`
```rust
const HUGE: f32 = 1.00000002E+30f32;
```

### `TINY`
```rust
const TINY: f32 = 1.0E-30f32;
```

### `L1`
```rust
const L1: f32 = 0.600000024f32;
```

### `L2`
```rust
const L2: f32 = 0.428571433f32;
```

### `L3`
```rust
const L3: f32 = 0.333333343f32;
```

### `L4`
```rust
const L4: f32 = 0.272728115f32;
```

### `L5`
```rust
const L5: f32 = 0.230660751f32;
```

### `L6`
```rust
const L6: f32 = 0.206975013f32;
```

### `P1`
```rust
const P1: f32 = 0.166666672f32;
```

### `P2`
```rust
const P2: f32 = -0.00277777785f32;
```

### `P3`
```rust
const P3: f32 = 6.61375597E-5f32;
```

### `P4`
```rust
const P4: f32 = -1.6533902E-6f32;
```

### `P5`
```rust
const P5: f32 = 4.13813694E-8f32;
```

### `LG2`
```rust
const LG2: f32 = 0.693147182f32;
```

### `LG2_H`
```rust
const LG2_H: f32 = 0.693145751f32;
```

### `LG2_L`
```rust
const LG2_L: f32 = 1.42860654E-6f32;
```

### `OVT`
```rust
const OVT: f32 = 4.29956657E-8f32;
```

### `CP`
```rust
const CP: f32 = 0.9617967f32;
```

### `CP_H`
```rust
const CP_H: f32 = 0.961914062f32;
```

### `CP_L`
```rust
const CP_L: f32 = -1.17368574E-4f32;
```

### `IVLN2`
```rust
const IVLN2: f32 = 1.44269502f32;
```

### `IVLN2_H`
```rust
const IVLN2_H: f32 = 1.44268799f32;
```

### `IVLN2_L`
```rust
const IVLN2_L: f32 = 7.05260754E-6f32;
```

