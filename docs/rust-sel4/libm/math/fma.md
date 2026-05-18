**libm > math > fma**

# Module: math::fma

## Contents

**Functions**

- [`fma`](#fma) - Fused multiply add (f64)
- [`fmaf`](#fmaf) - Floating multiply add (f32)

---

## libm::math::fma::fma

*Function*

Fused multiply add (f64)

Computes `(x*y)+z`, rounded as one ternary operation (i.e. calculated with infinite precision).

```rust
fn fma(x: f64, y: f64, z: f64) -> f64
```



## libm::math::fma::fmaf

*Function*

Floating multiply add (f32)

Computes `(x*y)+z`, rounded as one ternary operation (i.e. calculated with infinite precision).

```rust
fn fmaf(x: f32, y: f32, z: f32) -> f32
```



