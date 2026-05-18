**libm > math > atan2**

# Module: math::atan2

## Contents

**Functions**

- [`atan2`](#atan2) - Arctangent of y/x (f64)

---

## libm::math::atan2::atan2

*Function*

Arctangent of y/x (f64)

Computes the inverse tangent (arc tangent) of `y/x`.
Produces the correct result even for angles near pi/2 or -pi/2 (that is, when `x` is near 0).
Returns a value in radians, in the range of -pi to pi.

```rust
fn atan2(y: f64, x: f64) -> f64
```



