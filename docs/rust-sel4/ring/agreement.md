**ring > agreement**

# Module: agreement

## Contents

**Structs**

- [`Algorithm`](#algorithm) - A key agreement algorithm.
- [`EphemeralPrivateKey`](#ephemeralprivatekey) - An ephemeral private key for use (only) with `agree_ephemeral`. The
- [`PublicKey`](#publickey) - A public key for key agreement.
- [`UnparsedPublicKey`](#unparsedpublickey) - An unparsed, possibly malformed, public key for key agreement.

**Functions**

- [`agree_ephemeral`](#agree_ephemeral) - Performs a key agreement with an ephemeral private key and the given public

---

## ring::agreement::Algorithm

*Struct*

A key agreement algorithm.

**Traits:** Eq

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> Result<(), ::core::fmt::Error>`
- **PartialEq**
  - `fn eq(self: &Self, other: &Self) -> bool`



## ring::agreement::EphemeralPrivateKey

*Struct*

An ephemeral private key for use (only) with `agree_ephemeral`. The
signature of `agree_ephemeral` ensures that an `EphemeralPrivateKey` can be
used for at most one key agreement.

**Methods:**

- `fn generate(alg: &'static Algorithm, rng: &dyn rand::SecureRandom) -> Result<Self, error::Unspecified>` - Generate a new ephemeral private key for the given algorithm.
- `fn compute_public_key(self: &Self) -> Result<PublicKey, error::Unspecified>` - Computes the public key from the private key.
- `fn algorithm(self: &Self) -> &'static Algorithm` - The algorithm for the private key.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut ::core::fmt::Formatter) -> Result<(), ::core::fmt::Error>`



## ring::agreement::PublicKey

*Struct*

A public key for key agreement.

**Methods:**

- `fn algorithm(self: &Self) -> &'static Algorithm` - The algorithm for the public key.

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> Result<(), core::fmt::Error>`
- **Clone**
  - `fn clone(self: &Self) -> PublicKey`
- **AsRef**
  - `fn as_ref(self: &Self) -> &[u8]`



## ring::agreement::UnparsedPublicKey

*Struct*

An unparsed, possibly malformed, public key for key agreement.

**Generic Parameters:**
- B

**Methods:**

- `fn new(algorithm: &'static Algorithm, bytes: B) -> Self` - Constructs a new `UnparsedPublicKey`.
- `fn algorithm(self: &Self) -> &'static Algorithm` - The algorithm for the public key.
- `fn bytes(self: &Self) -> &B` - TODO: doc

**Traits:** Copy

**Trait Implementations:**

- **Debug**
  - `fn fmt(self: &Self, f: & mut core::fmt::Formatter) -> Result<(), core::fmt::Error>`
- **Clone**
  - `fn clone(self: &Self) -> UnparsedPublicKey<B>`
- **AsRef**
  - `fn as_ref(self: &Self) -> &[u8]`



## ring::agreement::agree_ephemeral

*Function*

Performs a key agreement with an ephemeral private key and the given public
key.

`my_private_key` is the ephemeral private key to use. Since it is moved, it
will not be usable after calling `agree_ephemeral`, thus guaranteeing that
the key is used for only one key agreement.

`peer_public_key` is the peer's public key. `agree_ephemeral` will return
`Err(error_value)` if it does not match `my_private_key's` algorithm/curve.
`agree_ephemeral` verifies that it is encoded in the standard form for the
algorithm and that the key is *valid*; see the algorithm's documentation for
details on how keys are to be encoded and what constitutes a valid key for
that algorithm.

After the key agreement is done, `agree_ephemeral` calls `kdf` with the raw
key material from the key agreement operation and then returns what `kdf`
returns.

```rust
fn agree_ephemeral<B, R, impl FnOnce(&[u8]) -> R>(my_private_key: EphemeralPrivateKey, peer_public_key: &UnparsedPublicKey<B>, kdf: impl Trait) -> Result<R, error::Unspecified>
```



