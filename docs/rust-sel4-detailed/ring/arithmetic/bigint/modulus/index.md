*[ring](../../../index.md) / [arithmetic](../../index.md) / [bigint](../index.md) / [modulus](index.md)*

---

# Module `modulus`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`OwnedModulus`](#ownedmodulus) | struct | The modulus *m* for a ring ℤ/mℤ, along with the precomputed values needed for efficient Montgomery multiplication modulo *m*. |
| [`Modulus`](#modulus) | struct |  |
| [`MODULUS_MIN_LIMBS`](#modulus-min-limbs) | const | The x86 implementation of `bn_mul_mont`, at least, requires at least 4 limbs. |
| [`MODULUS_MAX_LIMBS`](#modulus-max-limbs) | const |  |

## Structs

### `OwnedModulus<M>`

```rust
struct OwnedModulus<M> {
    limbs: self::boxed_limbs::BoxedLimbs<M>,
    n0: super::N0,
    len_bits: crate::bits::BitLength,
}
```

The modulus *m* for a ring ℤ/mℤ, along with the precomputed values needed
for efficient Montgomery multiplication modulo *m*. The value must be odd
and larger than 2. The larger-than-1 requirement is imposed, at least, by
the modular inversion code.

#### Implementations

- <span id="ownedmodulus-from-be-bytes"></span>`fn from_be_bytes(input: untrusted::Input<'_>) -> Result<Self, error::KeyRejected>` — [`KeyRejected`](../../../error/index.md#keyrejected)

- <span id="ownedmodulus-verify-less-than"></span>`fn verify_less_than<L>(&self, l: &Modulus<'_, L>) -> Result<(), error::Unspecified>` — [`Modulus`](#modulus), [`Unspecified`](../../../error/index.md#unspecified)

- <span id="ownedmodulus-to-elem"></span>`fn to_elem<L>(&self, l: &Modulus<'_, L>) -> Result<Elem<L, Unencoded>, error::Unspecified>` — [`Modulus`](#modulus), [`Elem`](../index.md#elem), [`Unencoded`](../../montgomery/index.md#unencoded), [`Unspecified`](../../../error/index.md#unspecified)

- <span id="ownedmodulus-modulus"></span>`fn modulus(&self, cpu_features: cpu::Features) -> Modulus<'_, M>` — [`Features`](../../../cpu/index.md#features), [`Modulus`](#modulus)

- <span id="ownedmodulus-len-bits"></span>`fn len_bits(&self) -> BitLength` — [`BitLength`](../../../bits/index.md#bitlength)

#### Trait Implementations

##### `impl<M: PublicModulus> Clone for OwnedModulus<M>`

- <span id="ownedmodulus-clone"></span>`fn clone(&self) -> Self`

### `Modulus<'a, M>`

```rust
struct Modulus<'a, M> {
    limbs: &'a [u64],
    n0: super::N0,
    len_bits: crate::bits::BitLength,
    m: core::marker::PhantomData<M>,
    cpu_features: cpu::Features,
}
```

#### Implementations

- <span id="modulus-oner"></span>`fn oneR(&self, out: &mut [u64])`

- <span id="modulus-zero"></span>`fn zero<E>(&self) -> Elem<M, E>` — [`Elem`](../index.md#elem)

- <span id="modulus-limbs"></span>`fn limbs(&self) -> &[u64]`

- <span id="modulus-n0"></span>`fn n0(&self) -> &N0` — [`N0`](../../n0/index.md#n0)

- <span id="modulus-len-bits"></span>`fn len_bits(&self) -> BitLength` — [`BitLength`](../../../bits/index.md#bitlength)

- <span id="modulus-cpu-features"></span>`fn cpu_features(&self) -> cpu::Features` — [`Features`](../../../cpu/index.md#features)

## Constants

### `MODULUS_MIN_LIMBS`
```rust
const MODULUS_MIN_LIMBS: usize = 4usize;
```

The x86 implementation of `bn_mul_mont`, at least, requires at least 4
limbs. For a long time we have required 4 limbs for all targets, though
this may be unnecessary. TODO: Replace this with
`n.len() < 256 / LIMB_BITS` so that 32-bit and 64-bit platforms behave the
same.

### `MODULUS_MAX_LIMBS`
```rust
const MODULUS_MAX_LIMBS: usize = 128usize;
```

