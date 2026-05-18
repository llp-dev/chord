**libm > math > fdim**

# Module: math::fdim

## Contents

**Functions**

- [`fdim`](#fdim) - Positive difference (f64)
- [`fdimf`](#fdimf) - Positive difference (f32)

---

## libm::math::fdim::fdim

*Function*

Positive difference (f64)

Determines the positive difference between arguments, returning:
* x - y if x > y, or
* +0    if x <= y, or
* NAN   if either argument is NAN.

A range error may occur.

```rust
fn fdim(x: f64, y: f64) -> f64
```



## libm::math::fdim::fdimf

*Function*

Positive difference (f32)

Determines the positive difference between arguments, returning:
* x - y if x > y, or
* +0    if x <= y, or
* NAN   if either argument is NAN.

A range error may occur.

```rust
fn fdimf(x: f32, y: f32) -> f32
```



