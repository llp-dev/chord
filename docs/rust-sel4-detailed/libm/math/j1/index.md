*[libm](../../index.md) / [math](../index.md) / [j1](index.md)*

---

# Module `j1`

## Contents

- [Functions](#functions)
  - [`common`](#common)
  - [`j1`](#j1)
  - [`y1`](#y1)
  - [`pone`](#pone)
  - [`qone`](#qone)
- [Constants](#constants)
  - [`INVSQRTPI`](#invsqrtpi)
  - [`TPI`](#tpi)
  - [`R00`](#r00)
  - [`R01`](#r01)
  - [`R02`](#r02)
  - [`R03`](#r03)
  - [`S01`](#s01)
  - [`S02`](#s02)
  - [`S03`](#s03)
  - [`S04`](#s04)
  - [`S05`](#s05)
  - [`U0`](#u0)
  - [`V0`](#v0)
  - [`PR8`](#pr8)
  - [`PS8`](#ps8)
  - [`PR5`](#pr5)
  - [`PS5`](#ps5)
  - [`PR3`](#pr3)
  - [`PS3`](#ps3)
  - [`PR2`](#pr2)
  - [`PS2`](#ps2)
  - [`QR8`](#qr8)
  - [`QS8`](#qs8)
  - [`QR5`](#qr5)
  - [`QS5`](#qs5)
  - [`QR3`](#qr3)
  - [`QS3`](#qs3)
  - [`QR2`](#qr2)
  - [`QS2`](#qs2)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`common`](#common) | fn |  |
| [`j1`](#j1) | fn | First order of the [Bessel function](https://en.wikipedia.org/wiki/Bessel_function) of the first kind (f64). |
| [`y1`](#y1) | fn | First order of the [Bessel function](https://en.wikipedia.org/wiki/Bessel_function) of the second kind (f64). |
| [`pone`](#pone) | fn |  |
| [`qone`](#qone) | fn |  |
| [`INVSQRTPI`](#invsqrtpi) | const |  |
| [`TPI`](#tpi) | const |  |
| [`R00`](#r00) | const |  |
| [`R01`](#r01) | const |  |
| [`R02`](#r02) | const |  |
| [`R03`](#r03) | const |  |
| [`S01`](#s01) | const |  |
| [`S02`](#s02) | const |  |
| [`S03`](#s03) | const |  |
| [`S04`](#s04) | const |  |
| [`S05`](#s05) | const |  |
| [`U0`](#u0) | const |  |
| [`V0`](#v0) | const |  |
| [`PR8`](#pr8) | const |  |
| [`PS8`](#ps8) | const |  |
| [`PR5`](#pr5) | const |  |
| [`PS5`](#ps5) | const |  |
| [`PR3`](#pr3) | const |  |
| [`PS3`](#ps3) | const |  |
| [`PR2`](#pr2) | const |  |
| [`PS2`](#ps2) | const |  |
| [`QR8`](#qr8) | const |  |
| [`QS8`](#qs8) | const |  |
| [`QR5`](#qr5) | const |  |
| [`QS5`](#qs5) | const |  |
| [`QR3`](#qr3) | const |  |
| [`QS3`](#qs3) | const |  |
| [`QR2`](#qr2) | const |  |
| [`QS2`](#qs2) | const |  |

## Functions

### `common`

```rust
fn common(ix: u32, x: f64, y1: bool, sign: bool) -> f64
```

### `j1`

```rust
fn j1(x: f64) -> f64
```

First order of the [Bessel function](https://en.wikipedia.org/wiki/Bessel_function) of the first kind (f64).

### `y1`

```rust
fn y1(x: f64) -> f64
```

First order of the [Bessel function](https://en.wikipedia.org/wiki/Bessel_function) of the second kind (f64).

### `pone`

```rust
fn pone(x: f64) -> f64
```

### `qone`

```rust
fn qone(x: f64) -> f64
```

## Constants

### `INVSQRTPI`
```rust
const INVSQRTPI: f64 = 0.56418958354775628f64;
```

### `TPI`
```rust
const TPI: f64 = 0.63661977236758138f64;
```

### `R00`
```rust
const R00: f64 = -0.0625f64;
```

### `R01`
```rust
const R01: f64 = 0.0014070566695518971f64;
```

### `R02`
```rust
const R02: f64 = -1.599556310840356E-5f64;
```

### `R03`
```rust
const R03: f64 = 4.9672799960958445E-8f64;
```

### `S01`
```rust
const S01: f64 = 0.019153759953836346f64;
```

### `S02`
```rust
const S02: f64 = 1.8594678558863092E-4f64;
```

### `S03`
```rust
const S03: f64 = 1.1771846404262368E-6f64;
```

### `S04`
```rust
const S04: f64 = 5.0463625707621704E-9f64;
```

### `S05`
```rust
const S05: f64 = 1.2354227442613791E-11f64;
```

### `U0`
```rust
const U0: [f64; 5];
```

### `V0`
```rust
const V0: [f64; 5];
```

### `PR8`
```rust
const PR8: [f64; 6];
```

### `PS8`
```rust
const PS8: [f64; 5];
```

### `PR5`
```rust
const PR5: [f64; 6];
```

### `PS5`
```rust
const PS5: [f64; 5];
```

### `PR3`
```rust
const PR3: [f64; 6];
```

### `PS3`
```rust
const PS3: [f64; 5];
```

### `PR2`
```rust
const PR2: [f64; 6];
```

### `PS2`
```rust
const PS2: [f64; 5];
```

### `QR8`
```rust
const QR8: [f64; 6];
```

### `QS8`
```rust
const QS8: [f64; 6];
```

### `QR5`
```rust
const QR5: [f64; 6];
```

### `QS5`
```rust
const QS5: [f64; 6];
```

### `QR3`
```rust
const QR3: [f64; 6];
```

### `QS3`
```rust
const QS3: [f64; 6];
```

### `QR2`
```rust
const QR2: [f64; 6];
```

### `QS2`
```rust
const QS2: [f64; 6];
```

