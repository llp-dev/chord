*[libm](../../index.md) / [math](../index.md) / [expf](index.md)*

---

# Module `expf`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`expf`](#expf) | fn | Exponential, base *e* (f32) |
| [`HALF`](#half) | const |  |
| [`LN2_HI`](#ln2-hi) | const |  |
| [`LN2_LO`](#ln2-lo) | const |  |
| [`INV_LN2`](#inv-ln2) | const |  |
| [`P1`](#p1) | const |  |
| [`P2`](#p2) | const |  |

## Functions

### `expf`

```rust
fn expf(x: f32) -> f32
```

Exponential, base *e* (f32)

Calculate the exponential of `x`, that is, *e* raised to the power `x`
(where *e* is the base of the natural system of logarithms, approximately 2.71828).

## Constants

### `HALF`
```rust
const HALF: [f32; 2];
```

### `LN2_HI`
```rust
const LN2_HI: f32 = 0.693145751f32;
```

### `LN2_LO`
```rust
const LN2_LO: f32 = 1.42860677E-6f32;
```

### `INV_LN2`
```rust
const INV_LN2: f32 = 1.44269502f32;
```

### `P1`
```rust
const P1: f32 = 0.166666254f32;
```

### `P2`
```rust
const P2: f32 = -0.00276673329f32;
```

