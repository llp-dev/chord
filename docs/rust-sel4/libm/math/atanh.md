**libm > math > atanh**

# Module: math::atanh

## Contents

**Functions**

- [`atanh`](#atanh) - Inverse hyperbolic tangent (f64)

---

## libm::math::atanh::atanh

*Function*

Inverse hyperbolic tangent (f64)

Calculates the inverse hyperbolic tangent of `x`.
Is defined as `log((1+x)/(1-x))/2 = log1p(2x/(1-x))/2`.

```rust
fn atanh(x: f64) -> f64
```



