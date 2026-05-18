*[libm](../../../index.md) / [math](../../index.md) / [generic](../index.md) / [fma](index.md)*

---

# Module `fma`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`Norm`](#norm) | struct | Representation of `F` that has handled subnormals. |
| [`fma_round`](#fma-round) | fn | Fused multiply-add that works when there is not a larger float size available. |

## Structs

### `Norm<F: Float>`

```rust
struct Norm<F: Float> {
    m: <F as >::Int,
    e: i32,
    neg: bool,
}
```

Representation of `F` that has handled subnormals.

#### Fields

- **`m`**: `<F as >::Int`

  Normalized significand with one guard bit, unsigned.

- **`e`**: `i32`

  Exponent of the mantissa such that `m * 2^e = x`. Accounts for the shift in the mantissa
  and the guard bit; that is, 1.0 will normalize as `m = 1 << 53` and `e = -53`.

#### Implementations

- <span id="norm-const-exp-unbias"></span>`const EXP_UNBIAS: u32`

- <span id="norm-const-zero-inf-nan"></span>`const ZERO_INF_NAN: u32`

- <span id="norm-from-float"></span>`fn from_float(x: F) -> Self`

- <span id="norm-is-zero-nan-inf"></span>`fn is_zero_nan_inf(self) -> bool`

  True if the value was zero, infinity, or NaN.

- <span id="norm-is-zero"></span>`fn is_zero(self) -> bool`

  The only value we have

#### Trait Implementations

##### `impl<F: clone::Clone + Float> Clone for Norm<F>`

- <span id="norm-clone"></span>`fn clone(&self) -> Norm<F>` — [`Norm`](#norm)

##### `impl<F: marker::Copy + Float> Copy for Norm<F>`

##### `impl<F: fmt::Debug + Float> Debug for Norm<F>`

- <span id="norm-debug-fmt"></span>`fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result`

## Functions

### `fma_round`

```rust
fn fma_round<F>(x: F, y: F, z: F, _round: crate::support::Round) -> crate::support::FpResult<F>
where
    F: Float + CastFrom<<F as >::SignedInt> + CastFrom<i8>,
    <F as >::Int: HInt,
    u32: CastInto<<F as >::Int>
```

Fused multiply-add that works when there is not a larger float size available. Computes
`(x * y) + z`.

