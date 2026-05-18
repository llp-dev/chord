*[libm](../../index.md) / [math](../index.md) / [j0f](index.md)*

---

# Module `j0f`

## Contents

- [Functions](#functions)
  - [`common`](#common)
  - [`j0f`](#j0f)
  - [`y0f`](#y0f)
  - [`pzerof`](#pzerof)
  - [`qzerof`](#qzerof)
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
| [`j0f`](#j0f) | fn | Zeroth order of the [Bessel function](https://en.wikipedia.org/wiki/Bessel_function) of the first kind (f32). |
| [`y0f`](#y0f) | fn | Zeroth order of the [Bessel function](https://en.wikipedia.org/wiki/Bessel_function) of the second kind (f32). |
| [`pzerof`](#pzerof) | fn |  |
| [`qzerof`](#qzerof) | fn |  |
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
fn common(ix: u32, x: f32, y0: bool) -> f32
```

### `j0f`

```rust
fn j0f(x: f32) -> f32
```

Zeroth order of the [Bessel function](https://en.wikipedia.org/wiki/Bessel_function) of the first kind (f32).

### `y0f`

```rust
fn y0f(x: f32) -> f32
```

Zeroth order of the [Bessel function](https://en.wikipedia.org/wiki/Bessel_function) of the second kind (f32).

### `pzerof`

```rust
fn pzerof(x: f32) -> f32
```

### `qzerof`

```rust
fn qzerof(x: f32) -> f32
```

## Constants

### `INVSQRTPI`
```rust
const INVSQRTPI: f32 = 0.564189613f32;
```

### `TPI`
```rust
const TPI: f32 = 0.636619746f32;
```

### `R02`
```rust
const R02: f32 = 0.015625f32;
```

### `R03`
```rust
const R03: f32 = -1.89979299E-4f32;
```

### `R04`
```rust
const R04: f32 = 1.82954045E-6f32;
```

### `R05`
```rust
const R05: f32 = -4.61832705E-9f32;
```

### `S01`
```rust
const S01: f32 = 0.0156191029f32;
```

### `S02`
```rust
const S02: f32 = 1.16926785E-4f32;
```

### `S03`
```rust
const S03: f32 = 5.13546524E-7f32;
```

### `S04`
```rust
const S04: f32 = 1.16614007E-9f32;
```

### `U00`
```rust
const U00: f32 = -0.0738042966f32;
```

### `U01`
```rust
const U01: f32 = 0.176666453f32;
```

### `U02`
```rust
const U02: f32 = -0.0138185676f32;
```

### `U03`
```rust
const U03: f32 = 3.47453431E-4f32;
```

### `U04`
```rust
const U04: f32 = -3.81407062E-6f32;
```

### `U05`
```rust
const U05: f32 = 1.9559014E-8f32;
```

### `U06`
```rust
const U06: f32 = -3.98205184E-11f32;
```

### `V01`
```rust
const V01: f32 = 0.0127304839f32;
```

### `V02`
```rust
const V02: f32 = 7.60068651E-5f32;
```

### `V03`
```rust
const V03: f32 = 2.59150852E-7f32;
```

### `V04`
```rust
const V04: f32 = 4.41110315E-10f32;
```

### `PR8`
```rust
const PR8: [f32; 6];
```

### `PS8`
```rust
const PS8: [f32; 5];
```

### `PR5`
```rust
const PR5: [f32; 6];
```

### `PS5`
```rust
const PS5: [f32; 5];
```

### `PR3`
```rust
const PR3: [f32; 6];
```

### `PS3`
```rust
const PS3: [f32; 5];
```

### `PR2`
```rust
const PR2: [f32; 6];
```

### `PS2`
```rust
const PS2: [f32; 5];
```

### `QR8`
```rust
const QR8: [f32; 6];
```

### `QS8`
```rust
const QS8: [f32; 6];
```

### `QR5`
```rust
const QR5: [f32; 6];
```

### `QS5`
```rust
const QS5: [f32; 6];
```

### `QR3`
```rust
const QR3: [f32; 6];
```

### `QS3`
```rust
const QS3: [f32; 6];
```

### `QR2`
```rust
const QR2: [f32; 6];
```

### `QS2`
```rust
const QS2: [f32; 6];
```

