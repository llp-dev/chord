*[ring](../../index.md) / [rsa](../index.md) / [public_exponent](index.md)*

---

# Module `public_exponent`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`PublicExponent`](#publicexponent) | struct | The exponent `e` of an RSA public key. |

## Structs

### `PublicExponent`

```rust
struct PublicExponent(core::num::NonZeroU64);
```

The exponent `e` of an RSA public key.

#### Implementations

- <span id="publicexponent-const-3"></span>`const _3: Self`

- <span id="publicexponent-const-65537"></span>`const _65537: Self`

- <span id="publicexponent-const-max"></span>`const MAX: Self`

- <span id="publicexponent-from-be-bytes"></span>`fn from_be_bytes(input: untrusted::Input<'_>, min_value: Self) -> Result<Self, error::KeyRejected>` — [`KeyRejected`](../../error/index.md#keyrejected)

- <span id="publicexponent-be-bytes"></span>`fn be_bytes(&self) -> impl ExactSizeIterator<Item = u8> + Clone + '_`

  The big-endian encoding of the exponent.

  

  There are no leading zeros.

- <span id="publicexponent-value"></span>`fn value(self) -> NonZeroU64`

#### Trait Implementations

##### `impl Clone for PublicExponent`

- <span id="publicexponent-clone"></span>`fn clone(&self) -> PublicExponent` — [`PublicExponent`](#publicexponent)

##### `impl Copy for PublicExponent`

