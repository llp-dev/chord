*[libm](../../../index.md) / [math](../../index.md) / [support](../index.md) / [modular](index.md)*

---

# Module `modular`

This module provides accelerated modular multiplication by large powers
of two, which is needed for computing floating point remainders in `fmod`
and similar functions.

To keep the equations somewhat concise, the following conventions are used:
 - all integer operations are in the mathematical sense, without overflow
 - concatenation means multiplication: `2xq = 2 * x * q`
 - `R = (1 << U::BITS)` is the modulus of wrapping arithmetic in `U`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Reducer`](#reducer) | struct | Helper type for computing the reductions. |
| [`linear_mul_reduction`](#linear-mul-reduction) | fn | Compute the remainder `(x << e) % y` with unbounded integers. |

## Structs

### `Reducer<U: HInt>`

```rust
struct Reducer<U: HInt> {
    m: U,
    r: U,
    _2xq: <U as >::D,
}
```

Helper type for computing the reductions. The implementation has a number
of seemingly weird choices, but everything is aimed at streamlining
`Reducer::word_reduce` into its current form.

Implicitly contains:
 n in (R/8, R/4)
 x in [0, 2n)
The value of `n` is fixed for a given `Reducer`,
but the value of `x` is modified by the methods.

#### Implementations

- <span id="reducer-new"></span>`fn new(x: U, n: U) -> Self`

  Construct a reducer for `(x << _) mod n`.

  

  Requires `R/8 < n < R/4` and `x < 2n`.

- <span id="reducer-partial-remainder"></span>`fn partial_remainder(&self) -> U`

  Extract the current remainder `x` in the range `[0, 2n)`

- <span id="reducer-shift-reduce"></span>`fn shift_reduce(&mut self, k: u32) -> U`

  Replace the remainder `x` with `(x << k) - un`,

  for a suitable quotient `u`, which is returned.

  

  Requires that `k < U::BITS`.

- <span id="reducer-word-reduce"></span>`fn word_reduce(&mut self) -> U`

  Replace the remainder `x` with `x(R/2) - un`,

  for a suitable quotient `u`, which is returned.

#### Trait Implementations

##### `impl<U: clone::Clone + HInt> Clone for Reducer<U>`

- <span id="reducer-clone"></span>`fn clone(&self) -> Reducer<U>` — [`Reducer`](#reducer)

##### `impl<U: fmt::Debug + HInt> Debug for Reducer<U>`

- <span id="reducer-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

##### `impl<U: cmp::Eq + HInt> Eq for Reducer<U>`

##### `impl<U: cmp::PartialEq + HInt> PartialEq for Reducer<U>`

- <span id="reducer-partialeq-eq"></span>`fn eq(&self, other: &Reducer<U>) -> bool` — [`Reducer`](#reducer)

##### `impl<U: HInt> StructuralPartialEq for Reducer<U>`

## Functions

### `linear_mul_reduction`

```rust
fn linear_mul_reduction<U>(x: U, e: u32, y: U) -> U
where
    U: HInt + Int<Unsigned = U>,
    <U as >::D: NarrowingDiv
```

Compute the remainder `(x << e) % y` with unbounded integers.
Requires `x < 2y` and `y.leading_zeros() >= 2`

