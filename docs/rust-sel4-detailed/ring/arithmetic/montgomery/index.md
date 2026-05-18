*[ring](../../index.md) / [arithmetic](../index.md) / [montgomery](index.md)*

---

# Module `montgomery`

## Contents

- [Structs](#structs)
  - [`N0`](#n0)
- [Enums](#enums)
  - [`Unencoded`](#unencoded)
  - [`R`](#r)
  - [`RRR`](#rrr)
  - [`RR`](#rr)
  - [`RInverse`](#rinverse)
- [Traits](#traits)
  - [`Encoding`](#encoding)
  - [`ReductionEncoding`](#reductionencoding)
  - [`ProductEncoding`](#productencoding)
- [Functions](#functions)
  - [`bn_mul_mont`](#bn-mul-mont)
  - [`mul_mont`](#mul-mont)
  - [`limbs_from_mont_in_place`](#limbs-from-mont-in-place)
  - [`limbs_mont_mul`](#limbs-mont-mul)
  - [`limbs_mont_square`](#limbs-mont-square)

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`N0`](#n0) | struct |  |
| [`Unencoded`](#unencoded) | enum |  |
| [`R`](#r) | enum |  |
| [`RRR`](#rrr) | enum |  |
| [`RR`](#rr) | enum |  |
| [`RInverse`](#rinverse) | enum |  |
| [`Encoding`](#encoding) | trait |  |
| [`ReductionEncoding`](#reductionencoding) | trait | The encoding of the result of a reduction. |
| [`ProductEncoding`](#productencoding) | trait | The encoding of the result of a multiplication. |
| [`bn_mul_mont`](#bn-mul-mont) | fn |  |
| [`mul_mont`](#mul-mont) | fn |  |
| [`limbs_from_mont_in_place`](#limbs-from-mont-in-place) | fn |  |
| [`limbs_mont_mul`](#limbs-mont-mul) | fn | r *= a |
| [`limbs_mont_square`](#limbs-mont-square) | fn | r = r**2 |

## Structs

### `N0`

```rust
struct N0([u64; 2]);
```

#### Implementations

- <span id="n0-const-limbs-used"></span>`const LIMBS_USED: usize`

- <span id="n0-precalculated"></span>`const fn precalculated(n0: u64) -> Self`

#### Trait Implementations

##### `impl Clone for N0`

- <span id="n0-clone"></span>`fn clone(&self) -> N0` — [`N0`](../n0/index.md#n0)

##### `impl Copy for N0`

## Enums

### `Unencoded`

```rust
enum Unencoded {
}
```

#### Trait Implementations

##### `impl Clone for Unencoded`

- <span id="unencoded-clone"></span>`fn clone(&self) -> Unencoded` — [`Unencoded`](#unencoded)

##### `impl Copy for Unencoded`

##### `impl Encoding for Unencoded`

##### `impl ReductionEncoding for Unencoded`

- <span id="unencoded-reductionencoding-type-output"></span>`type Output = RInverse`

### `R`

```rust
enum R {
}
```

#### Trait Implementations

##### `impl Clone for R`

- <span id="r-clone"></span>`fn clone(&self) -> R` — [`R`](#r)

##### `impl Copy for R`

##### `impl Encoding for R`

##### `impl ReductionEncoding for R`

- <span id="r-reductionencoding-type-output"></span>`type Output = Unencoded`

### `RRR`

```rust
enum RRR {
}
```

#### Trait Implementations

##### `impl Clone for RRR`

- <span id="rrr-clone"></span>`fn clone(&self) -> RRR` — [`RRR`](#rrr)

##### `impl Copy for RRR`

##### `impl Encoding for RRR`

##### `impl ReductionEncoding for RRR`

- <span id="rrr-reductionencoding-type-output"></span>`type Output = RR`

### `RR`

```rust
enum RR {
}
```

#### Trait Implementations

##### `impl Clone for RR`

- <span id="rr-clone"></span>`fn clone(&self) -> RR` — [`RR`](#rr)

##### `impl Copy for RR`

##### `impl Encoding for RR`

##### `impl ReductionEncoding for RR`

- <span id="rr-reductionencoding-type-output"></span>`type Output = R`

### `RInverse`

```rust
enum RInverse {
}
```

#### Trait Implementations

##### `impl Clone for RInverse`

- <span id="rinverse-clone"></span>`fn clone(&self) -> RInverse` — [`RInverse`](#rinverse)

##### `impl Copy for RInverse`

##### `impl Encoding for RInverse`

## Traits

### `Encoding`

```rust
trait Encoding { ... }
```

#### Implementors

- [`RInverse`](#rinverse)
- [`RRR`](#rrr)
- [`RR`](#rr)
- [`R`](#r)
- [`Unencoded`](#unencoded)

### `ReductionEncoding`

```rust
trait ReductionEncoding { ... }
```

The encoding of the result of a reduction.

#### Associated Types

- `type Output: 1`

#### Implementors

- [`RRR`](#rrr)
- [`RR`](#rr)
- [`R`](#r)
- [`Unencoded`](#unencoded)

### `ProductEncoding`

```rust
trait ProductEncoding { ... }
```

The encoding of the result of a multiplication.

#### Associated Types

- `type Output: 1`

#### Implementors

- `(R, E)`
- `(RInverse, E)`
- `(RR, RInverse)`
- `(RR, RR)`
- `(RR, Unencoded)`
- `(RRR, RInverse)`
- `(Unencoded, E)`

## Functions

### `bn_mul_mont`

```rust
unsafe fn bn_mul_mont(r: *mut u64, a: *const u64, b: *const u64, n: *const u64, n0: &N0, num_limbs: usize)
```

### `mul_mont`

```rust
unsafe fn mul_mont(r: *mut u64, a: *const u64, b: *const u64, n: *const u64, n0: &N0, num_limbs: usize, _: cpu::Features)
```

### `limbs_from_mont_in_place`

```rust
fn limbs_from_mont_in_place(r: &mut [u64], tmp: &mut [u64], m: &[u64], n0: &N0)
```

### `limbs_mont_mul`

```rust
fn limbs_mont_mul(r: &mut [u64], a: &[u64], m: &[u64], n0: &N0, cpu_features: cpu::Features)
```

r *= a

### `limbs_mont_square`

```rust
fn limbs_mont_square(r: &mut [u64], m: &[u64], n0: &N0, cpu_features: cpu::Features)
```

r = r**2

