**libm > math > expm1f**

# Module: math::expm1f

## Contents

**Functions**

- [`expm1f`](#expm1f) - Exponential, base *e*, of x-1 (f32)

---

## libm::math::expm1f::expm1f

*Function*

Exponential, base *e*, of x-1 (f32)

Calculates the exponential of `x` and subtract 1, that is, *e* raised
to the power `x` minus 1 (where *e* is the base of the natural
system of logarithms, approximately 2.71828).
The result is accurate even for small values of `x`,
where using `exp(x)-1` would lose many significant digits.

```rust
fn expm1f(x: f32) -> f32
```



