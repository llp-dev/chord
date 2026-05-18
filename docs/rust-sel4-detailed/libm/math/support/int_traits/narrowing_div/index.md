*[libm](../../../../index.md) / [math](../../../index.md) / [support](../../index.md) / [int_traits](../index.md) / [narrowing_div](index.md)*

---

# Module `narrowing_div`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`NarrowingDiv`](#narrowingdiv) | trait | Trait for unsigned division of a double-wide integer when the quotient doesn't overflow. |
| [`div_three_digits_by_two`](#div-three-digits-by-two) | fn | Implement `u3N / u2N`-division on top of `u2N / uN`-division. |
| [`impl_narrowing_div_primitive!`](#impl-narrowing-div-primitive) | macro |  |
| [`impl_narrowing_div_recurse!`](#impl-narrowing-div-recurse) | macro |  |

## Traits

### `NarrowingDiv`

```rust
trait NarrowingDiv: DInt + MinInt<Unsigned = Self> { ... }
```

Trait for unsigned division of a double-wide integer
when the quotient doesn't overflow.

This is the inverse of widening multiplication:
 - for any `x` and nonzero `y`: `x.widen_mul(y).checked_narrowing_div_rem(y) == Some((x, 0))`,
 - and for any `r in 0..y`: `x.carrying_mul(y, r).checked_narrowing_div_rem(y) == Some((x, r))`,

#### Required Methods

- `fn unchecked_narrowing_div_rem(self, n: <Self as >::H) -> (<Self as >::H, <Self as >::H)`

  Computes `(self / n, self % n))`

#### Provided Methods

- `fn checked_narrowing_div_rem(self, n: <Self as >::H) -> Option<(<Self as >::H, <Self as >::H)>`

  Returns `Some((self / n, self % n))` when `self.hi() < n`.

#### Implementors

- [`u256`](../../big/index.md#u256)
- `u128`
- `u16`
- `u32`
- `u64`

## Functions

### `div_three_digits_by_two`

```rust
unsafe fn div_three_digits_by_two<U>(a0: U, a: <U as >::D, n: <U as >::D) -> (U, <U as >::D)
where
    U: HInt,
    <U as >::D: Int + NarrowingDiv
```

Implement `u3N / u2N`-division on top of `u2N / uN`-division.

Returns the quotient and remainder of `(a * R + a0) / n`,
where `R = (1 << U::BITS)` is the digit size.

# Safety
Requires that `n.leading_zeros() == 0` and `a < n`.

## Macros

### `impl_narrowing_div_primitive!`

### `impl_narrowing_div_recurse!`

