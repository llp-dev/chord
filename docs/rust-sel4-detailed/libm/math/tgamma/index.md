*[libm](../../index.md) / [math](../index.md) / [tgamma](index.md)*

---

# Module `tgamma`

## Contents

- [Functions](#functions)
  - [`sinpi`](#sinpi)
  - [`s`](#s)
  - [`tgamma`](#tgamma)
- [Constants](#constants)
  - [`PI`](#pi)
  - [`N`](#n)
  - [`GMHALF`](#gmhalf)
  - [`SNUM`](#snum)
  - [`SDEN`](#sden)
  - [`FACT`](#fact)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`sinpi`](#sinpi) | fn |  |
| [`s`](#s) | fn |  |
| [`tgamma`](#tgamma) | fn | The [Gamma function](https://en.wikipedia.org/wiki/Gamma_function) (f64). |
| [`PI`](#pi) | const |  |
| [`N`](#n) | const |  |
| [`GMHALF`](#gmhalf) | const |  |
| [`SNUM`](#snum) | const |  |
| [`SDEN`](#sden) | const |  |
| [`FACT`](#fact) | const |  |

## Functions

### `sinpi`

```rust
fn sinpi(x: f64) -> f64
```

### `s`

```rust
fn s(x: f64) -> f64
```

### `tgamma`

```rust
fn tgamma(x: f64) -> f64
```

The [Gamma function](https://en.wikipedia.org/wiki/Gamma_function) (f64).

## Constants

### `PI`
```rust
const PI: f64 = 3.1415926535897931f64;
```

### `N`
```rust
const N: usize = 12usize;
```

### `GMHALF`
```rust
const GMHALF: f64 = 5.5246800407767296f64;
```

### `SNUM`
```rust
const SNUM: [f64; 13];
```

### `SDEN`
```rust
const SDEN: [f64; 13];
```

### `FACT`
```rust
const FACT: [f64; 23];
```

