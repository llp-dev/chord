**libm > math > acosh**

# Module: math::acosh

## Contents

**Functions**

- [`acosh`](#acosh) - Inverse hyperbolic cosine (f64)

---

## libm::math::acosh::acosh

*Function*

Inverse hyperbolic cosine (f64)

Calculates the inverse hyperbolic cosine of `x`.
Is defined as `log(x + sqrt(x*x-1))`.
`x` must be a number greater than or equal to 1.

```rust
fn acosh(x: f64) -> f64
```



