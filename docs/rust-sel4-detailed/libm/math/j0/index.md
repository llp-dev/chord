*[libm](../../index.md) / [math](../index.md) / [j0](index.md)*

---

# Module `j0`

## Contents

- [Functions](#functions)
  - [`common`](#common)
  - [`j0`](#j0)
  - [`y0`](#y0)
  - [`pzero`](#pzero)
  - [`qzero`](#qzero)
- [Constants](#constants)
  - [`INVSQRTPI`](#invsqrtpi)
  - [`TPI`](#tpi)
  - [`R02`](#r02)
  - [`R03`](#r03)
  - [`R04`](#r04)
  - [`R05`](#r05)
  - [`S01`](#s01)
  - [`S02`](#s02)
  - [`S03`](#s03)
  - [`S04`](#s04)
  - [`U00`](#u00)
  - [`U01`](#u01)
  - [`U02`](#u02)
  - [`U03`](#u03)
  - [`U04`](#u04)
  - [`U05`](#u05)
  - [`U06`](#u06)
  - [`V01`](#v01)
  - [`V02`](#v02)
  - [`V03`](#v03)
  - [`V04`](#v04)
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
| [`j0`](#j0) | fn | Zeroth order of the [Bessel function](https://en.wikipedia.org/wiki/Bessel_function) of the first kind (f64). |
| [`y0`](#y0) | fn | Zeroth order of the [Bessel function](https://en.wikipedia.org/wiki/Bessel_function) of the second kind (f64). |
| [`pzero`](#pzero) | fn |  |
| [`qzero`](#qzero) | fn |  |
| [`INVSQRTPI`](#invsqrtpi) | const |  |
| [`TPI`](#tpi) | const |  |
| [`R02`](#r02) | const |  |
| [`R03`](#r03) | const |  |
| [`R04`](#r04) | const |  |
| [`R05`](#r05) | const |  |
| [`S01`](#s01) | const |  |
| [`S02`](#s02) | const |  |
| [`S03`](#s03) | const |  |
| [`S04`](#s04) | const |  |
| [`U00`](#u00) | const |  |
| [`U01`](#u01) | const |  |
| [`U02`](#u02) | const |  |
| [`U03`](#u03) | const |  |
| [`U04`](#u04) | const |  |
| [`U05`](#u05) | const |  |
| [`U06`](#u06) | const |  |
| [`V01`](#v01) | const |  |
| [`V02`](#v02) | const |  |
| [`V03`](#v03) | const |  |
| [`V04`](#v04) | const |  |
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
fn common(ix: u32, x: f64, y0: bool) -> f64
```

### `j0`

```rust
fn j0(x: f64) -> f64
```

Zeroth order of the [Bessel function](https://en.wikipedia.org/wiki/Bessel_function) of the first kind (f64).

### `y0`

```rust
fn y0(x: f64) -> f64
```

Zeroth order of the [Bessel function](https://en.wikipedia.org/wiki/Bessel_function) of the second kind (f64).

### `pzero`

```rust
fn pzero(x: f64) -> f64
```

### `qzero`

```rust
fn qzero(x: f64) -> f64
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

### `R02`
```rust
const R02: f64 = 0.015624999999999995f64;
```

### `R03`
```rust
const R03: f64 = -1.8997929423885472E-4f64;
```

### `R04`
```rust
const R04: f64 = 1.8295404953270067E-6f64;
```

### `R05`
```rust
const R05: f64 = -4.6183268853210319E-9f64;
```

### `S01`
```rust
const S01: f64 = 0.015619102946489001f64;
```

### `S02`
```rust
const S02: f64 = 1.1692678466333745E-4f64;
```

### `S03`
```rust
const S03: f64 = 5.1354655020731811E-7f64;
```

### `S04`
```rust
const S04: f64 = 1.1661400333379E-9f64;
```

### `U00`
```rust
const U00: f64 = -0.073804295108687232f64;
```

### `U01`
```rust
const U01: f64 = 0.17666645250918112f64;
```

### `U02`
```rust
const U02: f64 = -0.01381856719455969f64;
```

### `U03`
```rust
const U03: f64 = 3.4745343209368365E-4f64;
```

### `U04`
```rust
const U04: f64 = -3.8140705372436416E-6f64;
```

### `U05`
```rust
const U05: f64 = 1.9559013703502292E-8f64;
```

### `U06`
```rust
const U06: f64 = -3.982051941321034E-11f64;
```

### `V01`
```rust
const V01: f64 = 0.01273048348341237f64;
```

### `V02`
```rust
const V02: f64 = 7.6006862735035325E-5f64;
```

### `V03`
```rust
const V03: f64 = 2.5915085184045781E-7f64;
```

### `V04`
```rust
const V04: f64 = 4.4111031133267547E-10f64;
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

