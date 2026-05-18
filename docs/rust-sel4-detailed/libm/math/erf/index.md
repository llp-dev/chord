*[libm](../../index.md) / [math](../index.md) / [erf](index.md)*

---

# Module `erf`

## Contents

- [Functions](#functions)
  - [`erfc1`](#erfc1)
  - [`erfc2`](#erfc2)
  - [`erf`](#erf)
  - [`erfc`](#erfc)
- [Constants](#constants)
  - [`ERX`](#erx)
  - [`EFX8`](#efx8)
  - [`PP0`](#pp0)
  - [`PP1`](#pp1)
  - [`PP2`](#pp2)
  - [`PP3`](#pp3)
  - [`PP4`](#pp4)
  - [`QQ1`](#qq1)
  - [`QQ2`](#qq2)
  - [`QQ3`](#qq3)
  - [`QQ4`](#qq4)
  - [`QQ5`](#qq5)
  - [`PA0`](#pa0)
  - [`PA1`](#pa1)
  - [`PA2`](#pa2)
  - [`PA3`](#pa3)
  - [`PA4`](#pa4)
  - [`PA5`](#pa5)
  - [`PA6`](#pa6)
  - [`QA1`](#qa1)
  - [`QA2`](#qa2)
  - [`QA3`](#qa3)
  - [`QA4`](#qa4)
  - [`QA5`](#qa5)
  - [`QA6`](#qa6)
  - [`RA0`](#ra0)
  - [`RA1`](#ra1)
  - [`RA2`](#ra2)
  - [`RA3`](#ra3)
  - [`RA4`](#ra4)
  - [`RA5`](#ra5)
  - [`RA6`](#ra6)
  - [`RA7`](#ra7)
  - [`SA1`](#sa1)
  - [`SA2`](#sa2)
  - [`SA3`](#sa3)
  - [`SA4`](#sa4)
  - [`SA5`](#sa5)
  - [`SA6`](#sa6)
  - [`SA7`](#sa7)
  - [`SA8`](#sa8)
  - [`RB0`](#rb0)
  - [`RB1`](#rb1)
  - [`RB2`](#rb2)
  - [`RB3`](#rb3)
  - [`RB4`](#rb4)
  - [`RB5`](#rb5)
  - [`RB6`](#rb6)
  - [`SB1`](#sb1)
  - [`SB2`](#sb2)
  - [`SB3`](#sb3)
  - [`SB4`](#sb4)
  - [`SB5`](#sb5)
  - [`SB6`](#sb6)
  - [`SB7`](#sb7)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`erfc1`](#erfc1) | fn |  |
| [`erfc2`](#erfc2) | fn |  |
| [`erf`](#erf) | fn | Error function (f64) |
| [`erfc`](#erfc) | fn | Complementary error function (f64) |
| [`ERX`](#erx) | const |  |
| [`EFX8`](#efx8) | const |  |
| [`PP0`](#pp0) | const |  |
| [`PP1`](#pp1) | const |  |
| [`PP2`](#pp2) | const |  |
| [`PP3`](#pp3) | const |  |
| [`PP4`](#pp4) | const |  |
| [`QQ1`](#qq1) | const |  |
| [`QQ2`](#qq2) | const |  |
| [`QQ3`](#qq3) | const |  |
| [`QQ4`](#qq4) | const |  |
| [`QQ5`](#qq5) | const |  |
| [`PA0`](#pa0) | const |  |
| [`PA1`](#pa1) | const |  |
| [`PA2`](#pa2) | const |  |
| [`PA3`](#pa3) | const |  |
| [`PA4`](#pa4) | const |  |
| [`PA5`](#pa5) | const |  |
| [`PA6`](#pa6) | const |  |
| [`QA1`](#qa1) | const |  |
| [`QA2`](#qa2) | const |  |
| [`QA3`](#qa3) | const |  |
| [`QA4`](#qa4) | const |  |
| [`QA5`](#qa5) | const |  |
| [`QA6`](#qa6) | const |  |
| [`RA0`](#ra0) | const |  |
| [`RA1`](#ra1) | const |  |
| [`RA2`](#ra2) | const |  |
| [`RA3`](#ra3) | const |  |
| [`RA4`](#ra4) | const |  |
| [`RA5`](#ra5) | const |  |
| [`RA6`](#ra6) | const |  |
| [`RA7`](#ra7) | const |  |
| [`SA1`](#sa1) | const |  |
| [`SA2`](#sa2) | const |  |
| [`SA3`](#sa3) | const |  |
| [`SA4`](#sa4) | const |  |
| [`SA5`](#sa5) | const |  |
| [`SA6`](#sa6) | const |  |
| [`SA7`](#sa7) | const |  |
| [`SA8`](#sa8) | const |  |
| [`RB0`](#rb0) | const |  |
| [`RB1`](#rb1) | const |  |
| [`RB2`](#rb2) | const |  |
| [`RB3`](#rb3) | const |  |
| [`RB4`](#rb4) | const |  |
| [`RB5`](#rb5) | const |  |
| [`RB6`](#rb6) | const |  |
| [`SB1`](#sb1) | const |  |
| [`SB2`](#sb2) | const |  |
| [`SB3`](#sb3) | const |  |
| [`SB4`](#sb4) | const |  |
| [`SB5`](#sb5) | const |  |
| [`SB6`](#sb6) | const |  |
| [`SB7`](#sb7) | const |  |

## Functions

### `erfc1`

```rust
fn erfc1(x: f64) -> f64
```

### `erfc2`

```rust
fn erfc2(ix: u32, x: f64) -> f64
```

### `erf`

```rust
fn erf(x: f64) -> f64
```

Error function (f64)

Calculates an approximation to the “error function”, which estimates
the probability that an observation will fall within x standard
deviations of the mean (assuming a normal distribution).

### `erfc`

```rust
fn erfc(x: f64) -> f64
```

Complementary error function (f64)

Calculates the complementary probability.
Is `1 - erf(x)`. Is computed directly, so that you can use it to avoid
the loss of precision that would result from subtracting
large probabilities (on large `x`) from 1.

## Constants

### `ERX`
```rust
const ERX: f64 = 0.84506291151046753f64;
```

### `EFX8`
```rust
const EFX8: f64 = 1.0270333367641007f64;
```

### `PP0`
```rust
const PP0: f64 = 0.12837916709551256f64;
```

### `PP1`
```rust
const PP1: f64 = -0.3250421072470015f64;
```

### `PP2`
```rust
const PP2: f64 = -0.02848174957559851f64;
```

### `PP3`
```rust
const PP3: f64 = -0.0057702702964894416f64;
```

### `PP4`
```rust
const PP4: f64 = -2.3763016656650163E-5f64;
```

### `QQ1`
```rust
const QQ1: f64 = 0.39791722395915535f64;
```

### `QQ2`
```rust
const QQ2: f64 = 0.065022249988767294f64;
```

### `QQ3`
```rust
const QQ3: f64 = 0.0050813062818757656f64;
```

### `QQ4`
```rust
const QQ4: f64 = 1.3249473800432164E-4f64;
```

### `QQ5`
```rust
const QQ5: f64 = -3.9602282787753681E-6f64;
```

### `PA0`
```rust
const PA0: f64 = -0.0023621185607526594f64;
```

### `PA1`
```rust
const PA1: f64 = 0.41485611868374833f64;
```

### `PA2`
```rust
const PA2: f64 = -0.37220787603570132f64;
```

### `PA3`
```rust
const PA3: f64 = 0.31834661990116175f64;
```

### `PA4`
```rust
const PA4: f64 = -0.11089469428239668f64;
```

### `PA5`
```rust
const PA5: f64 = 0.035478304325618236f64;
```

### `PA6`
```rust
const PA6: f64 = -0.0021663755948687908f64;
```

### `QA1`
```rust
const QA1: f64 = 0.10642088040084423f64;
```

### `QA2`
```rust
const QA2: f64 = 0.54039791770217105f64;
```

### `QA3`
```rust
const QA3: f64 = 0.071828654414196266f64;
```

### `QA4`
```rust
const QA4: f64 = 0.12617121980876164f64;
```

### `QA5`
```rust
const QA5: f64 = 0.013637083912029051f64;
```

### `QA6`
```rust
const QA6: f64 = 0.011984499846799107f64;
```

### `RA0`
```rust
const RA0: f64 = -0.0098649440348471482f64;
```

### `RA1`
```rust
const RA1: f64 = -0.69385857270718176f64;
```

### `RA2`
```rust
const RA2: f64 = -10.558626225323291f64;
```

### `RA3`
```rust
const RA3: f64 = -62.375332450326006f64;
```

### `RA4`
```rust
const RA4: f64 = -162.39666946257347f64;
```

### `RA5`
```rust
const RA5: f64 = -184.60509290671104f64;
```

### `RA6`
```rust
const RA6: f64 = -81.287435506306593f64;
```

### `RA7`
```rust
const RA7: f64 = -9.8143293441691454f64;
```

### `SA1`
```rust
const SA1: f64 = 19.651271667439257f64;
```

### `SA2`
```rust
const SA2: f64 = 137.65775414351904f64;
```

### `SA3`
```rust
const SA3: f64 = 434.56587747522923f64;
```

### `SA4`
```rust
const SA4: f64 = 645.38727173326788f64;
```

### `SA5`
```rust
const SA5: f64 = 429.00814002756783f64;
```

### `SA6`
```rust
const SA6: f64 = 108.63500554177944f64;
```

### `SA7`
```rust
const SA7: f64 = 6.5702497703192817f64;
```

### `SA8`
```rust
const SA8: f64 = -0.060424415214858099f64;
```

### `RB0`
```rust
const RB0: f64 = -0.0098649429247000992f64;
```

### `RB1`
```rust
const RB1: f64 = -0.79928323768052301f64;
```

### `RB2`
```rust
const RB2: f64 = -17.757954917754752f64;
```

### `RB3`
```rust
const RB3: f64 = -160.63638485582192f64;
```

### `RB4`
```rust
const RB4: f64 = -637.56644336838963f64;
```

### `RB5`
```rust
const RB5: f64 = -1025.0951316110772f64;
```

### `RB6`
```rust
const RB6: f64 = -483.5191916086514f64;
```

### `SB1`
```rust
const SB1: f64 = 30.338060743482458f64;
```

### `SB2`
```rust
const SB2: f64 = 325.79251299657392f64;
```

### `SB3`
```rust
const SB3: f64 = 1536.729586084437f64;
```

### `SB4`
```rust
const SB4: f64 = 3199.8582195085955f64;
```

### `SB5`
```rust
const SB5: f64 = 2553.0504064331644f64;
```

### `SB6`
```rust
const SB6: f64 = 474.52854120695537f64;
```

### `SB7`
```rust
const SB7: f64 = -22.440952446585818f64;
```

