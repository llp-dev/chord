**libm > math > asinhf**

# Module: math::asinhf

## Contents

**Functions**

- [`asinhf`](#asinhf) - Inverse hyperbolic sine (f32)

---

## libm::math::asinhf::asinhf

*Function*

Inverse hyperbolic sine (f32)

Calculates the inverse hyperbolic sine of `x`.
Is defined as `sgn(x)*log(|x|+sqrt(x*x+1))`.

```rust
fn asinhf(x: f32) -> f32
```



