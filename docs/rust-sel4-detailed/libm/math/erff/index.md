*[libm](../../index.md) / [math](../index.md) / [erff](index.md)*

---

# Module `erff`

## Contents

- [Functions](#functions)
  - [`erfc1`](#erfc1)
  - [`erfc2`](#erfc2)
  - [`erff`](#erff)
  - [`erfcf`](#erfcf)
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
| [`erff`](#erff) | fn | Error function (f32) |
| [`erfcf`](#erfcf) | fn | Complementary error function (f32) |
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
fn erfc1(x: f32) -> f32
```

### `erfc2`

```rust
fn erfc2(ix: u32, x: f32) -> f32
```

### `erff`

```rust
fn erff(x: f32) -> f32
```

Error function (f32)

Calculates an approximation to the “error function”, which estimates
the probability that an observation will fall within x standard
deviations of the mean (assuming a normal distribution).

### `erfcf`

```rust
fn erfcf(x: f32) -> f32
```

Complementary error function (f32)

Calculates the complementary probability.
Is `1 - erf(x)`. Is computed directly, so that you can use it to avoid
the loss of precision that would result from subtracting
large probabilities (on large `x`) from 1.

## Constants

### `ERX`
```rust
const ERX: f32 = 0.845062911f32;
```

### `EFX8`
```rust
const EFX8: f32 = 1.02703333f32;
```

### `PP0`
```rust
const PP0: f32 = 0.128379166f32;
```

### `PP1`
```rust
const PP1: f32 = -0.325042099f32;
```

### `PP2`
```rust
const PP2: f32 = -0.0284817498f32;
```

### `PP3`
```rust
const PP3: f32 = -0.00577027025f32;
```

### `PP4`
```rust
const PP4: f32 = -2.37630175E-5f32;
```

### `QQ1`
```rust
const QQ1: f32 = 0.397917211f32;
```

### `QQ2`
```rust
const QQ2: f32 = 0.0650222525f32;
```

### `QQ3`
```rust
const QQ3: f32 = 0.00508130621f32;
```

### `QQ4`
```rust
const QQ4: f32 = 1.32494737E-4f32;
```

### `QQ5`
```rust
const QQ5: f32 = -3.96022824E-6f32;
```

### `PA0`
```rust
const PA0: f32 = -0.00236211857f32;
```

### `PA1`
```rust
const PA1: f32 = 0.414856106f32;
```

### `PA2`
```rust
const PA2: f32 = -0.37220788f32;
```

### `PA3`
```rust
const PA3: f32 = 0.31834662f32;
```

### `PA4`
```rust
const PA4: f32 = -0.110894695f32;
```

### `PA5`
```rust
const PA5: f32 = 0.0354783051f32;
```

### `PA6`
```rust
const PA6: f32 = -0.00216637552f32;
```

### `QA1`
```rust
const QA1: f32 = 0.106420882f32;
```

### `QA2`
```rust
const QA2: f32 = 0.540397942f32;
```

### `QA3`
```rust
const QA3: f32 = 0.0718286559f32;
```

### `QA4`
```rust
const QA4: f32 = 0.126171216f32;
```

### `QA5`
```rust
const QA5: f32 = 0.0136370836f32;
```

### `QA6`
```rust
const QA6: f32 = 0.0119845001f32;
```

### `RA0`
```rust
const RA0: f32 = -0.00986494403f32;
```

### `RA1`
```rust
const RA1: f32 = -0.693858563f32;
```

### `RA2`
```rust
const RA2: f32 = -10.5586262f32;
```

### `RA3`
```rust
const RA3: f32 = -62.3753319f32;
```

### `RA4`
```rust
const RA4: f32 = -162.396667f32;
```

### `RA5`
```rust
const RA5: f32 = -184.605087f32;
```

### `RA6`
```rust
const RA6: f32 = -81.2874374f32;
```

### `RA7`
```rust
const RA7: f32 = -9.81432914f32;
```

### `SA1`
```rust
const SA1: f32 = 19.6512718f32;
```

### `SA2`
```rust
const SA2: f32 = 137.657761f32;
```

### `SA3`
```rust
const SA3: f32 = 434.565887f32;
```

### `SA4`
```rust
const SA4: f32 = 645.387268f32;
```

### `SA5`
```rust
const SA5: f32 = 429.008148f32;
```

### `SA6`
```rust
const SA6: f32 = 108.635002f32;
```

### `SA7`
```rust
const SA7: f32 = 6.57024956f32;
```

### `SA8`
```rust
const SA8: f32 = -0.0604244135f32;
```

### `RB0`
```rust
const RB0: f32 = -0.0098649431f32;
```

### `RB1`
```rust
const RB1: f32 = -0.799283266f32;
```

### `RB2`
```rust
const RB2: f32 = -17.7579556f32;
```

### `RB3`
```rust
const RB3: f32 = -160.636383f32;
```

### `RB4`
```rust
const RB4: f32 = -637.566467f32;
```

### `RB5`
```rust
const RB5: f32 = -1025.09509f32;
```

### `RB6`
```rust
const RB6: f32 = -483.519196f32;
```

### `SB1`
```rust
const SB1: f32 = 30.3380604f32;
```

### `SB2`
```rust
const SB2: f32 = 325.792511f32;
```

### `SB3`
```rust
const SB3: f32 = 1536.72961f32;
```

### `SB4`
```rust
const SB4: f32 = 3199.85815f32;
```

### `SB5`
```rust
const SB5: f32 = 2553.05029f32;
```

### `SB6`
```rust
const SB6: f32 = 474.528534f32;
```

### `SB7`
```rust
const SB7: f32 = -22.4409523f32;
```

