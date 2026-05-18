*[libm](../../index.md) / [math](../index.md) / [j1f](index.md)*

---

# Module `j1f`

## Contents

- [Functions](#functions)
  - [`common`](#common)
  - [`j1f`](#j1f)
  - [`y1f`](#y1f)
  - [`ponef`](#ponef)
  - [`qonef`](#qonef)
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
| [`j1f`](#j1f) | fn | First order of the [Bessel function](https://en.wikipedia.org/wiki/Bessel_function) of the first kind (f32). |
| [`y1f`](#y1f) | fn | First order of the [Bessel function](https://en.wikipedia.org/wiki/Bessel_function) of the second kind (f32). |
| [`ponef`](#ponef) | fn |  |
| [`qonef`](#qonef) | fn |  |
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
fn common(ix: u32, x: f32, y1: bool, sign: bool) -> f32
```

### `j1f`

```rust
fn j1f(x: f32) -> f32
```

First order of the [Bessel function](https://en.wikipedia.org/wiki/Bessel_function) of the first kind (f32).

### `y1f`

```rust
fn y1f(x: f32) -> f32
```

First order of the [Bessel function](https://en.wikipedia.org/wiki/Bessel_function) of the second kind (f32).

### `ponef`

```rust
fn ponef(x: f32) -> f32
```

### `qonef`

```rust
fn qonef(x: f32) -> f32
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

### `R00`
```rust
const R00: f32 = -0.0625f32;
```

### `R01`
```rust
const R01: f32 = 0.0014070567f32;
```

### `R02`
```rust
const R02: f32 = -1.59955634E-5f32;
```

### `R03`
```rust
const R03: f32 = 4.96727992E-8f32;
```

### `S01`
```rust
const S01: f32 = 0.0191537607f32;
```

### `S02`
```rust
const S02: f32 = 1.85946788E-4f32;
```

### `S03`
```rust
const S03: f32 = 1.17718469E-6f32;
```

### `S04`
```rust
const S04: f32 = 5.04636244E-9f32;
```

### `S05`
```rust
const S05: f32 = 1.2354227E-11f32;
```

### `U0`
```rust
const U0: [f32; 5];
```

### `V0`
```rust
const V0: [f32; 5];
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

