**libm > math > atan2f**

# Module: math::atan2f

## Contents

**Functions**

- [`atan2f`](#atan2f) - Arctangent of y/x (f32)

---

## libm::math::atan2f::atan2f

*Function*

Arctangent of y/x (f32)

Computes the inverse tangent (arc tangent) of `y/x`.
Produces the correct result even for angles near pi/2 or -pi/2 (that is, when `x` is near 0).
Returns a value in radians, in the range of -pi to pi.

```rust
fn atan2f(y: f32, x: f32) -> f32
```



