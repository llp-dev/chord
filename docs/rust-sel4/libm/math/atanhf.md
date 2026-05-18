**libm > math > atanhf**

# Module: math::atanhf

## Contents

**Functions**

- [`atanhf`](#atanhf) - Inverse hyperbolic tangent (f32)

---

## libm::math::atanhf::atanhf

*Function*

Inverse hyperbolic tangent (f32)

Calculates the inverse hyperbolic tangent of `x`.
Is defined as `log((1+x)/(1-x))/2 = log1p(2x/(1-x))/2`.

```rust
fn atanhf(x: f32) -> f32
```



