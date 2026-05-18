**libm > math > expm1**

# Module: math::expm1

## Contents

**Functions**

- [`expm1`](#expm1) - Exponential, base *e*, of x-1 (f64)

---

## libm::math::expm1::expm1

*Function*

Exponential, base *e*, of x-1 (f64)

Calculates the exponential of `x` and subtract 1, that is, *e* raised
to the power `x` minus 1 (where *e* is the base of the natural
system of logarithms, approximately 2.71828).
The result is accurate even for small values of `x`,
where using `exp(x)-1` would lose many significant digits.

```rust
fn expm1(x: f64) -> f64
```



