*[ring](../../../index.md) / [arithmetic](../../index.md) / [bigint](../index.md) / [boxed_limbs](index.md)*

---

# Module `boxed_limbs`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`BoxedLimbs`](#boxedlimbs) | struct | All `BoxedLimbs<M>` are stored in the same number of limbs. |

## Structs

### `BoxedLimbs<M>`

```rust
struct BoxedLimbs<M> {
    limbs: alloc::boxed::Box<[u64]>,
    m: core::marker::PhantomData<M>,
}
```

All `BoxedLimbs<M>` are stored in the same number of limbs.

#### Fields

- **`m`**: `core::marker::PhantomData<M>`

  The modulus *m* that determines the size of `limbx`.

#### Implementations

- <span id="boxedlimbs-new-unchecked"></span>`fn new_unchecked(limbs: Box<[u64]>) -> Self`

- <span id="boxedlimbs-positive-minimal-width-from-be-bytes"></span>`fn positive_minimal_width_from_be_bytes(input: untrusted::Input<'_>) -> Result<Self, error::KeyRejected>` — [`KeyRejected`](../../../error/index.md#keyrejected)

- <span id="boxedlimbs-from-be-bytes-padded-less-than"></span>`fn from_be_bytes_padded_less_than(input: untrusted::Input<'_>, m: &Modulus<'_, M>) -> Result<Self, error::Unspecified>` — [`Modulus`](../modulus/index.md#modulus), [`Unspecified`](../../../error/index.md#unspecified)

- <span id="boxedlimbs-is-zero"></span>`fn is_zero(&self) -> bool`

- <span id="boxedlimbs-zero"></span>`fn zero(len: usize) -> Self`

- <span id="boxedlimbs-into-limbs"></span>`fn into_limbs(self) -> Box<[u64]>`

#### Trait Implementations

##### `impl<M> Clone for BoxedLimbs<M>`

- <span id="boxedlimbs-clone"></span>`fn clone(&self) -> Self`

##### `impl<M> Deref for BoxedLimbs<M>`

- <span id="boxedlimbs-deref-type-target"></span>`type Target = [u64]`

- <span id="boxedlimbs-deref"></span>`fn deref(&self) -> &<Self as >::Target`

##### `impl<M> DerefMut for BoxedLimbs<M>`

- <span id="boxedlimbs-derefmut-deref-mut"></span>`fn deref_mut(&mut self) -> &mut <Self as >::Target`

##### `impl Receiver for BoxedLimbs<M>`

- <span id="boxedlimbs-receiver-type-target"></span>`type Target = T`

