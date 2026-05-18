*[ring](../../index.md) / [rsa](../index.md) / [public_modulus](index.md)*

---

# Module `public_modulus`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`PublicModulus`](#publicmodulus) | struct | The modulus (n) of an RSA public key. |

## Structs

### `PublicModulus`

```rust
struct PublicModulus {
    value: self::modulus::OwnedModulus<crate::rsa::N>,
    oneRR: bigint::One<crate::rsa::N, crate::arithmetic::montgomery::RR>,
}
```

The modulus (n) of an RSA public key.

#### Implementations

- <span id="publicmodulus-from-be-bytes"></span>`fn from_be_bytes(n: untrusted::Input<'_>, allowed_bit_lengths: RangeInclusive<bits::BitLength>, cpu_features: cpu::Features) -> Result<Self, error::KeyRejected>` — [`BitLength`](../../bits/index.md#bitlength), [`Features`](../../cpu/index.md#features), [`KeyRejected`](../../error/index.md#keyrejected)

- <span id="publicmodulus-be-bytes"></span>`fn be_bytes(&self) -> impl ExactSizeIterator<Item = u8> + Clone + '_`

  The big-endian encoding of the modulus.

  

  There are no leading zeros.

- <span id="publicmodulus-len-bits"></span>`fn len_bits(&self) -> bits::BitLength` — [`BitLength`](../../bits/index.md#bitlength)

  The length of the modulus in bits.

- <span id="publicmodulus-value"></span>`fn value(&self, cpu_features: cpu::Features) -> self::modulus::Modulus<'_, N>` — [`Features`](../../cpu/index.md#features), [`Modulus`](../../arithmetic/bigint/modulus/index.md#modulus), [`N`](../index.md#n)

- <span id="publicmodulus-onerr"></span>`fn oneRR(&self) -> &bigint::Elem<N, RR>` — [`Elem`](../../arithmetic/bigint/index.md#elem), [`N`](../index.md#n), [`RR`](../../arithmetic/montgomery/index.md#rr)

#### Trait Implementations

##### `impl Clone for PublicModulus`

- <span id="publicmodulus-clone"></span>`fn clone(&self) -> PublicModulus` — [`PublicModulus`](#publicmodulus)

