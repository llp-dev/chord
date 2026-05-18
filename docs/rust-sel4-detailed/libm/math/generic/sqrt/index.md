*[libm](../../../index.md) / [math](../../index.md) / [generic](../index.md) / [sqrt](index.md)*

---

# Module `sqrt`

Generic square root algorithm.

This routine operates around `m_u2`, a U.2 (fixed point with two integral bits) mantissa
within the range [1, 4). A table lookup provides an initial estimate, then goldschmidt
iterations at various widths are used to approach the real values.

For the iterations, `r` is a U0 number that approaches `1/sqrt(m_u2)`, and `s` is a U2 number
that approaches `sqrt(m_u2)`. Recall that m_u2 ∈ [1, 4).

With Newton-Raphson iterations, this would be:

- `w = r * r           w ~ 1 / m`
- `u = 3 - m * w       u ~ 3 - m * w = 3 - m / m = 2`
- `r = r * u / 2       r ~ r`

(Note that the righthand column does not show anything analytically meaningful (i.e. r ~ r),
since the value of performing one iteration is in reducing the error representable by `~`).

Instead of Newton-Raphson iterations, Goldschmidt iterations are used to calculate
`s = m * r`:

- `s = m * r           s ~ m / sqrt(m)`
- `u = 3 - s * r       u ~ 3 - (m / sqrt(m)) * (1 / sqrt(m)) = 3 - m / m = 2`
- `r = r * u / 2       r ~ r`
- `s = s * u / 2       s ~ s`

The above is precise because it uses the original value `m`. There is also a faster version
that performs fewer steps but does not use `m`:

- `u = 3 - s * r       u ~ 3 - 1`
- `r = r * u / 2       r ~ r`
- `s = s * u / 2       s ~ s`

Rounding errors accumulate faster with the second version, so it is only used for subsequent
iterations within the same width integer. The first version is always used for the first
iteration at a new width in order to avoid this accumulation.

Goldschmidt has the advantage over Newton-Raphson that `sqrt(x)` and `1/sqrt(x)` are
computed at the same time, i.e. there is no need to calculate `1/sqrt(x)` and invert it.

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Exp`](#exp) | enum | Representation of whether we shift the exponent into a `u32`, or modify it in place to save the shift operations. |
| [`SqrtHelper`](#sqrthelper) | trait | Size-specific constants related to the square root routine. |
| [`sqrt`](#sqrt) | fn |  |
| [`sqrt_round`](#sqrt-round) | fn |  |
| [`wmulh`](#wmulh) | fn | Multiply at the wider integer size, returning the high half. |
| [`goldschmidt`](#goldschmidt) | fn | Perform `count` goldschmidt iterations, returning `(r_u0, s_u?)`. |

## Enums

### `Exp<T>`

```rust
enum Exp<T> {
    Shifted(u32),
    NoShift(T),
}
```

Representation of whether we shift the exponent into a `u32`, or modify it in place to save
the shift operations.

#### Variants

- **`Shifted`**

  The exponent has been shifted to a `u32` and is LSB-aligned.

- **`NoShift`**

  The exponent is in its natural position in integer repr.

## Traits

### `SqrtHelper`

```rust
trait SqrtHelper: Float { ... }
```

Size-specific constants related to the square root routine.

#### Associated Types

- `type ISet1: 4`

- `type ISet2: 3`

#### Associated Constants

- `const SET1_ROUNDS: u32`

- `const SET2_ROUNDS: u32`

- `const FINAL_ROUNDS: u32`

#### Implementors

- `f32`
- `f64`

## Functions

### `sqrt`

```rust
fn sqrt<F>(x: F) -> F
where
    F: Float + SqrtHelper,
    <F as >::Int: HInt + From<u8> + From<<F as >::ISet2> + CastInto<<F as >::ISet1> + CastInto<<F as >::ISet2>,
    u32: CastInto<<F as >::Int>
```

### `sqrt_round`

```rust
fn sqrt_round<F>(x: F, _round: crate::support::Round) -> crate::support::FpResult<F>
where
    F: Float + SqrtHelper,
    <F as >::Int: HInt + From<u8> + From<<F as >::ISet2> + CastInto<<F as >::ISet1> + CastInto<<F as >::ISet2>,
    u32: CastInto<<F as >::Int>
```

### `wmulh`

```rust
fn wmulh<I: HInt>(a: I, b: I) -> I
```

Multiply at the wider integer size, returning the high half.

### `goldschmidt`

```rust
fn goldschmidt<F, I>(r_u0: I, s_u2: I, count: u32, final_set: bool) -> (I, I)
where
    F: SqrtHelper,
    I: HInt + From<u8>
```

Perform `count` goldschmidt iterations, returning `(r_u0, s_u?)`.

- `r_u0` is the reciprocal `r ~ 1 / sqrt(m)`, as U0.
- `s_u2` is the square root, `s ~ sqrt(m)`, as U2.
- `count` is the number of iterations to perform.
- `final_set` should be true if this is the last round (same-sized integer). If so, the
  returned `s` will be U3, for later shifting. Otherwise, the returned `s` is U2.

Note that performance relies on the optimizer being able to unroll these loops (reasonably
trivial, `count` is a constant when called).

