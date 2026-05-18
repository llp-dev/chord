*[ring](../../index.md) / [rsa](../index.md) / [public_key](index.md)*

---

# Module `public_key`

## Quick Reference

| Item | Kind | Description |
|------|------|-------------|
| [`PublicKey`](#publickey) | struct | An RSA Public Key. |
| [`Inner`](#inner) | struct | `PublicKey` but without any superfluous allocations, optimized for one-shot RSA signature verification. |
| [`fill_be_bytes_n`](#fill-be-bytes-n) | fn | Returns the big-endian representation of `elem` that is the same length as the minimal-length big-endian representation of the modulus `n`. |

## Structs

### `PublicKey`

```rust
struct PublicKey {
    inner: Inner,
    serialized: alloc::boxed::Box<[u8]>,
}
```

An RSA Public Key.

#### Implementations

- <span id="publickey-from-modulus-and-exponent"></span>`fn from_modulus_and_exponent(n: untrusted::Input<'_>, e: untrusted::Input<'_>, n_min_bits: bits::BitLength, n_max_bits: bits::BitLength, e_min_value: PublicExponent, cpu_features: cpu::Features) -> Result<Self, error::KeyRejected>` — [`BitLength`](../../bits/index.md#bitlength), [`PublicExponent`](../public_exponent/index.md#publicexponent), [`Features`](../../cpu/index.md#features), [`KeyRejected`](../../error/index.md#keyrejected)

- <span id="publickey-modulus-len"></span>`fn modulus_len(&self) -> usize`

  The length, in bytes, of the public modulus.

  

  The modulus length is rounded up to a whole number of bytes if its

  bit length isn't a multiple of 8.

- <span id="publickey-inner"></span>`fn inner(&self) -> &Inner` — [`Inner`](#inner)

#### Trait Implementations

##### `impl AsRef for PublicKey`

- <span id="publickey-asref-as-ref"></span>`fn as_ref(&self) -> &[u8]`

##### `impl Clone for PublicKey`

- <span id="publickey-clone"></span>`fn clone(&self) -> PublicKey` — [`PublicKey`](#publickey)

##### `impl Debug for PublicKey`

- <span id="publickey-debug-fmt"></span>`fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> Result<(), ::core::fmt::Error>`

### `Inner`

```rust
struct Inner {
    n: self::public_modulus::PublicModulus,
    e: self::public_exponent::PublicExponent,
}
```

`PublicKey` but without any superfluous allocations, optimized for one-shot
RSA signature verification.

#### Implementations

- <span id="inner-from-modulus-and-exponent"></span>`fn from_modulus_and_exponent(n: untrusted::Input<'_>, e: untrusted::Input<'_>, n_min_bits: bits::BitLength, n_max_bits: bits::BitLength, e_min_value: PublicExponent, cpu_features: cpu::Features) -> Result<Self, error::KeyRejected>` — [`BitLength`](../../bits/index.md#bitlength), [`PublicExponent`](../public_exponent/index.md#publicexponent), [`Features`](../../cpu/index.md#features), [`KeyRejected`](../../error/index.md#keyrejected)

- <span id="inner-n"></span>`fn n(&self) -> &PublicModulus` — [`PublicModulus`](../public_modulus/index.md#publicmodulus)

  The public modulus.

- <span id="inner-e"></span>`fn e(&self) -> PublicExponent` — [`PublicExponent`](../public_exponent/index.md#publicexponent)

  The public exponent.

- <span id="inner-exponentiate"></span>`fn exponentiate<'out>(&self, base: untrusted::Input<'_>, out_buffer: &'out mut [u8; 1024], cpu_features: cpu::Features) -> Result<&'out [u8], error::Unspecified>` — [`Features`](../../cpu/index.md#features), [`Unspecified`](../../error/index.md#unspecified)

  Calculates base**e (mod n), filling the first part of `out_buffer` with

  the result.

  

  This is constant-time with respect to the value in `base` (only).

  

  The result will be a slice of the encoded bytes of the result within

  `out_buffer`, if successful.

- <span id="inner-exponentiate-elem"></span>`fn exponentiate_elem(&self, base: &bigint::Elem<N>, cpu_features: cpu::Features) -> bigint::Elem<N>` — [`Elem`](../../arithmetic/bigint/index.md#elem), [`N`](../index.md#n), [`Features`](../../cpu/index.md#features)

  Calculates base**e (mod n).

  

  This is constant-time with respect to `base` only.

#### Trait Implementations

##### `impl Clone for Inner`

- <span id="inner-clone"></span>`fn clone(&self) -> Inner` — [`Inner`](#inner)

## Functions

### `fill_be_bytes_n`

```rust
fn fill_be_bytes_n(elem: bigint::Elem<super::N>, n_bits: bits::BitLength, out: &mut [u8; 1024]) -> &[u8]
```

Returns the big-endian representation of `elem` that is
the same length as the minimal-length big-endian representation of
the modulus `n`.

`n_bits` must be the bit length of the public modulus `n`.

