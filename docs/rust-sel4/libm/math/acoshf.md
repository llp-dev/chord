**libm > math > acoshf**

# Module: math::acoshf

## Contents

**Functions**

- [`acoshf`](#acoshf) - Inverse hyperbolic cosine (f32)

---

## libm::math::acoshf::acoshf

*Function*

Inverse hyperbolic cosine (f32)

Calculates the inverse hyperbolic cosine of `x`.
Is defined as `log(x + sqrt(x*x-1))`.
`x` must be a number greater than or equal to 1.

```rust
fn acoshf(x: f32) -> f32
```



