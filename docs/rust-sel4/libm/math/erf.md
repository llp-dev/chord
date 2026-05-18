**libm > math > erf**

# Module: math::erf

## Contents

**Functions**

- [`erf`](#erf) - Error function (f64)
- [`erfc`](#erfc) - Complementary error function (f64)

---

## libm::math::erf::erf

*Function*

Error function (f64)

Calculates an approximation to the “error function”, which estimates
the probability that an observation will fall within x standard
deviations of the mean (assuming a normal distribution).

```rust
fn erf(x: f64) -> f64
```



## libm::math::erf::erfc

*Function*

Complementary error function (f64)

Calculates the complementary probability.
Is `1 - erf(x)`. Is computed directly, so that you can use it to avoid
the loss of precision that would result from subtracting
large probabilities (on large `x`) from 1.

```rust
fn erfc(x: f64) -> f64
```



