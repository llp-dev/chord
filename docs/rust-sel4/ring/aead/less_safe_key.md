**ring > aead > less_safe_key**

# Module: aead::less_safe_key

## Contents

**Structs**

- [`LessSafeKey`](#lesssafekey) - Immutable keys for use in situations where `OpeningKey`/`SealingKey` and

---

## ring::aead::less_safe_key::LessSafeKey

*Struct*

Immutable keys for use in situations where `OpeningKey`/`SealingKey` and
`NonceSequence` cannot reasonably be used.

Prefer to use `OpeningKey`/`SealingKey` and `NonceSequence` when practical.

**Methods:**

- `fn new(key: UnboundKey) -> Self` - Constructs a `LessSafeKey`.
- `fn open_in_place_separate_tag<'in_out, A>(self: &Self, nonce: Nonce, aad: Aad<A>, tag: Tag, in_out: &'in_out  mut [u8], ciphertext: RangeFrom<usize>) -> Result<&'in_out  mut [u8], error::Unspecified>` - Like [open_in_place](Self::open_in_place), except the authentication tag is
- `fn open_in_place<'in_out, A>(self: &Self, nonce: Nonce, aad: Aad<A>, in_out: &'in_out  mut [u8]) -> Result<&'in_out  mut [u8], error::Unspecified>` - Like [`super::OpeningKey::open_in_place()`], except it accepts an
- `fn open_within<'in_out, A>(self: &Self, nonce: Nonce, aad: Aad<A>, in_out: &'in_out  mut [u8], ciphertext_and_tag: RangeFrom<usize>) -> Result<&'in_out  mut [u8], error::Unspecified>` - Like [`super::OpeningKey::open_within()`], except it accepts an
- `fn seal_in_place_append_tag<A, InOut>(self: &Self, nonce: Nonce, aad: Aad<A>, in_out: & mut InOut) -> Result<(), error::Unspecified>` - Like [`super::SealingKey::seal_in_place_append_tag()`], except it
- `fn seal_in_place_separate_tag<A>(self: &Self, nonce: Nonce, aad: Aad<A>, in_out: & mut [u8]) -> Result<Tag, error::Unspecified>` - Like `super::SealingKey::seal_in_place_separate_tag()`, except it
- `fn algorithm(self: &Self) -> &'static Algorithm` - The key's AEAD algorithm.

**Trait Implementations:**

- **Clone**
  - `fn clone(self: &Self) -> LessSafeKey`
- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> Result<(), core::fmt::Error>`



