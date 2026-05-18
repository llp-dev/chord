**libm > math > asinh**

# Module: math::asinh

## Contents

**Functions**

- [`asinh`](#asinh) - Inverse hyperbolic sine (f64)

---

## libm::math::asinh::asinh

*Function*

Inverse hyperbolic sine (f64)

Calculates the inverse hyperbolic sine of `x`.
Is defined as `sgn(x)*log(|x|+sqrt(x*x+1))`.

```rust
fn asinh(x: f64) -> f64
```



