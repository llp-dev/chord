**libm > math > erff**

# Module: math::erff

## Contents

**Functions**

- [`erfcf`](#erfcf) - Complementary error function (f32)
- [`erff`](#erff) - Error function (f32)

---

## libm::math::erff::erfcf

*Function*

Complementary error function (f32)

Calculates the complementary probability.
Is `1 - erf(x)`. Is computed directly, so that you can use it to avoid
the loss of precision that would result from subtracting
large probabilities (on large `x`) from 1.

```rust
fn erfcf(x: f32) -> f32
```



## libm::math::erff::erff

*Function*

Error function (f32)

Calculates an approximation to the “error function”, which estimates
the probability that an observation will fall within x standard
deviations of the mean (assuming a normal distribution).

```rust
fn erff(x: f32) -> f32
```



